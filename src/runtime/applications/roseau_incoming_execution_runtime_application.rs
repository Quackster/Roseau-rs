use crate::dao::{
    CatalogueDao, DaoError, InventoryDao, ItemDao, MessengerDao, NavigatorDao, PlayerDao, RoomDao,
};
use crate::game::catalogue::{
    CatalogueIncomingOutcome, CatalogueIncomingPlan, CatalogueOrderInfoNetworkPlan,
    CataloguePurchaseExecution, CataloguePurchaseNetworkPlan, CataloguePurchaseOutcome,
    CatalogueTicketPurchaseExecution, CatalogueTicketPurchaseNetworkPlan,
};
use crate::game::commands::CommandContext;
use crate::game::inventory::{
    InventoryCommandExecutor, InventoryCommandNetworkPlan, InventoryIncomingPlan,
};
use crate::game::item::interactors::{PoolChangeBoothInteractor, PoolLiftInteractor};
use crate::game::item::{
    Item, ItemCommandExecution, ItemCommandNetworkPlan, ItemIncomingPlan,
    ItemInteractionEffectExecutor, ItemInteractionEffectNetworkPlan,
    ItemInteractionEffectRoomExecutor,
};
use crate::game::messenger::{
    Messenger, MessengerCommandOutcome, MessengerEffectNetworkPlan, MessengerFriendRefreshExecutor,
    MessengerIncomingPlan,
};
use crate::game::moderation::{ModerationIncomingPlan, ModerationRoomContext};
use crate::game::navigator::NavigatorSearchNetworkPlan;
use crate::game::pathfinder::make_path;
use crate::game::player::{
    FindUserNetworkPlan, FindUserOutcome, PasswordIncomingPlan, PlayerCommandNetworkPlan,
    PlayerEffect, PlayerEffectNetworkPlan, PlayerIncomingOutcome, PlayerIncomingPlan,
    PlayerPasswordActionExecutor, PlayerPasswordActionNetworkPlan, PlayerProfileUpdateExecutor,
    PlayerSession,
};
use crate::game::room::entity::RoomUser;
use crate::game::room::entity::RoomUserEffect;
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::settings::RoomType;
use crate::game::room::{
    Room, RoomCommandExecution, RoomData, RoomDecorationIncomingPlan, RoomDecorationNetworkPlan,
    RoomEffect, RoomEffectNetworkPlan, RoomEntryIncomingPlan, RoomEntryNetworkPlan,
    RoomIncomingPlan, RoomLeaveNetworkPlan, RoomLeavePlan, RoomMapping, RoomOccupant,
    RoomPoolNetworkPlan, RoomSummary, RoomUserEffectNetworkPlan, RoomUserIncomingPlan,
    RoomUserRoomEffectExecutor,
};
use crate::game::room::{RoomUnitIncomingPlan, RoomUnitNetworkPlan};
use crate::messages::outgoing::{
    ActiveObjects, BuddyAddRequests, BuddyList, BuddyListFriend, FlatProperty, HeightMap, Items,
    MessengerMessage as MessengerMessagePacket, MessengerReady, MessengerSmsAccount,
    MessengersReady, MyPersistentMessage, ObjectsWorld, RoomReady, Status, StripInfo, Users,
    WalletBalance, YouAreController, YouAreOwner,
};
use crate::messages::OutgoingMessage;
use crate::messages::{
    IncomingCommandExecutor, IncomingExecutionEffect, PendingIncomingCommandBatch,
};
use crate::runtime::{RoseauApplicationRuntime, RoseauIncomingExecutionRuntimePlan};

#[derive(Clone, Copy)]
struct CatalogueIncomingDaoSet<'a> {
    catalogue: &'a dyn CatalogueDao,
    inventory: &'a dyn InventoryDao,
    item: &'a dyn ItemDao,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IncomingRoomContext {
    user_id: i32,
    room_id: i32,
    room_owner_id: i32,
    has_rights: bool,
    has_explicit_rights: bool,
    has_owner_rights: bool,
    all_super_user: bool,
}

impl RoseauApplicationRuntime {
    pub fn collect_incoming_execution_runtime_plans(
        &self,
        effects: &[IncomingExecutionEffect],
        connection_id: i32,
    ) -> Vec<RoseauIncomingExecutionRuntimePlan> {
        RoseauIncomingExecutionRuntimePlan::collect(effects, connection_id)
    }

    pub fn apply_incoming_execution_runtime_plans(
        &mut self,
        effects: &[IncomingExecutionEffect],
        connection_id: i32,
    ) -> Vec<RoseauIncomingExecutionRuntimePlan> {
        let plans = self.collect_incoming_execution_runtime_plans(effects, connection_id);
        let network_effects = plans
            .iter()
            .map(|plan| match plan {
                RoseauIncomingExecutionRuntimePlan::Network(effect) => effect.clone(),
            })
            .collect::<Vec<_>>();

        self.startup_runtime_mut()
            .apply_network_effects(network_effects)
            .into_iter()
            .map(RoseauIncomingExecutionRuntimePlan::Network)
            .collect()
    }

    pub fn drain_pending_incoming_commands(&mut self) -> Vec<PendingIncomingCommandBatch> {
        self.startup_runtime_mut().drain_pending_incoming_commands()
    }

    pub fn apply_pending_incoming_commands(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        let batches = self.drain_pending_incoming_commands();
        self.apply_pending_incoming_command_batches_inner(
            player_dao,
            room_dao,
            navigator_dao,
            messenger_dao,
            None,
            &batches,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_pending_incoming_commands_with_catalogue(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        catalogue_dao: &dyn CatalogueDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        let batches = self.drain_pending_incoming_commands();
        self.apply_pending_incoming_command_batches_with_catalogue(
            player_dao,
            room_dao,
            catalogue_dao,
            inventory_dao,
            item_dao,
            navigator_dao,
            messenger_dao,
            &batches,
        )
    }

    pub fn apply_pending_incoming_command_batches(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
        batches: &[PendingIncomingCommandBatch],
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        self.apply_pending_incoming_command_batches_inner(
            player_dao,
            room_dao,
            navigator_dao,
            messenger_dao,
            None,
            batches,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_pending_incoming_command_batches_with_catalogue(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        catalogue_dao: &dyn CatalogueDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
        batches: &[PendingIncomingCommandBatch],
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        self.apply_pending_incoming_command_batches_inner(
            player_dao,
            room_dao,
            navigator_dao,
            messenger_dao,
            Some(CatalogueIncomingDaoSet {
                catalogue: catalogue_dao,
                inventory: inventory_dao,
                item: item_dao,
            }),
            batches,
        )
    }

    fn apply_pending_incoming_command_batches_inner(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
        catalogue_daos: Option<CatalogueIncomingDaoSet<'_>>,
        batches: &[PendingIncomingCommandBatch],
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        let mut plans = Vec::new();

        for batch in batches {
            plans.extend(self.apply_pending_incoming_command_batch(
                player_dao,
                room_dao,
                navigator_dao,
                messenger_dao,
                catalogue_daos,
                batch,
            )?);
        }

        Ok(plans)
    }

    fn apply_pending_incoming_command_batch(
        &mut self,
        player_dao: &dyn PlayerDao,
        room_dao: &dyn RoomDao,
        navigator_dao: &dyn NavigatorDao,
        messenger_dao: &dyn MessengerDao,
        catalogue_daos: Option<CatalogueIncomingDaoSet<'_>>,
        batch: &PendingIncomingCommandBatch,
    ) -> Result<Vec<RoseauIncomingExecutionRuntimePlan>, DaoError> {
        let command_context = CommandContext::new();
        let effects = IncomingCommandExecutor::plan(
            self.game().command_manager(),
            &command_context,
            batch.commands(),
        );
        let mut network_effects = Vec::new();
        network_effects.extend(self.apply_password_incoming_effects(player_dao, batch, &effects)?);
        network_effects.extend(self.get_credits_network_effects(batch, &effects));
        network_effects.extend(self.find_user_network_effects(player_dao, batch, &effects)?);
        network_effects.extend(self.player_incoming_network_effects(player_dao, batch, &effects)?);
        self.apply_personal_greeting_effects(player_dao, batch, &effects)?;
        network_effects
            .extend(self.update_pool_figure_network_effects(player_dao, batch, &effects)?);
        network_effects.extend(self.room_command_network_effects(
            room_dao,
            catalogue_daos.map(|daos| daos.item),
            batch,
            &effects,
        )?);
        network_effects.extend(self.room_entry_network_effects(room_dao, batch, &effects)?);
        network_effects.extend(self.room_control_network_effects(room_dao, batch, &effects)?);
        network_effects
            .extend(self.room_user_incoming_network_effects(room_dao, None, batch, &effects)?);
        network_effects.extend(self.moderation_incoming_network_effects(batch, &effects));
        if let Some(catalogue_daos) = catalogue_daos {
            network_effects.extend(self.decoration_incoming_network_effects(
                room_dao,
                catalogue_daos.item,
                catalogue_daos.inventory,
                batch,
                &effects,
            )?);
            network_effects.extend(self.room_user_incoming_network_effects(
                room_dao,
                Some(catalogue_daos.item),
                batch,
                &effects,
            )?);
            network_effects.extend(self.close_pool_change_booth_network_effects(
                room_dao,
                catalogue_daos.item,
                batch,
                &effects,
            )?);
            network_effects.extend(self.room_pool_network_effects(
                catalogue_daos.item,
                batch,
                &effects,
            )?);
            network_effects.extend(self.add_wall_item_network_effects(
                room_dao,
                catalogue_daos.inventory,
                catalogue_daos.item,
                batch,
                &effects,
            )?);
            network_effects.extend(self.goto_flat_network_effects(
                room_dao,
                catalogue_daos.item,
                batch,
                &effects,
            )?);
            if has_room_login(&effects) {
                network_effects.extend(self.goto_flat_network_effects(
                    room_dao,
                    catalogue_daos.item,
                    batch,
                    &[IncomingExecutionEffect::GoToFlat],
                )?);
            }
            network_effects.extend(self.item_incoming_network_effects(
                room_dao,
                catalogue_daos.item,
                catalogue_daos.inventory,
                batch,
                &effects,
            )?);
            network_effects.extend(self.inventory_incoming_network_effects(
                catalogue_daos.inventory,
                batch,
                &effects,
            )?);
            network_effects.extend(self.catalogue_incoming_network_effects(
                player_dao,
                catalogue_daos,
                batch,
                &effects,
            )?);
        }
        network_effects.extend(self.room_unit_incoming_network_effects(batch, &effects));
        network_effects.extend(self.navigator_incoming_network_effects(
            room_dao,
            navigator_dao,
            batch,
            &effects,
        )?);
        network_effects.extend(self.messenger_init_network_effects(
            player_dao,
            messenger_dao,
            batch,
            &effects,
        )?);
        network_effects.extend(self.apply_messenger_incoming_effects(
            player_dao,
            messenger_dao,
            batch,
            &effects,
        )?);
        network_effects.extend(self.close_user_connections_network_effects(batch, &effects));

        network_effects.extend(
            self.collect_incoming_execution_runtime_plans(&effects, batch.connection_id())
                .into_iter()
                .map(|plan| match plan {
                    RoseauIncomingExecutionRuntimePlan::Network(effect) => effect,
                }),
        );

        let unapplied = self
            .startup_runtime_mut()
            .apply_network_effects(network_effects);
        Ok(unapplied
            .into_iter()
            .map(RoseauIncomingExecutionRuntimePlan::Network)
            .collect())
    }

    fn close_user_connections_network_effects(
        &self,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::CloseUserConnections))
        {
            return Vec::new();
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Vec::new();
        };

        PlayerEffectNetworkPlan::plan(
            &PlayerEffect::CloseUserConnections {
                user_id: session.details().id(),
            },
            self.game().player_manager(),
        )
    }

    fn apply_password_incoming_effects(
        &mut self,
        player_dao: &dyn PlayerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let actions = PasswordIncomingPlan::plan_all(effects);
        if actions.is_empty() {
            return Ok(Vec::new());
        }

        let current = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .map(|session| session.details().clone())
            .unwrap_or_default();
        let default_credits = self
            .game()
            .variables()
            .map(|variables| variables.user_default_credits())
            .unwrap_or_default();
        let base_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let outcomes = PlayerPasswordActionExecutor::execute_all(
            player_dao,
            self.game().player_manager(),
            &current,
            &actions,
            batch.connection_id(),
            batch.server_port(),
            base_server_port,
            default_credits,
        )?;

        for outcome in &outcomes {
            let Some(login) = outcome.login() else {
                continue;
            };
            let Some(details) = login.details() else {
                continue;
            };

            player_dao.update_last_login(details)?;
            let private_server_port = i32::from(
                self.startup_runtime()
                    .startup_plan()
                    .server_plan()
                    .private_server_port(),
            );
            let pending_private_room_id = (batch.server_port() == private_server_port)
                .then(|| {
                    self.game()
                        .player_manager()
                        .pending_private_room_id_for_user(details.id())
                })
                .flatten();
            let mut session =
                PlayerSession::new(batch.connection_id(), batch.server_port(), details.clone());
            if let Some(room_id) = pending_private_room_id {
                session.set_pending_private_room_id(room_id);
            }
            self.game_mut().player_manager_mut().insert(session);
            let room_login = login.public_room_lookup_id().is_some();
            let room_metadata = if room_login {
                let main_server_port = i32::from(
                    self.startup_runtime()
                        .startup_plan()
                        .server_plan()
                        .server_port(),
                );
                self.game()
                    .room_manager()
                    .get_room_by_port(batch.server_port(), main_server_port)
                    .map(|room| {
                        (
                            room.data().model_name().to_owned(),
                            room.data().name().to_owned(),
                        )
                    })
            } else {
                None
            };
            self.startup_runtime_mut().update_connection_context(
                batch.connection_id(),
                |context| {
                    context.set_authenticated(true);
                    context.set_credits(details.credits());
                    context.set_main_server_connection(!room_login);
                    context.set_in_room(room_login);
                    if let Some((model_name, room_name)) = room_metadata.as_ref() {
                        context.set_room_model_name(model_name);
                        context.set_current_room_name(room_name);
                    }
                },
            );
        }

        for outcome in &outcomes {
            let Some(profile_update) = outcome.profile_update() else {
                continue;
            };
            let Some(details) = profile_update.details() else {
                continue;
            };
            self.game_mut()
                .player_manager_mut()
                .sync_player_details(details);
        }

        let mut network_effects =
            PlayerPasswordActionNetworkPlan::plan_all(&outcomes, batch.connection_id());
        network_effects.extend(PlayerEffectNetworkPlan::plan_all(
            &outcomes
                .iter()
                .flat_map(|outcome| {
                    outcome
                        .login()
                        .map(|login| login.effects().to_vec())
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>(),
            self.game().player_manager(),
        ));

        Ok(network_effects)
    }

    fn update_pool_figure_network_effects(
        &mut self,
        player_dao: &dyn PlayerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(Vec::new());
        };
        let mut network_effects = Vec::new();

        for effect in effects {
            let IncomingExecutionEffect::UpdatePoolFigure { pool_figure } = effect else {
                continue;
            };
            let outcome = PlayerProfileUpdateExecutor::update_pool_figure(
                player_dao,
                session.details(),
                pool_figure,
            )?;
            let Some(details) = outcome.details() else {
                continue;
            };
            self.game_mut()
                .player_manager_mut()
                .sync_player_details(details);

            let room_player_ids = self.room_player_ids_for_batch(batch);
            if room_player_ids.is_empty() {
                continue;
            }

            let room_users = self
                .room_sessions_for_batch(batch)
                .into_iter()
                .map(|other| room_user_from_session(other, self.room_id_for_batch(batch)))
                .collect::<Vec<_>>();
            network_effects.extend(RoomUserEffectNetworkPlan::plan_all(
                &[RoomUserEffect::SendUsers {
                    entity_id: details.id(),
                }],
                details.id(),
                &room_player_ids,
                &room_users,
                self.game().player_manager(),
            ));
        }

        Ok(network_effects)
    }

    fn apply_personal_greeting_effects(
        &mut self,
        player_dao: &dyn PlayerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<(), DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(());
        };

        for effect in effects {
            let IncomingExecutionEffect::AssignPersonalMessage { message } = effect else {
                continue;
            };
            let outcome = PlayerProfileUpdateExecutor::update_personal_greeting(
                player_dao,
                session.details(),
                message,
            )?;
            let Some(details) = outcome.details() else {
                continue;
            };
            self.game_mut()
                .player_manager_mut()
                .sync_player_details(details);
        }

        Ok(())
    }

    fn player_incoming_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(Vec::new());
        };

        let outcomes = PlayerIncomingPlan::plan_all(
            &effects
                .iter()
                .filter(|effect| !matches!(effect, IncomingExecutionEffect::FindUser { .. }))
                .cloned()
                .collect::<Vec<_>>(),
            player_dao,
            session.details(),
            "now",
            "On Hotel View",
        )?;
        let mut network_effects = Vec::new();

        for outcome in outcomes {
            match outcome {
                PlayerIncomingOutcome::Command(outcome) => {
                    network_effects.extend(PlayerCommandNetworkPlan::plan(
                        &outcome,
                        batch.connection_id(),
                    ));
                }
                PlayerIncomingOutcome::FindUser(outcome) => {
                    network_effects
                        .extend(FindUserNetworkPlan::plan(&outcome, batch.connection_id()));
                }
            }
        }

        Ok(network_effects)
    }

    fn find_user_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            let IncomingExecutionEffect::FindUser { username } = effect else {
                continue;
            };

            let outcome = if username.is_empty() {
                FindUserOutcome::Missing
            } else {
                player_dao
                    .details_by_username(username)?
                    .map(|details| FindUserOutcome::found(&details, "now", "On Hotel View"))
                    .unwrap_or(FindUserOutcome::Missing)
            };
            outcomes.push(outcome);
        }

        Ok(FindUserNetworkPlan::plan_all(
            &outcomes,
            batch.connection_id(),
        ))
    }

    fn get_credits_network_effects(
        &self,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::GetCredits))
        {
            return Vec::new();
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Vec::new();
        };

        vec![
            write_response(
                batch.connection_id(),
                WalletBalance::new(session.details().credits())
                    .compose()
                    .get(),
            ),
            write_response(batch.connection_id(), MessengerSmsAccount.compose().get()),
            write_response(batch.connection_id(), MessengersReady.compose().get()),
        ]
    }

    fn room_command_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        item_dao: Option<&dyn ItemDao>,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let current_player = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .map(|session| session.details().clone())
            .unwrap_or_default();
        let mut network_effects = Vec::new();

        for effect in effects {
            network_effects.extend(self.close_public_room_connections_for_create_flat(
                effect,
                batch.connection_id(),
                current_player.id(),
            ));
            let has_owner_rights =
                self.has_room_command_owner_rights(room_dao, effect, current_player.id())?;
            if let IncomingExecutionEffect::DeleteFlat { room_id } = effect {
                if has_owner_rights {
                    Self::delete_room_items(item_dao, *room_id)?;
                }
            }
            let executions =
                RoomIncomingPlan::plan(effect, room_dao, &current_player, has_owner_rights)?;

            for execution in executions {
                let outcome = match execution {
                    RoomCommandExecution::Created(room) => {
                        self.game_mut()
                            .room_manager_mut()
                            .add(crate::game::room::RoomSummary::new(room.clone()));
                        RoomCommandExecution::Created(room).command_outcome()
                    }
                    RoomCommandExecution::Deleted { room_id } => {
                        network_effects.extend(self.deleted_room_cleanup_network_effects(room_id));
                        continue;
                    }
                    RoomCommandExecution::Updated(room)
                        if matches!(effect, IncomingExecutionEffect::SetFlatInfo { .. }) =>
                    {
                        network_effects
                            .extend(self.set_flat_info_privilege_network_effects(room_dao, &room)?);
                        RoomCommandExecution::Updated(room).command_outcome()
                    }
                    other => other.command_outcome(),
                };
                if let Some(packet) = outcome.flat_created() {
                    network_effects.push(write_response(
                        batch.connection_id(),
                        packet.compose().get(),
                    ));
                }
                if let Some(packet) = outcome.flat_info_packet() {
                    network_effects.push(write_response(
                        batch.connection_id(),
                        packet.compose().get(),
                    ));
                }
            }
        }

        Ok(network_effects)
    }

    fn delete_room_items(item_dao: Option<&dyn ItemDao>, room_id: i32) -> Result<(), DaoError> {
        let Some(item_dao) = item_dao else {
            return Ok(());
        };

        for item_id in item_dao
            .room_items(room_id)?
            .keys()
            .copied()
            .collect::<Vec<_>>()
        {
            item_dao.delete_item(i64::from(item_id))?;
        }

        Ok(())
    }

    fn deleted_room_cleanup_network_effects(
        &mut self,
        room_id: i32,
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        let private_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );
        let Some(room_port) = self
            .game()
            .room_manager()
            .get_room_by_id(room_id)
            .map(|room| room.data().server_port(private_server_port))
        else {
            return Vec::new();
        };
        let network_effects = self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| session.server_port() == room_port)
            .map(
                |session| crate::server::PlayerNetworkEffect::CloseConnection {
                    connection_id: session.connection_id(),
                },
            )
            .collect::<Vec<_>>();
        self.game_mut()
            .room_manager_mut()
            .remove_loaded_room(room_id);
        network_effects
    }

    fn set_flat_info_privilege_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        room_data: &RoomData,
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let mut room = Room::new(room_data.clone());
        room.load(room_dao.room_rights(room_data.id())?);
        let room_players = self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_data.id())
            })
            .map(|session| session.details().clone())
            .collect::<Vec<_>>();
        let mut room_effects = Vec::new();

        for player in &room_players {
            room_effects.push(RoomEffect::RemoveRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
            });
            let has_room_all_rights = self.has_room_all_rights(player.rank());
            room_effects.extend(room.refresh_flat_privileges(player, has_room_all_rights, false));
        }

        let mut network_effects =
            RoomEffectNetworkPlan::plan_all(&room_effects, self.game().player_manager());
        network_effects.extend(
            self.room_control_status_network_effects_for_room_id(room_data.id(), &room_effects),
        );
        Ok(network_effects)
    }

    fn close_public_room_connections_for_create_flat(
        &self,
        effect: &IncomingExecutionEffect,
        current_connection_id: i32,
        user_id: i32,
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !matches!(
            effect,
            IncomingExecutionEffect::CreateFlat { .. }
                | IncomingExecutionEffect::ClosePublicRoomConnections
        ) || user_id == 0
        {
            return Vec::new();
        }

        let base_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );

        self.game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session.connection_id() != current_connection_id
                    && session.details().id() == user_id
                    && self
                        .game()
                        .room_manager()
                        .get_room_by_port(session.server_port(), base_port)
                        .is_some_and(|room| room.data().room_type() == RoomType::Public)
            })
            .map(
                |session| crate::server::PlayerNetworkEffect::CloseConnection {
                    connection_id: session.connection_id(),
                },
            )
            .collect()
    }

    fn has_room_command_owner_rights(
        &self,
        room_dao: &dyn RoomDao,
        effect: &IncomingExecutionEffect,
        user_id: i32,
    ) -> Result<bool, DaoError> {
        match effect {
            IncomingExecutionEffect::CreateFlat { .. }
            | IncomingExecutionEffect::GetFlatInfo { .. } => Ok(true),
            IncomingExecutionEffect::DeleteFlat { room_id }
            | IncomingExecutionEffect::SetFlatInfo { room_id, .. }
            | IncomingExecutionEffect::UpdateFlat { room_id, .. } => {
                let Some(room) = room_dao.room(*room_id, false)? else {
                    return Ok(false);
                };
                let has_room_all_rights = self
                    .game()
                    .player_manager()
                    .get_by_id(user_id)
                    .is_some_and(|session| self.has_room_all_rights(session.details().rank()));
                Ok(room.owner_id() == user_id || has_room_all_rights)
            }
            _ => Ok(false),
        }
    }

    fn room_entry_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let mut network_effects = Vec::new();
        for effect in effects {
            let IncomingExecutionEffect::TryFlat { room_id, .. } = effect else {
                continue;
            };
            network_effects.extend(self.leave_current_room_for_try_flat(batch.connection_id()));
            let Some(session_details) = self
                .game()
                .player_manager()
                .players()
                .get(&batch.connection_id())
                .map(|session| session.details().clone())
            else {
                continue;
            };
            let Some(room_data) = room_dao.room(*room_id, true)? else {
                continue;
            };
            let mut room = Room::new(room_data);
            room.load(room_dao.room_rights(*room_id)?);
            let room_players = self.room_player_details_by_room_id(*room_id);
            let outcomes =
                RoomEntryIncomingPlan::plan(effect, &room, &session_details, &room_players, false);
            network_effects.extend(RoomEntryNetworkPlan::plan_all(
                &outcomes,
                batch.connection_id(),
            ));
            if outcomes
                .iter()
                .any(|outcome| matches!(outcome, crate::game::room::RoomEntryOutcome::LetIn))
            {
                self.game_mut()
                    .room_manager_mut()
                    .add(RoomSummary::new(room.data().clone()));
                if let Some(session) = self
                    .game_mut()
                    .player_manager_mut()
                    .get_mut(batch.connection_id())
                {
                    session.set_pending_private_room_id(*room_id);
                }
            }
            let doorbell_effects = outcomes
                .iter()
                .flat_map(|outcome| outcome.doorbell_effects())
                .cloned()
                .collect::<Vec<_>>();
            network_effects.extend(RoomEffectNetworkPlan::plan_all(
                &doorbell_effects,
                self.game().player_manager(),
            ));
        }

        Ok(network_effects)
    }

    fn leave_current_room_for_try_flat(
        &mut self,
        connection_id: i32,
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        let Some((user_id, username, room_id)) = self
            .game()
            .player_manager()
            .players()
            .get(&connection_id)
            .and_then(|session| {
                session.room_user().map(|room_user| {
                    (
                        session.details().id(),
                        session.details().username().to_owned(),
                        room_user.room_id(),
                    )
                })
            })
        else {
            return Vec::new();
        };

        let effects = RoomLeavePlan::new(user_id, username, room_id).effects();

        if let Some(session) = self.game_mut().player_manager_mut().get_mut(connection_id) {
            session.clear_room_user();
        }

        let mut remove_loaded_room = false;
        if let Some(room) = self
            .game_mut()
            .room_manager_mut()
            .get_room_by_id_mut(room_id)
        {
            let player_count = room.player_count().saturating_sub(1);
            room.set_player_count(player_count);
            remove_loaded_room = player_count == 0;
        }
        if remove_loaded_room {
            self.game_mut()
                .room_manager_mut()
                .remove_loaded_room(room_id);
        }

        let room_connection_ids = self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session.details().id() != user_id
                    && session
                        .room_user()
                        .is_some_and(|room_user| room_user.room_id() == room_id)
            })
            .map(|session| session.connection_id())
            .collect::<Vec<_>>();
        let private_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );

        RoomLeaveNetworkPlan::plan_all_for_connection_ids(
            &effects,
            &room_connection_ids,
            self.game().player_manager(),
            private_server_port,
        )
    }

    fn room_control_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects.iter().any(is_room_control_effect) {
            return Ok(Vec::new());
        }

        let Some(sender) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .map(|session| session.details().clone())
        else {
            return Ok(Vec::new());
        };
        let main_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let private_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );
        let Some(room_data) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .and_then(|session| {
                session.room_user().and_then(|room_user| {
                    self.game()
                        .room_manager()
                        .get_room_by_id(room_user.room_id())
                })
            })
            .or_else(|| {
                self.game()
                    .room_manager()
                    .get_room_by_port(batch.server_port(), main_server_port)
            })
            .map(|room| room.data().clone())
        else {
            return Ok(Vec::new());
        };

        let mut room = Room::new(room_data.clone());
        room.load(room_dao.room_rights(room_data.id())?);
        let has_room_all_rights = self.has_room_all_rights(sender.rank());
        let mut room_effects = Vec::new();

        for effect in effects {
            match effect {
                IncomingExecutionEffect::AssignRights { username } => {
                    let target = self.room_player_details_by_name_for_batch(batch, username);
                    room_effects.extend(room.assign_user_rights(
                        &sender,
                        target.as_ref(),
                        has_room_all_rights,
                    ));
                }
                IncomingExecutionEffect::RemoveRights { username } => {
                    let target = self.room_player_details_by_name_for_batch(batch, username);
                    room_effects.extend(room.revoke_user_rights(
                        &sender,
                        target.as_ref(),
                        has_room_all_rights,
                    ));
                }
                IncomingExecutionEffect::KickUser { username } => {
                    let target = self.room_player_details_by_name_for_batch(batch, username);
                    room_effects.extend(room.kick_user(
                        self.game().player_manager(),
                        &sender,
                        target.as_ref(),
                        has_room_all_rights,
                    ));
                }
                IncomingExecutionEffect::LetUserIn { username } => {
                    let target = self
                        .game()
                        .player_manager()
                        .get_private_room_player_by_name(username, private_server_port)
                        .map(|session| session.details().clone());
                    room_effects.extend(room.let_user_in(
                        &sender,
                        target.as_ref(),
                        has_room_all_rights,
                    ));
                }
                _ => {}
            }
        }

        for effect in &room_effects {
            if let RoomEffect::SaveRights { room_id, rights } = effect {
                room_dao.save_room_rights(*room_id, rights)?;
            }
        }

        let mut network_effects =
            RoomEffectNetworkPlan::plan_all(&room_effects, self.game().player_manager());
        network_effects.extend(self.room_control_status_network_effects(batch, &room_effects));

        Ok(network_effects)
    }

    fn room_control_status_network_effects(
        &mut self,
        batch: &PendingIncomingCommandBatch,
        room_effects: &[RoomEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !room_effects.iter().any(|effect| {
            matches!(
                effect,
                RoomEffect::SetRoomUserStatus { .. }
                    | RoomEffect::RemoveRoomUserStatus { .. }
                    | RoomEffect::MarkRoomUserForUpdate { .. }
            )
        }) {
            return Vec::new();
        }

        let room_player_ids = self.room_player_ids_for_batch(batch);
        let room_id = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .and_then(|session| session.room_user())
            .map(|room_user| room_user.room_id())
            .unwrap_or_else(|| batch.server_port());
        let mut room_users = self
            .room_sessions_for_batch(batch)
            .into_iter()
            .map(|session| room_user_from_session(session, room_id))
            .collect::<Vec<_>>();
        let mut user_effects = Vec::new();

        for user in &mut room_users {
            for effect in room_effects {
                RoomUserRoomEffectExecutor::apply(user.entity_id(), user, effect);
            }
            if user.needs_update() {
                user_effects.push(user.send_status_effect());
                user.set_needs_update(false);
            }
        }

        self.sync_room_users_to_player_sessions(&room_users);

        RoomUserEffectNetworkPlan::plan_all(
            &user_effects,
            -1,
            &room_player_ids,
            &room_users,
            self.game().player_manager(),
        )
    }

    fn room_control_status_network_effects_for_room_id(
        &mut self,
        room_id: i32,
        room_effects: &[RoomEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !room_effects.iter().any(|effect| {
            matches!(
                effect,
                RoomEffect::SetRoomUserStatus { .. }
                    | RoomEffect::RemoveRoomUserStatus { .. }
                    | RoomEffect::MarkRoomUserForUpdate { .. }
            )
        }) {
            return Vec::new();
        }

        let sessions = self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        let room_player_ids = sessions
            .iter()
            .map(|session| session.details().id())
            .collect::<Vec<_>>();
        let mut room_users = sessions
            .iter()
            .map(|session| room_user_from_session(session, room_id))
            .collect::<Vec<_>>();
        let mut user_effects = Vec::new();

        for user in &mut room_users {
            for effect in room_effects {
                RoomUserRoomEffectExecutor::apply(user.entity_id(), user, effect);
            }
            if user.needs_update() {
                user_effects.push(user.send_status_effect());
                user.set_needs_update(false);
            }
        }

        self.sync_room_users_to_player_sessions(&room_users);

        RoomUserEffectNetworkPlan::plan_all(
            &user_effects,
            -1,
            &room_player_ids,
            &room_users,
            self.game().player_manager(),
        )
    }

    fn room_player_details_by_name_for_batch(
        &self,
        batch: &PendingIncomingCommandBatch,
        username: &str,
    ) -> Option<crate::game::player::PlayerDetails> {
        self.room_sessions_for_batch(batch)
            .into_iter()
            .find(|session| session.details().username().eq_ignore_ascii_case(username))
            .map(|session| session.details().clone())
    }

    fn room_player_details_by_room_id(
        &self,
        room_id: i32,
    ) -> Vec<crate::game::player::PlayerDetails> {
        self.game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_id)
            })
            .map(|session| session.details().clone())
            .collect()
    }

    fn room_user_incoming_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        item_dao: Option<&dyn ItemDao>,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects.iter().any(|effect| {
            matches!(
                effect,
                IncomingExecutionEffect::SetRoomStatus { .. }
                    | IncomingExecutionEffect::RemoveRoomStatus { .. }
                    | IncomingExecutionEffect::MarkRoomNeedsUpdate
                    | IncomingExecutionEffect::ResetAfkTimer
                    | IncomingExecutionEffect::GoAway
                    | IncomingExecutionEffect::WalkTo { .. }
                    | IncomingExecutionEffect::LookTo { .. }
                    | IncomingExecutionEffect::Talk { .. }
                    | IncomingExecutionEffect::Command(_)
                    | IncomingExecutionEffect::EnterDoor { .. }
                    | IncomingExecutionEffect::SplashPosition { .. }
            )
        }) {
            return Ok(Vec::new());
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(Vec::new());
        };

        let main_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let room_context = session
            .room_user()
            .and_then(|room_user| {
                self.game()
                    .room_manager()
                    .get_room_by_id(room_user.room_id())
            })
            .or_else(|| {
                self.game()
                    .room_manager()
                    .get_room_by_port(batch.server_port(), main_server_port)
            })
            .map(|room| (room.data().id(), room.data().model_name().to_owned()));
        let (room_id, room_model) = if let Some((room_id, model_name)) = room_context {
            (room_id, room_dao.model(&model_name)?)
        } else {
            (batch.server_port(), None)
        };

        let room_player_ids = self.room_player_ids_for_batch(batch);
        if room_player_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut room_users = self
            .room_sessions_for_batch(batch)
            .into_iter()
            .map(|other| room_user_from_session(other, room_id))
            .collect::<Vec<_>>();

        let mut acting_user = room_user_from_session(&session, room_id);
        let mut room_user_effects = Vec::new();
        for effect in effects {
            let requires_item_dao = matches!(
                effect,
                IncomingExecutionEffect::EnterDoor { .. }
                    | IncomingExecutionEffect::SplashPosition { .. }
            );
            if item_dao.is_some() != requires_item_dao {
                continue;
            }
            room_user_effects.extend(Self::apply_persistent_room_user_effect(
                &mut acting_user,
                effect,
                room_model.as_ref(),
                &room_users,
                item_dao,
                room_id,
            )?);
        }
        if acting_user.needs_update() {
            room_user_effects.push(acting_user.send_status_effect());
            acting_user.set_needs_update(false);
        }

        if let Some(session) = self
            .game_mut()
            .player_manager_mut()
            .get_mut(batch.connection_id())
        {
            session.set_room_user(acting_user.clone());
        }
        if let Some(user) = room_users
            .iter_mut()
            .find(|user| user.entity_id() == acting_user.entity_id())
        {
            *user = acting_user;
        } else {
            room_users.push(acting_user);
        }

        Self::apply_java_talk_recipient_looks(
            &mut room_users,
            &mut room_user_effects,
            effects,
            session.details().id(),
            self.runtime().game_variables().talk_look_at_reset() as i64,
        );
        self.sync_room_users_to_player_sessions(&room_users);

        Ok(RoomUserEffectNetworkPlan::plan_all(
            &room_user_effects,
            session.details().id(),
            &room_player_ids,
            &room_users,
            self.game().player_manager(),
        ))
    }

    fn apply_java_talk_recipient_looks(
        room_users: &mut [RoomUser],
        room_user_effects: &mut Vec<RoomUserEffect>,
        effects: &[IncomingExecutionEffect],
        acting_user_id: i32,
        talk_look_at_reset: i64,
    ) {
        let Some(speaker_position) = room_users
            .iter()
            .find(|user| user.entity_id() == acting_user_id)
            .map(RoomUser::position)
        else {
            return;
        };

        for effect in effects {
            let IncomingExecutionEffect::Talk { mode, .. } = effect else {
                continue;
            };
            if !matches!(mode.as_str(), "CHAT" | "SHOUT") {
                continue;
            }

            for user in room_users
                .iter_mut()
                .filter(|user| user.entity_id() != acting_user_id)
            {
                if mode == "CHAT"
                    && user.position().distance(speaker_position) > crate::settings::TALK_DISTANCE
                {
                    continue;
                }

                user.look_towards(speaker_position);
                user.set_look_reset_time(talk_look_at_reset);
                if user.needs_update() {
                    room_user_effects.push(user.send_status_effect());
                    user.set_needs_update(false);
                }
            }
        }
    }

    fn sync_room_users_to_player_sessions(&mut self, room_users: &[RoomUser]) {
        let updates = room_users
            .iter()
            .filter_map(|room_user| {
                self.game()
                    .player_manager()
                    .players()
                    .values()
                    .find(|session| {
                        session.details().id() == room_user.entity_id()
                            && session
                                .room_user()
                                .is_some_and(|current| current.room_id() == room_user.room_id())
                    })
                    .map(|session| (session.connection_id(), room_user.clone()))
            })
            .collect::<Vec<_>>();

        for (connection_id, room_user) in updates {
            if let Some(session) = self.game_mut().player_manager_mut().get_mut(connection_id) {
                session.set_room_user(room_user);
            }
        }
    }

    fn apply_persistent_room_user_effect(
        user: &mut RoomUser,
        effect: &IncomingExecutionEffect,
        room_model: Option<&RoomModel>,
        room_users: &[RoomUser],
        item_dao: Option<&dyn ItemDao>,
        room_id: i32,
    ) -> Result<Vec<RoomUserEffect>, DaoError> {
        match effect {
            IncomingExecutionEffect::WalkTo { x, y } => {
                let Some(model) = room_model else {
                    return Ok(Vec::new());
                };
                let path = Self::room_user_path(user, *x, *y, model, room_users);
                user.walk_to(*x, *y, path);
                Ok(Vec::new())
            }
            IncomingExecutionEffect::GoAway => {
                let Some(model) = room_model else {
                    return Ok(Vec::new());
                };
                let door = Position::with_rotation(
                    model.door_x(),
                    model.door_y(),
                    model.door_z() as f64,
                    model.door_rotation(),
                );
                let path = Self::room_user_path(user, door.x(), door.y(), model, room_users);
                Ok(user.go_away(door, path))
            }
            IncomingExecutionEffect::EnterDoor { item_id } => {
                let (Some(model), Some(item_dao)) = (room_model, item_dao) else {
                    return Ok(Vec::new());
                };
                let Some(item) = item_dao.room_items(room_id)?.remove(item_id) else {
                    return Ok(Vec::new());
                };
                if !Self::can_enter_teleporter_from_position(user.position(), &item) {
                    return Ok(Vec::new());
                }
                let target = item.position();
                let path = Self::room_user_path(user, target.x(), target.y(), model, room_users);
                user.walk_to(target.x(), target.y(), path);
                Ok(Vec::new())
            }
            IncomingExecutionEffect::SplashPosition { position } => {
                let (Some(model), Some(item_dao)) = (room_model, item_dao) else {
                    return Ok(Vec::new());
                };
                let Some(item_id) = user.current_item_id() else {
                    return Ok(Vec::new());
                };
                let Some(item) = item_dao.item(item_id)? else {
                    return Ok(Vec::new());
                };
                if item.definition().sprite() != "poolLift" {
                    return Ok(Vec::new());
                }
                let Ok(landing_position) = Position::parse_xyz(position) else {
                    return Ok(Vec::new());
                };
                let exit_path = Self::room_user_path(user, 18, 19, model, room_users);
                Ok(user.splash_from_pool_lift(landing_position, exit_path))
            }
            _ => Ok(RoomUserIncomingPlan::plan(user, effect)),
        }
    }

    fn can_enter_teleporter_from_position(position: Position, item: &Item) -> bool {
        if !item.definition().behaviour().is_teleporter() {
            return false;
        }

        let item_position = item.position();
        matches!(item_position.rotation(), 0 | 2)
            && position.x() == item_position.x() + 1
            && position.y() == item_position.y()
            || item_position.rotation() == 4
                && position.x() == item_position.x()
                && position.y() == item_position.y() + 1
    }

    fn room_user_path(
        user: &RoomUser,
        x: i32,
        y: i32,
        model: &RoomModel,
        room_users: &[RoomUser],
    ) -> Vec<Position> {
        let mapping = RoomMapping::new(model.clone());
        let occupants = Self::room_occupants(room_users);

        make_path(
            user.position(),
            Position::new(x, y, 0.0),
            model.map_size_x(),
            model.map_size_y(),
            |_, position, _| {
                mapping.is_valid_tile(
                    user.entity_id(),
                    position.x(),
                    position.y(),
                    &[],
                    &occupants,
                    false,
                )
            },
        )
    }

    fn room_occupants(room_users: &[RoomUser]) -> Vec<RoomOccupant> {
        room_users
            .iter()
            .map(|user| RoomOccupant::new(user.entity_id(), user.position(), user.goal()))
            .collect()
    }

    fn moderation_incoming_network_effects(
        &self,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::CryForHelp { .. }))
        {
            return Vec::new();
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Vec::new();
        };

        let main_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let room = moderation_room_context_for_port(self, batch.server_port(), main_server_port);
        let moderation_effects = effects
            .iter()
            .flat_map(|effect| {
                ModerationIncomingPlan::plan(
                    effect,
                    self.game().player_manager(),
                    self.game().moderation_manager(),
                    main_server_port,
                    room,
                    session.details().username(),
                    "now",
                )
            })
            .collect::<Vec<_>>();

        self.plan_moderation_effect_network_effects(&moderation_effects)
    }

    fn room_pool_network_effects(
        &self,
        item_dao: &dyn ItemDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects.iter().any(|effect| {
            matches!(
                effect,
                IncomingExecutionEffect::JumpPerformance { .. }
                    | IncomingExecutionEffect::SplashPosition { .. }
            )
        }) {
            return Ok(Vec::new());
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(Vec::new());
        };
        let user_id = session.details().id();
        let has_splash_position = effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::SplashPosition { .. }));
        let invalid_splash_effects = || {
            if has_splash_position {
                PlayerEffectNetworkPlan::plan(
                    &PlayerEffect::CloseUserConnections { user_id },
                    self.game().player_manager(),
                )
            } else {
                Vec::new()
            }
        };

        let Some(item_id) = session
            .room_user()
            .and_then(|room_user| room_user.current_item_id())
        else {
            return Ok(invalid_splash_effects());
        };
        let Some(item) = item_dao.item(item_id)? else {
            return Ok(invalid_splash_effects());
        };
        if item.definition().sprite() != "poolLift" {
            return Ok(invalid_splash_effects());
        };

        let room_connection_ids = self.room_connection_ids_for_batch(batch);
        let mut network_effects = RoomPoolNetworkPlan::plan_all_for_connection_ids(
            effects,
            session.details().username(),
            &room_connection_ids,
        );
        if has_splash_position {
            let open_effects = PoolLiftInteractor::open(&item);
            network_effects.extend(
                ItemInteractionEffectNetworkPlan::plan_all_for_connection_ids(
                    &open_effects,
                    batch.connection_id(),
                    session.details().username(),
                    session.details().tickets(),
                    &room_connection_ids,
                    std::slice::from_ref(&item),
                ),
            );
        }

        Ok(network_effects)
    }

    fn close_pool_change_booth_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::ClosePoolChangeBooth))
        {
            return Ok(Vec::new());
        }

        let Some(context) = self.incoming_room_context(room_dao, batch)? else {
            return Ok(Vec::new());
        };
        let Some(model) = self
            .game()
            .room_manager()
            .get_room_by_id(context.room_id)
            .and_then(|room| room_dao.model(room.data().model_name()).ok().flatten())
        else {
            return Ok(Vec::new());
        };
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(Vec::new());
        };

        let items = item_dao.room_items(context.room_id)?;
        let items = items.into_values().collect::<Vec<_>>();
        let mut mapping = RoomMapping::new(model.clone());
        mapping.regenerate_collision_maps(items.clone());

        let mut acting_user = room_user_from_session(&session, context.room_id);
        let Some(item_id) =
            mapping.highest_item_id(acting_user.position().x(), acting_user.position().y())
        else {
            return Ok(Vec::new());
        };
        let Some(item) = items.iter().find(|item| item.id() == item_id) else {
            return Ok(Vec::new());
        };
        if item.definition().sprite() != "poolBooth" {
            return Ok(Vec::new());
        }

        let room_player_ids = self.room_player_ids_for_batch(batch);
        let room_connection_ids = self.room_connection_ids_for_batch(batch);
        let room_users = self
            .room_sessions_for_batch(batch)
            .into_iter()
            .map(|other| room_user_from_session(other, context.room_id))
            .collect::<Vec<_>>();
        let occupants = Self::room_occupants(&room_users);
        let interaction_effects = PoolChangeBoothInteractor::close_ui(item);
        let mut network_effects = ItemInteractionEffectNetworkPlan::plan_all_for_connection_ids(
            &interaction_effects,
            batch.connection_id(),
            session.details().username(),
            session.details().tickets(),
            &room_connection_ids,
            &items,
        );

        ItemInteractionEffectExecutor::apply_all(&mut acting_user, &interaction_effects);
        let mut room_user_effects = ItemInteractionEffectRoomExecutor::apply_all(
            &mut acting_user,
            &interaction_effects,
            &mut mapping,
            &items,
            &occupants,
            !session.details().pool_figure().is_empty(),
            session.details().tickets(),
        );
        if acting_user.needs_update() {
            room_user_effects.push(acting_user.send_status_effect());
            acting_user.set_needs_update(false);
        }

        if let Some(session) = self
            .game_mut()
            .player_manager_mut()
            .get_mut(batch.connection_id())
        {
            session.set_room_user(acting_user.clone());
        }
        let mut updated_room_users = room_users;
        if let Some(user) = updated_room_users
            .iter_mut()
            .find(|user| user.entity_id() == acting_user.entity_id())
        {
            *user = acting_user;
        } else {
            updated_room_users.push(acting_user);
        }
        network_effects.extend(RoomUserEffectNetworkPlan::plan_all(
            &room_user_effects,
            session.details().id(),
            &room_player_ids,
            &updated_room_users,
            self.game().player_manager(),
        ));

        Ok(network_effects)
    }

    fn decoration_incoming_network_effects(
        &self,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
        inventory_dao: &dyn InventoryDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::ApplyDecoration { .. }))
        {
            return Ok(Vec::new());
        }

        let Some(context) = self.incoming_room_context(room_dao, batch)? else {
            return Ok(Vec::new());
        };

        let outcomes = RoomDecorationIncomingPlan::plan_all(
            effects,
            item_dao,
            room_dao,
            context.room_id,
            context.has_owner_rights,
        )?;

        let mut network_effects = RoomDecorationNetworkPlan::plan_all_for_connection_ids(
            &outcomes,
            &self.room_connection_ids_for_batch(batch),
        );
        if outcomes.iter().any(|outcome| {
            matches!(
                outcome,
                crate::game::room::RoomDecorationOutcome::Applied { .. }
            )
        }) {
            let refresh =
                InventoryCommandExecutor::refresh_inventory(inventory_dao, context.user_id, "new")?;
            let mut inventory_effects =
                InventoryCommandNetworkPlan::plan(&refresh, batch.connection_id());
            if inventory_effects.is_empty() {
                let mut response = StripInfo::new([]).compose();
                inventory_effects.push(crate::server::PlayerNetworkEffect::WriteResponse {
                    connection_id: batch.connection_id(),
                    packet: response.get(),
                });
            }
            network_effects.extend(inventory_effects);
        }

        Ok(network_effects)
    }

    fn item_incoming_network_effects(
        &self,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
        inventory_dao: &dyn InventoryDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects.iter().any(is_item_command_effect) {
            return Ok(Vec::new());
        }

        let executions = if let Some(context) = self.incoming_room_context(room_dao, batch)? {
            ItemIncomingPlan::plan_all(
                effects,
                item_dao,
                context.room_id,
                context.room_owner_id,
                context.user_id,
                context.has_rights,
                context.has_owner_rights,
                context.all_super_user,
                None,
            )?
        } else {
            let strip_effects = effects
                .iter()
                .filter(|effect| matches!(effect, IncomingExecutionEffect::UseStripItem { .. }))
                .cloned()
                .collect::<Vec<_>>();
            ItemIncomingPlan::plan_all(
                &strip_effects,
                item_dao,
                0,
                0,
                0,
                false,
                false,
                false,
                None,
            )?
        };
        let room_connection_ids = self.room_connection_ids_for_batch(batch);
        let mut network_effects =
            ItemCommandNetworkPlan::plan_all_for_connection_ids(&executions, &room_connection_ids);

        if executions
            .iter()
            .any(|execution| matches!(execution, ItemCommandExecution::Deleted { .. }))
        {
            if let Some(session) = self
                .game()
                .player_manager()
                .players()
                .get(&batch.connection_id())
            {
                let refresh = InventoryCommandExecutor::refresh_inventory(
                    inventory_dao,
                    session.details().id(),
                    "last",
                )?;
                network_effects.extend(InventoryCommandNetworkPlan::plan(
                    &refresh,
                    batch.connection_id(),
                ));
            }
        }

        Ok(network_effects)
    }

    fn goto_flat_network_effects(
        &mut self,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::GoToFlat))
        {
            return Ok(Vec::new());
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(Vec::new());
        };
        let main_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let private_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );
        let room = if batch.server_port() == private_server_port {
            let Some(room_id) = session.pending_private_room_id() else {
                return Ok(Vec::new());
            };
            if self.game().room_manager().get_room_by_id(room_id).is_none() {
                if let Some(room_data) = room_dao.room(room_id, true)? {
                    self.game_mut()
                        .room_manager_mut()
                        .add(RoomSummary::new(room_data));
                }
            }
            self.game().room_manager().get_room_by_id(room_id).cloned()
        } else {
            self.game()
                .room_manager()
                .get_room_by_port(batch.server_port(), main_server_port)
                .cloned()
        };
        let Some(room) = room else {
            return Ok(vec![crate::server::PlayerNetworkEffect::CloseConnection {
                connection_id: batch.connection_id(),
            }]);
        };

        let room_data = room.data();
        let mut packets = Vec::new();
        let mut room_items = item_dao
            .room_items(room_data.id())?
            .into_values()
            .collect::<Vec<_>>();
        room_items.sort_by_key(|item| item.id());
        let mut passive_objects = item_dao
            .public_room_items(room_data.model_name(), room_data.id())?
            .into_values()
            .collect::<Vec<_>>();
        passive_objects.sort_by_key(|item| item.id());

        if room_data.room_type() == RoomType::Private {
            packets.push(RoomReady::new(room_data.description()).compose().get());
            packets.push(
                FlatProperty::new("wallpaper", private_room_wallpaper(room_data.wall()))
                    .compose()
                    .get(),
            );
            packets.push(
                FlatProperty::new("floor", private_room_floor(room_data.floor()))
                    .compose()
                    .get(),
            );
        }

        let rights = room_dao.room_rights(room_data.id())?;
        let has_room_all_rights = self.has_room_all_rights(session.details().rank());
        if room_data.owner_id() == session.details().id() || has_room_all_rights {
            packets.push(YouAreOwner.compose().get());
        } else if room_data.has_all_super_user()
            || rights
                .iter()
                .any(|right_user_id| *right_user_id == session.details().id())
        {
            packets.push(YouAreController.compose().get());
        }

        let room_model = room_dao.model(room_data.model_name())?;
        if let Some(model) = room_model.as_ref() {
            packets.push(HeightMap::new(model.height_map()).compose().get());
        }
        packets.push(
            ObjectsWorld::new(room_data.model_name(), passive_objects)
                .compose()
                .get(),
        );
        packets.push(
            ActiveObjects::new(
                room_items
                    .iter()
                    .filter(|item| item.definition().behaviour().is_on_floor())
                    .cloned(),
            )
            .compose()
            .get(),
        );
        if room_data.room_type() == RoomType::Private {
            packets.push(
                Items::new(
                    room_items
                        .iter()
                        .filter(|item| item.definition().behaviour().is_on_wall())
                        .cloned(),
                )
                .compose()
                .get(),
            );
        }

        let existing_room_users = self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|other| other.connection_id() != batch.connection_id())
            .filter(|other| {
                other
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_data.id())
            })
            .map(|other| room_user_from_session(other, room_data.id()))
            .collect::<Vec<_>>();
        packets.push(
            Users::new(existing_room_users.iter().map(|user| user.user_entry()))
                .compose()
                .get(),
        );
        packets.push(
            Status::new(existing_room_users.iter().map(|user| user.status_entity()))
                .compose()
                .get(),
        );

        let mut current_user = room_user_from_session(&session, room_data.id());
        if let Some(model) = room_model.as_ref() {
            let door_position = Position::with_rotation(
                model.door_x(),
                model.door_y(),
                model.door_z() as f64,
                model.door_rotation(),
            );
            current_user.set_position(door_position);
        }
        packets.push(Users::new([current_user.user_entry()]).compose().get());
        packets.push(Status::new([current_user.status_entity()]).compose().get());

        if let Some(session) = self
            .game_mut()
            .player_manager_mut()
            .get_mut(batch.connection_id())
        {
            session.set_room_user(current_user);
            session.clear_pending_private_room_id();
        }

        self.startup_runtime_mut()
            .update_connection_context(batch.connection_id(), |context| {
                context.set_main_server_connection(false);
                context.set_in_room(true);
                context.set_room_model_name(room_data.model_name());
                context.set_current_room_name(room_data.name());
            });

        Ok(packets
            .into_iter()
            .map(|packet| write_response(batch.connection_id(), packet))
            .collect())
    }

    fn add_wall_item_network_effects(
        &self,
        room_dao: &dyn RoomDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::AddWallItem { .. }))
        {
            return Ok(Vec::new());
        }

        let Some(context) = self.incoming_room_context(room_dao, batch)? else {
            return Ok(Vec::new());
        };
        if !context.has_explicit_rights {
            return Ok(Vec::new());
        }

        let inventory_items = inventory_dao.inventory_items(context.user_id)?;
        let mut executions = Vec::new();

        for effect in effects {
            let IncomingExecutionEffect::AddWallItem {
                sprite,
                wall_position,
                extra_data,
            } = effect
            else {
                continue;
            };
            let Some(inventory_item) = inventory_items
                .iter()
                .find(|item| item.definition().sprite() == sprite)
            else {
                continue;
            };
            let definition = inventory_item.definition();
            if !definition.behaviour().is_on_wall() {
                continue;
            }

            let mut item = inventory_dao.new_item(definition.id(), context.user_id, extra_data)?;
            item.set_room_id(context.room_id);
            item.set_wall_position(wall_position);
            item_dao.save_item(&item)?;
            executions.push(ItemCommandExecution::RoomItemPlaced(item));
        }

        Ok(ItemCommandNetworkPlan::plan_all_for_connection_ids(
            &executions,
            &self.room_connection_ids_for_batch(batch),
        ))
    }

    fn incoming_room_context(
        &self,
        room_dao: &dyn RoomDao,
        batch: &PendingIncomingCommandBatch,
    ) -> Result<Option<IncomingRoomContext>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(None);
        };

        let main_server_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let Some(room) = session
            .room_user()
            .and_then(|room_user| {
                self.game()
                    .room_manager()
                    .get_room_by_id(room_user.room_id())
            })
            .or_else(|| {
                self.game()
                    .room_manager()
                    .get_room_by_port(batch.server_port(), main_server_port)
            })
        else {
            return Ok(None);
        };

        let user_id = session.details().id();
        let room_id = room.data().id();
        let rights = room_dao.room_rights(room_id)?;
        let has_room_all_rights = self.has_room_all_rights(session.details().rank());
        let has_owner_rights = room.data().owner_id() == user_id || has_room_all_rights;
        let has_explicit_rights =
            has_owner_rights || rights.iter().any(|right_user_id| *right_user_id == user_id);
        let has_rights = has_owner_rights
            || room.data().has_all_super_user()
            || rights.iter().any(|right_user_id| *right_user_id == user_id);

        Ok(Some(IncomingRoomContext {
            user_id,
            room_id,
            room_owner_id: room.data().owner_id(),
            has_rights,
            has_explicit_rights,
            has_owner_rights,
            all_super_user: room.data().has_all_super_user(),
        }))
    }

    fn has_room_all_rights(&self, rank: i32) -> bool {
        let player_manager = self.game().player_manager();
        player_manager.has_permission(rank, "room_all_rights")
            || player_manager.has_permission(rank, "room_admin")
    }

    fn room_id_for_batch(&self, batch: &PendingIncomingCommandBatch) -> i32 {
        self.game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .and_then(|session| session.room_user())
            .map(|room_user| room_user.room_id())
            .unwrap_or_else(|| batch.server_port())
    }

    fn room_sessions_for_batch(
        &self,
        batch: &PendingIncomingCommandBatch,
    ) -> Vec<&crate::game::player::PlayerSession> {
        let Some(room_id) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .and_then(|session| session.room_user())
            .map(|room_user| room_user.room_id())
        else {
            return self
                .game()
                .player_manager()
                .players()
                .values()
                .filter(|session| session.server_port() == batch.server_port())
                .collect();
        };
        let room_type = self
            .game()
            .room_manager()
            .get_room_by_id(room_id)
            .map(|room| room.data().room_type());

        self.game()
            .player_manager()
            .players()
            .values()
            .filter(|session| {
                session
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_id)
                    || (room_type == Some(RoomType::Public)
                        && session.server_port() == batch.server_port())
            })
            .collect()
    }

    fn room_player_ids_for_batch(&self, batch: &PendingIncomingCommandBatch) -> Vec<i32> {
        self.room_sessions_for_batch(batch)
            .into_iter()
            .map(|session| session.details().id())
            .collect()
    }

    fn room_connection_ids_for_batch(&self, batch: &PendingIncomingCommandBatch) -> Vec<i32> {
        self.room_sessions_for_batch(batch)
            .into_iter()
            .map(|session| session.connection_id())
            .collect()
    }

    fn inventory_incoming_network_effects(
        &self,
        inventory_dao: &dyn InventoryDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(Vec::new());
        };

        let executions =
            InventoryIncomingPlan::plan_all(effects, inventory_dao, session.details().id())?;
        Ok(InventoryCommandNetworkPlan::plan_all(
            &executions,
            batch.connection_id(),
        ))
    }

    fn catalogue_incoming_network_effects(
        &mut self,
        player_dao: &dyn PlayerDao,
        catalogue_daos: CatalogueIncomingDaoSet<'_>,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
            .cloned()
        else {
            return Ok(Vec::new());
        };

        let outcomes = CatalogueIncomingPlan::plan_all(
            effects,
            self.game().catalogue_manager(),
            catalogue_daos.catalogue,
            catalogue_daos.inventory,
            catalogue_daos.item,
            player_dao,
            session.details(),
        )?;
        let mut network_effects = Vec::new();

        for outcome in outcomes {
            match outcome {
                CatalogueIncomingOutcome::OrderInfo(plan) => {
                    network_effects.extend(CatalogueOrderInfoNetworkPlan::plan(
                        &plan,
                        batch.connection_id(),
                    ));
                }
                CatalogueIncomingOutcome::Purchase(execution) => {
                    let purchase_outcome = match &execution {
                        CataloguePurchaseExecution::Purchased { buyer, .. } => {
                            self.game_mut()
                                .player_manager_mut()
                                .insert(PlayerSession::new(
                                    session.connection_id(),
                                    session.server_port(),
                                    buyer.clone(),
                                ));
                            Some(CataloguePurchaseOutcome::AddedStripItem)
                        }
                        CataloguePurchaseExecution::NotEnoughCredits => {
                            Some(CataloguePurchaseOutcome::NotEnoughCredits)
                        }
                        CataloguePurchaseExecution::Ignored => None,
                    };
                    if let Some(purchase_outcome) = purchase_outcome {
                        network_effects.extend(CataloguePurchaseNetworkPlan::plan(
                            &purchase_outcome,
                            batch.connection_id(),
                        ));
                    }
                }
                CatalogueIncomingOutcome::TicketPurchase(execution) => {
                    let target_connection_id = match &execution {
                        CatalogueTicketPurchaseExecution::Purchased {
                            target: Some(target),
                            ..
                        } => self
                            .game()
                            .player_manager()
                            .get_by_id(target.id())
                            .map(|session| session.connection_id()),
                        _ => None,
                    };
                    if let Some(outcome) = execution.outcome() {
                        network_effects.extend(CatalogueTicketPurchaseNetworkPlan::plan(
                            outcome,
                            batch.connection_id(),
                            target_connection_id,
                        ));
                    }
                }
            }
        }

        Ok(network_effects)
    }

    fn room_unit_incoming_network_effects(
        &self,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<crate::server::PlayerNetworkEffect> {
        let base_port = i32::from(
            self.startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let outcomes = RoomUnitIncomingPlan::plan_all(
            effects,
            self.game().room_manager(),
            self.game().player_manager(),
            base_port,
        );

        RoomUnitNetworkPlan::plan_all(
            &outcomes,
            batch.connection_id(),
            &self.advertised_server_ip(),
            base_port,
        )
    }

    fn navigator_incoming_network_effects(
        &self,
        room_dao: &dyn RoomDao,
        navigator_dao: &dyn NavigatorDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let private_server_port = self
            .startup_runtime()
            .startup_plan()
            .server_plan()
            .private_server_port();
        let outcomes = crate::game::navigator::NavigatorIncomingPlan::plan_all(
            effects,
            navigator_dao,
            room_dao,
            self.game().room_manager(),
            self.game().player_manager(),
            &self.advertised_server_ip(),
            private_server_port,
        )?;

        Ok(NavigatorSearchNetworkPlan::plan_all(
            &outcomes,
            batch.connection_id(),
        ))
    }

    fn messenger_init_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        if !effects
            .iter()
            .any(|effect| matches!(effect, IncomingExecutionEffect::InitMessenger))
        {
            return Ok(Vec::new());
        }

        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(Vec::new());
        };

        let mut network_effects = Vec::new();
        network_effects.push(write_response(
            batch.connection_id(),
            MyPersistentMessage::new(session.details().personal_greeting())
                .compose()
                .get(),
        ));

        let request_names = messenger_dao
            .requests(session.details().id())?
            .into_iter()
            .filter_map(|user| player_dao.details_by_id(user.user_id()).ok().flatten())
            .map(|details| details.username().to_owned())
            .collect::<Vec<_>>();
        if !request_names.is_empty() {
            network_effects.push(write_response(
                batch.connection_id(),
                BuddyAddRequests::new(request_names).compose().get(),
            ));
        }

        let friends = messenger_dao
            .friends(session.details().id())?
            .into_iter()
            .filter_map(|user| player_dao.details_by_id(user.user_id()).ok().flatten())
            .map(|details| {
                let online = self
                    .game()
                    .player_manager()
                    .get_by_id(details.id())
                    .is_some();
                BuddyListFriend::new(
                    details.id(),
                    details.username(),
                    details.personal_greeting(),
                    online.then_some("On Hotel View"),
                    details.last_online().to_string(),
                )
            })
            .collect::<Vec<_>>();
        if !friends.is_empty() {
            network_effects.push(write_response(
                batch.connection_id(),
                BuddyList::new(friends, None).compose().get(),
            ));
        }

        for message in messenger_dao.unread_messages(session.details().id())? {
            let figure = player_dao
                .details_by_id(message.from_id())?
                .map(|details| details.figure().to_owned())
                .unwrap_or_default();
            network_effects.push(write_response(
                batch.connection_id(),
                MessengerMessagePacket::new(
                    message.id(),
                    message.from_id(),
                    message.time_sent().to_string(),
                    message.message(),
                    figure,
                )
                .compose()
                .get(),
            ));
        }

        network_effects.push(write_response(
            batch.connection_id(),
            MessengerReady.compose().get(),
        ));

        Ok(network_effects)
    }

    fn apply_messenger_incoming_effects(
        &self,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        batch: &PendingIncomingCommandBatch,
        effects: &[IncomingExecutionEffect],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let Some(session) = self
            .game()
            .player_manager()
            .players()
            .get(&batch.connection_id())
        else {
            return Ok(Vec::new());
        };

        let mut messenger = Messenger::new(session.details().id());
        messenger.load_from_dao(messenger_dao)?;
        let outcomes =
            MessengerIncomingPlan::plan_all(effects, player_dao, messenger_dao, &messenger)?;

        self.messenger_outcome_network_effects(
            player_dao,
            messenger_dao,
            session.details().id(),
            session.details().figure(),
            &outcomes,
        )
    }

    fn messenger_outcome_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        sender_id: i32,
        sender_figure: &str,
        outcomes: &[MessengerCommandOutcome],
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let mut network_effects = Vec::new();

        for outcome in outcomes {
            match outcome {
                MessengerCommandOutcome::BuddyRequestCreated { to_id } => {
                    network_effects.extend(self.messenger_request_network_effects(
                        player_dao,
                        messenger_dao,
                        *to_id,
                    )?);
                }
                MessengerCommandOutcome::BuddyAccepted { user_id }
                | MessengerCommandOutcome::BuddyRemoved { user_id } => {
                    network_effects.extend(self.messenger_friend_refresh_network_effects(
                        player_dao,
                        messenger_dao,
                        sender_id,
                        None,
                    )?);
                    network_effects.extend(self.messenger_friend_refresh_network_effects(
                        player_dao,
                        messenger_dao,
                        *user_id,
                        None,
                    )?);
                }
                MessengerCommandOutcome::MessagesSent { deliveries } => {
                    for delivery in deliveries {
                        let Some(message) = messenger_dao
                            .unread_messages(delivery.receiver_id())?
                            .into_iter()
                            .find(|message| message.id() == delivery.message_id())
                        else {
                            continue;
                        };

                        for recipient in self
                            .game()
                            .player_manager()
                            .players()
                            .values()
                            .filter(|session| session.details().id() == delivery.receiver_id())
                        {
                            network_effects.push(write_response(
                                recipient.connection_id(),
                                MessengerMessagePacket::new(
                                    message.id(),
                                    sender_id,
                                    message.time_sent().to_string(),
                                    message.message(),
                                    sender_figure,
                                )
                                .compose()
                                .get(),
                            ));
                        }
                    }
                }
                MessengerCommandOutcome::BuddyDeclined { .. }
                | MessengerCommandOutcome::MessageMarkedRead { .. }
                | MessengerCommandOutcome::Ignored => {}
            }
        }

        Ok(network_effects)
    }

    fn messenger_request_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        to_id: i32,
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let request_names = messenger_dao
            .requests(to_id)?
            .into_iter()
            .filter_map(|user| player_dao.details_by_id(user.user_id()).ok().flatten())
            .map(|details| details.username().to_owned())
            .collect::<Vec<_>>();
        if request_names.is_empty() {
            return Ok(Vec::new());
        }

        Ok(self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| session.details().id() == to_id)
            .map(|session| {
                write_response(
                    session.connection_id(),
                    BuddyAddRequests::new(request_names.clone()).compose().get(),
                )
            })
            .collect())
    }

    fn messenger_friend_refresh_network_effects(
        &self,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        user_id: i32,
        offline_user_id: Option<i32>,
    ) -> Result<Vec<crate::server::PlayerNetworkEffect>, DaoError> {
        let effect = MessengerFriendRefreshExecutor::refresh_friend_list(
            messenger_dao,
            player_dao,
            self.game().player_manager(),
            user_id,
            offline_user_id,
        )?;

        Ok(self
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|session| session.details().id() == user_id)
            .flat_map(|session| MessengerEffectNetworkPlan::plan(&effect, session.connection_id()))
            .collect())
    }
}

fn has_room_login(effects: &[IncomingExecutionEffect]) -> bool {
    effects.iter().any(|effect| {
        matches!(
            effect,
            IncomingExecutionEffect::Password(crate::game::player::PasswordAction::VerifyLogin {
                room_login: true,
                ..
            })
        )
    })
}

fn write_response(connection_id: i32, packet: String) -> crate::server::PlayerNetworkEffect {
    crate::server::PlayerNetworkEffect::WriteResponse {
        connection_id,
        packet,
    }
}

fn private_room_wallpaper(wall: &str) -> &str {
    wall.parse::<i32>()
        .ok()
        .filter(|value| *value > 0)
        .map(|_| wall)
        .unwrap_or("201")
}

fn private_room_floor(floor: &str) -> &str {
    floor
        .parse::<i32>()
        .ok()
        .filter(|value| *value > 0)
        .map(|_| floor)
        .unwrap_or("0")
}

fn room_user_from_session(session: &PlayerSession, room_id: i32) -> RoomUser {
    if let Some(room_user) = session.room_user() {
        return room_user.clone();
    }

    let details = session.details();
    let pool_figure = (!details.pool_figure().is_empty()).then(|| details.pool_figure().to_owned());
    let mut user = RoomUser::new(
        details.id(),
        details.username(),
        details.figure(),
        details.mission(),
        pool_figure,
    );
    user.set_room_id(room_id);
    user
}

fn moderation_room_context_for_port(
    application: &RoseauApplicationRuntime,
    server_port: i32,
    base_port: i32,
) -> ModerationRoomContext<'_> {
    application
        .game()
        .room_manager()
        .get_room_by_port(server_port, base_port)
        .map(|room| {
            ModerationRoomContext::new(
                room.data().id(),
                room.data().name(),
                room.data().room_type(),
            )
        })
        .unwrap_or_else(|| ModerationRoomContext::new(0, "", RoomType::Private))
}

fn is_item_command_effect(effect: &IncomingExecutionEffect) -> bool {
    matches!(
        effect,
        IncomingExecutionEffect::SetItemData { .. }
            | IncomingExecutionEffect::SetStuffData { .. }
            | IncomingExecutionEffect::UseStripItem { .. }
            | IncomingExecutionEffect::RemoveItem { .. }
            | IncomingExecutionEffect::ReturnItemToInventory { .. }
            | IncomingExecutionEffect::PlaceWallItemFromInventory { .. }
            | IncomingExecutionEffect::PlaceFloorItemFromInventory { .. }
            | IncomingExecutionEffect::MoveStuff { .. }
    )
}

fn is_room_control_effect(effect: &IncomingExecutionEffect) -> bool {
    matches!(
        effect,
        IncomingExecutionEffect::AssignRights { .. }
            | IncomingExecutionEffect::RemoveRights { .. }
            | IncomingExecutionEffect::KickUser { .. }
            | IncomingExecutionEffect::LetUserIn { .. }
    )
}
