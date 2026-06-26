use crate::dao::mysql::{MySqlApplicationTickExecutor, SqlExecutor};
use crate::dao::{
    CatalogueDao, DaoError, InventoryDao, ItemDao, MessengerDao, NavigatorDao, PlayerDao, RoomDao,
};
use crate::game::item::interactors::{
    BedInteractor, BlankInteractor, ChairInteractor, Interaction, ItemInteractionEffect,
    ItemInteractionEffectExecutor, ItemInteractionEffectNetworkPlan,
    ItemInteractionEffectRoomExecutor, PoolChangeBoothInteractor, PoolLadderInteractor,
    PoolLiftInteractor, PoolQueueInteractor, TeleporterInteractor,
};
use crate::game::item::Item;
use crate::game::player::PlayerSession;
use crate::game::pathfinder::make_path;
use crate::game::room::entity::RoomUser;
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::schedulers::{
    BotTickState, LidoPlayerState, PoolQueueTile, RoomUserTickState, RoomWalkEntity,
    RoomWalkScheduler, SchedulerEffect, SchedulerEffectExecutor,
};
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomMapping, RoomOccupant, RoomSummary};
use crate::game::RoomAfkState;
use crate::messages::outgoing::{
    ActiveObjects, HeightMap, Logout, ObjectsWorld, ShowProgram, Status, Users,
};
use crate::runtime::errors::RandomSource;
use crate::messages::OutgoingMessage;
use crate::runtime::{
    HostResolver, RoseauApplicationLoopReport, RoseauApplicationRuntime, RoseauConsole,
};
use crate::server::{PlayerNetworkEffect, ServerSocketBinder};
use std::time::Duration;

const APPLICATION_TICK_INTERVAL: Duration = Duration::from_secs(1);
const ROOM_WALK_TICK_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationLoopRunner {
    max_ticks: Option<usize>,
}

impl RoseauApplicationLoopRunner {
    pub fn new() -> Self {
        Self { max_ticks: None }
    }

    pub fn bounded(max_ticks: usize) -> Self {
        Self {
            max_ticks: Some(max_ticks),
        }
    }

    pub fn max_ticks(&self) -> Option<usize> {
        self.max_ticks
    }

    pub fn run<B: ServerSocketBinder, E: SqlExecutor, R: HostResolver>(
        &self,
        application: &mut RoseauApplicationRuntime,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        binder: &B,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationLoopReport, DaoError> {
        self.run_inner(
            application,
            tick_executor,
            resolver,
            binder,
            main_server_players,
            room_afk_states,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn run_with_incoming_daos<B: ServerSocketBinder, E: SqlExecutor, R: HostResolver>(
        &self,
        application: &mut RoseauApplicationRuntime,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        binder: &B,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
        incoming_daos: IncomingDaoSet<'_>,
    ) -> Result<RoseauApplicationLoopReport, DaoError> {
        self.run_inner(
            application,
            tick_executor,
            resolver,
            binder,
            main_server_players,
            room_afk_states,
            Some(incoming_daos),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn run_inner<B: ServerSocketBinder, E: SqlExecutor, R: HostResolver>(
        &self,
        application: &mut RoseauApplicationRuntime,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        binder: &B,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
        incoming_daos: Option<IncomingDaoSet<'_>>,
    ) -> Result<RoseauApplicationLoopReport, DaoError> {
        let mut tick_reports = Vec::new();
        let mut stopped = false;
        let mut console = if self.max_ticks.is_none() {
            RoseauConsole::start()
        } else {
            RoseauConsole::disabled()
        };

        loop {
            if self
                .max_ticks
                .is_some_and(|max_ticks| tick_reports.len() >= max_ticks)
            {
                break;
            }

            let report = application.run_tick_and_apply_runtime_plans(
                tick_executor,
                resolver,
                binder,
                main_server_players.iter().copied(),
                room_afk_states,
            )?;
            Self::remove_disconnected_player_sessions(application, &report);
            if let Some(incoming_daos) = incoming_daos {
                application.apply_pending_incoming_commands_with_catalogue(
                    incoming_daos.player,
                    incoming_daos.room,
                    incoming_daos.catalogue,
                    incoming_daos.inventory,
                    incoming_daos.item,
                    incoming_daos.navigator,
                    incoming_daos.messenger,
                )?;
                Self::remove_server_closed_player_sessions(application);
                Self::apply_room_walk_ticks(application, incoming_daos.room, incoming_daos.item)?;
            } else {
                Self::remove_server_closed_player_sessions(application);
            }
            Self::handle_pending_server_logs(application, &mut console);
            console.tick(application);
            stopped = !report.should_continue();
            tick_reports.push(report);

            if stopped {
                break;
            }

            if self.max_ticks.is_none() {
                if let Some(incoming_daos) = incoming_daos {
                    std::thread::sleep(ROOM_WALK_TICK_INTERVAL);
                    Self::apply_room_walk_ticks(
                        application,
                        incoming_daos.room,
                        incoming_daos.item,
                    )?;
                    Self::handle_pending_server_logs(application, &mut console);
                    console.tick(application);
                    std::thread::sleep(ROOM_WALK_TICK_INTERVAL);
                } else {
                    std::thread::sleep(APPLICATION_TICK_INTERVAL);
                }
            }
        }

        Ok(RoseauApplicationLoopReport::new(tick_reports, stopped))
    }

    fn handle_pending_server_logs(
        application: &mut RoseauApplicationRuntime,
        console: &mut RoseauConsole,
    ) {
        let lines = application.drain_pending_server_log_lines();
        if lines.is_empty() {
            return;
        }

        if console.is_enabled() {
            console.observe_logs(lines);
        } else {
            for line in lines {
                println!("{line}");
            }
        }
    }

    fn apply_room_walk_ticks(
        application: &mut RoseauApplicationRuntime,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
    ) -> Result<(), DaoError> {
        let public_base_port = i32::from(
            application
                .startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let private_server_port = i32::from(
            application
                .startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );
        let rooms = application
            .game()
            .room_manager()
            .loaded_rooms()
            .values()
            .map(|room| {
                (
                    room.data().id(),
                    room.data().model_name().to_owned(),
                    room.data().room_type(),
                    match room.data().room_type() {
                        RoomType::Public => room.data().server_port(public_base_port),
                        RoomType::Private => private_server_port,
                    },
                )
            })
            .collect::<Vec<_>>();

        for (room_id, model_name, room_type, server_port) in rooms {
            // Java: the room schedulers tick every entity in the room,
            // whatever the connection port (main-server players included),
            // so membership is by the RoomUser's room ID. Private rooms
            // additionally require the private-port connection.
            let sessions = application
                .game()
                .player_manager()
                .players()
                .values()
                .filter(|session| {
                    session
                        .room_user()
                        .is_some_and(|room_user| room_user.room_id() == room_id)
                        && (room_type == RoomType::Public || session.server_port() == server_port)
                })
                .filter_map(|session| {
                    session.room_user().map(|room_user| {
                        (
                            session.connection_id(),
                            session.details().id(),
                            !session.details().pool_figure().is_empty(),
                            room_user.clone(),
                        )
                    })
                })
                .collect::<Vec<_>>();

            if sessions.is_empty() {
                continue;
            }

            let Some(mut room_summary) = application
                .game()
                .room_manager()
                .get_room_by_id(room_id)
                .cloned()
            else {
                continue;
            };
            let Some(model) = room_dao.model(&model_name)? else {
                continue;
            };
            let items = match room_type {
                RoomType::Public => item_dao.public_room_items(&model_name, room_id)?,
                RoomType::Private => item_dao.room_items(room_id)?,
            }
            .into_values()
            .collect::<Vec<_>>();

            let mut mapping = RoomMapping::new(model.clone());
            mapping.regenerate_collision_maps(items.clone());

            let room_connection_ids = sessions
                .iter()
                .map(|(connection_id, _, _, _)| *connection_id)
                .collect::<Vec<_>>();
            let player_users: Vec<crate::game::room::entity::RoomUser> = sessions
                .iter()
                .map(|(_, _, _, room_user)| room_user.clone())
                .collect();
            let mut bot_users: Vec<Option<crate::game::room::entity::RoomUser>> = room_summary
                .scheduler()
                .map(|scheduler| scheduler.bots().iter().cloned().map(Some).collect())
                .unwrap_or_default();
            let mut room_users = player_users.clone();
            for bot_user in bot_users.iter().flatten() {
                room_users.push(bot_user.clone());
            }
            let occupants = room_users
                .iter()
                .map(|user| RoomOccupant::new(user.entity_id(), user.position(), user.goal()))
                .collect::<Vec<_>>();
            let mut all_scheduler_effects = Vec::new();

            // Java RoomEventScheduler: the room events tick every 500 ms.
            if let Some(scheduler) = room_summary.scheduler_mut() {
                let mut random = RandomSource::from_clock();
                let user_states: Vec<RoomUserTickState> = room_users
                    .iter()
                    .map(Self::room_user_tick_state)
                    .collect();
                let event_effects = scheduler.user_status_event_mut().tick(&user_states);
                all_scheduler_effects.extend(event_effects);

                let bot_starts: Vec<crate::game::room::model::Position> = scheduler
                    .bot_start_positions()
                    .to_vec();
                let bot_patrols: Vec<Vec<(i32, i32)>> = scheduler
                    .bot_patrol_positions()
                    .iter()
                    .cloned()
                    .collect();
                if let Some(bot_move) = scheduler.bot_move_event_mut() {
                    let starts = &bot_starts;
                    let patrols = &bot_patrols;
                    let bot_states: Vec<BotTickState> = bot_users
                        .iter()
                        .enumerate()
                        .filter_map(|(index, user)| {
                            let user = user.as_ref()?;
                            let start = starts.get(index)?;
                            // Java getNearbyPlayers: players within 3 of the
                            // bot's start position.
                            let nearby_players = player_users
                                .iter()
                                .filter(|player| start.distance(player.position()) <= 3)
                                .count();
                            let patrol_positions = patrols
                                .get(index)
                                .cloned()
                                .unwrap_or_default();
                            Some(
                                BotTickState::new(user.entity_id(), user.position(), *start)
                                    .walking(user.is_walking())
                                    .nearby_player_count(nearby_players)
                                    .patrol_positions(patrol_positions),
                            )
                        })
                        .collect();
                    let patrol_index = random.next_u64() as usize;
                    all_scheduler_effects.extend(bot_move.tick(&bot_states, patrol_index));
                }

                if let Some(lido) = scheduler.lido_event_mut() {
                    let lido_players: Vec<LidoPlayerState> = player_users
                        .iter()
                        .map(|user| {
                            LidoPlayerState::new(
                                user.entity_id(),
                                user.username(),
                                user.position(),
                                user.is_walking(),
                            )
                        })
                        .collect();
                    let pool_queue_tiles: Vec<PoolQueueTile> = items
                        .iter()
                        .filter(|item| item.definition().sprite() == "poolQueue")
                        .filter_map(|item| {
                            let position = item.position();
                            let mut target = position;
                            if let Some(custom_data) = item.custom_data() {
                                if let Ok(parsed) = Position::parse(custom_data) {
                                    target = parsed;
                                }
                            }
                            Some(PoolQueueTile::new(position, target))
                        })
                        .collect();
                    let camera_effect = random.next_i32(3).unwrap_or(0);
                    let target_index = random.next_u64() as usize;
                    all_scheduler_effects.extend(lido.tick(
                        &lido_players,
                        &pool_queue_tiles,
                        camera_effect,
                        target_index,
                    ));
                }

                if let Some(disco) = scheduler.disco_event_mut() {
                    let preferred_lamp_id = random.next_i32(5).unwrap_or(0) + 1;
                    let disco_id = random.next_i32(14).unwrap_or(0) + 1;
                    let include_floor_b = random.next_u64() & 1 == 1;
                    all_scheduler_effects.extend(disco.tick(
                        preferred_lamp_id,
                        disco_id,
                        include_floor_b,
                    ));
                }

                // Apply the per-entity event effects to the player sessions
                // (the same way the walk effects are applied) and to the
                // persisted bot users. WalkTo / SHOWPROGRAM effects are
                // handled below.
                let entity_only = |effect: &SchedulerEffect| {
                    !matches!(
                        effect,
                        SchedulerEffect::WalkTo { .. }
                            | SchedulerEffect::ShowProgram(_)
                            | SchedulerEffect::TargetCamera { .. }
                            | SchedulerEffect::SetCamera(_)
                            | SchedulerEffect::SendStatus(_)
                    )
                };
                for (connection_id, user_id, pool_figure_available, user) in &sessions {
                    let entity_effects: Vec<SchedulerEffect> = all_scheduler_effects
                        .iter()
                        .filter(|effect| effect.entity_id() == Some(user.entity_id()))
                        .filter(|effect| entity_only(*effect))
                        .cloned()
                        .collect();
                    if entity_effects.is_empty() {
                        continue;
                    }
                    if let Some(updated_user) = Self::apply_room_walk_effects(
                        application,
                        *connection_id,
                        &entity_effects,
                        &items,
                        &mut mapping,
                        &occupants,
                        *pool_figure_available,
                        &room_connection_ids,
                    ) {
                        if let Some(room_user) = room_users
                            .iter_mut()
                            .find(|room_user| room_user.entity_id() == *user_id)
                        {
                            *room_user = updated_user;
                        }
                    }
                }
                for bot_user in bot_users.iter_mut().flatten() {
                    let entity_id = bot_user.entity_id();
                    let entity_effects: Vec<SchedulerEffect> = all_scheduler_effects
                        .iter()
                        .filter(|effect| effect.entity_id() == Some(entity_id))
                        .filter(|effect| entity_only(*effect))
                        .cloned()
                        .collect();
                    if entity_effects.is_empty() {
                        continue;
                    }
                    SchedulerEffectExecutor::apply_all(bot_user, &entity_effects);
                    if let Some(snapshot) = room_users
                        .iter_mut()
                        .find(|user| user.entity_id() == entity_id)
                    {
                        *snapshot = bot_user.clone();
                    }
                }
                // WalkTo effects (bot patrol / lido queue) pathfind and start
                // the walk, like Java's `RoomUser.walkTo`.
                Self::apply_walk_to_effects(
                    application,
                    &mut room_users,
                    &sessions,
                    &all_scheduler_effects,
                    &mut mapping,
                    &occupants,
                    &items,
                    &model,
                );
            }

            // Java RoomWalkScheduler: ticks all room entities together so the
            // updated entities share one STATUS packet.
            let all_entities: Vec<RoomWalkEntity> = room_users
                .iter()
                .map(|user| {
                    RoomWalkEntity::new(user.entity_id(), user.position())
                        .walking(user.is_walking())
                        .needs_update(user.needs_update())
                        .with_goal(user.goal())
                        .with_next(user.next())
                        .path(user.path().clone())
                        .current_item_id(user.current_item_id())
                })
                .collect();
            // Java checks `item.canWalk(entity, ...)` per entity; the Rust
            // mapping takes one flag, so any session with a pool figure
            // enables the pool-figure walk check for the whole tick.
            let pool_figure_available = sessions
                .iter()
                .any(|(_, _, pool_figure_available, _)| *pool_figure_available);
            let walk_effects = RoomWalkScheduler::tick(
                &all_entities,
                &mapping,
                &items,
                &occupants,
                pool_figure_available,
            );
            if !walk_effects.is_empty() {
                for (connection_id, user_id, pool_figure_available, user) in &sessions {
                    // Players keep the TriggerCurrentItem effect: Java's
                    // `stopWalking` triggers the current item (sit/lay).
                    // The SendStatus effect (entity ID `None`) is included so
                    // it can clear the needs-update flag.
                    let entity_effects: Vec<SchedulerEffect> = walk_effects
                        .iter()
                        .filter(|effect| {
                            effect.entity_id() == Some(user.entity_id())
                                || matches!(effect, SchedulerEffect::SendStatus(_))
                        })
                        .cloned()
                        .collect();
                    if entity_effects.is_empty() {
                        continue;
                    }

                    if let Some(updated_user) = Self::apply_room_walk_effects(
                        application,
                        *connection_id,
                        &entity_effects,
                        &items,
                        &mut mapping,
                        &occupants,
                        *pool_figure_available,
                        &room_connection_ids,
                    ) {
                        if let Some(room_user) = room_users
                            .iter_mut()
                            .find(|room_user| room_user.entity_id() == *user_id)
                        {
                            *room_user = updated_user;
                        }
                    }

                    if room_type == RoomType::Public
                        && entity_effects
                            .iter()
                            .any(|effect| matches!(effect, SchedulerEffect::StopWalking { .. }))
                    {
                        let current_user = application
                            .game()
                            .player_manager()
                            .players()
                            .get(connection_id)
                            .and_then(|session| session.room_user())
                            .cloned();
                        if let Some(current_user) = current_user {
                            if let Some(connection) = room_dao
                                .room_connections(room_id)?
                                .into_iter()
                                .find(|connection| {
                                    connection.matches_source(
                                        current_user.position().x(),
                                        current_user.position().y(),
                                    )
                                })
                            {
                                let transition_effects = Self::public_room_transition_network_effects(
                                    application,
                                    room_dao,
                                    item_dao,
                                    *connection_id,
                                    room_id,
                                    connection.to_id(),
                                    connection.door_position(),
                                )?;
                                application
                                    .startup_runtime_mut()
                                    .apply_network_effects(transition_effects);
                            }
                        }
                    }
                }

                // Bots: Java `stopWalking` for non-players only marks the
                // update, so skip the player-only TriggerCurrentItem effect.
                // The SendStatus effect (entity ID `None`) is included so it
                // can clear the needs-update flag.
                for bot_user in bot_users.iter_mut().flatten() {
                    let entity_id = bot_user.entity_id();
                    let entity_effects: Vec<SchedulerEffect> = walk_effects
                        .iter()
                        .filter(|effect| {
                            (effect.entity_id() == Some(entity_id)
                                || matches!(effect, SchedulerEffect::SendStatus(_)))
                                && !matches!(effect, SchedulerEffect::TriggerCurrentItem { .. })
                        })
                        .cloned()
                        .collect();
                    if entity_effects.is_empty() {
                        continue;
                    }
                    SchedulerEffectExecutor::apply_all(bot_user, &entity_effects);
                    if let Some(snapshot) = room_users
                        .iter_mut()
                        .find(|user| user.entity_id() == entity_id)
                    {
                        *snapshot = bot_user.clone();
                    }
                }
            }

            // Persist bot state and the event `ticked` counters back to the
            // room manager so the next tick doesn't re-apply effects to
            // stale bot state.
            if let Some(scheduler) = room_summary.scheduler_mut() {
                let updated_bots: Vec<RoomUser> = bot_users
                    .iter()
                    .filter_map(|user| user.as_ref())
                    .cloned()
                    .collect();
                if scheduler.bots().len() == updated_bots.len() {
                    *scheduler.bots_mut() = updated_bots;
                }
            }
            if let Some(scheduler) = room_summary.scheduler().cloned() {
                if let Some(room) = application
                    .game_mut()
                    .room_manager_mut()
                    .get_room_by_id_mut(room_id)
                {
                    room.set_scheduler(Some(scheduler));
                }
            }

            all_scheduler_effects.extend(walk_effects);

            if all_scheduler_effects.is_empty() {
                continue;
            }

            let network_effects = Self::room_walk_network_effects(
                &all_scheduler_effects,
                &room_connection_ids,
                &room_users,
            );
            let show_program_effects = Self::room_show_program_network_effects(
                &all_scheduler_effects,
                &room_connection_ids,
            );
            application
                .startup_runtime_mut()
                .apply_network_effects(network_effects);
            application
                .startup_runtime_mut()
                .apply_network_effects(show_program_effects);
        }

        Ok(())
    }

    fn apply_room_walk_effects(
        application: &mut RoseauApplicationRuntime,
        connection_id: i32,
        scheduler_effects: &[crate::game::room::schedulers::SchedulerEffect],
        items: &[Item],
        mapping: &mut RoomMapping,
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
        room_connection_ids: &[i32],
    ) -> Option<RoomUser> {
        let details = application
            .game()
            .player_manager()
            .players()
            .get(&connection_id)?
            .details()
            .clone();
        let (updated_user, network_effects) = {
            let session = application
                .game_mut()
                .player_manager_mut()
                .get_mut(connection_id)?;
            let user = session.room_user_mut()?;
            SchedulerEffectExecutor::apply_all(user, scheduler_effects);
            let network_effects = if scheduler_effects
                .iter()
                .any(|effect| matches!(effect, SchedulerEffect::TriggerCurrentItem { .. }))
            {
                Self::apply_current_item_trigger(
                    user,
                    items,
                    mapping,
                    occupants,
                    pool_figure_available,
                    details.username(),
                    details.tickets(),
                    connection_id,
                    room_connection_ids,
                )
            } else {
                Vec::new()
            };
            (user.clone(), network_effects)
        };
        if !network_effects.is_empty() {
            application
                .startup_runtime_mut()
                .apply_network_effects(network_effects);
        }
        Some(updated_user)
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_current_item_trigger(
        user: &mut RoomUser,
        items: &[Item],
        mapping: &mut RoomMapping,
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
        username: &str,
        tickets: i32,
        connection_id: i32,
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        let mut network_effects = Vec::new();
        let mut effects = Self::stopped_item_effects(user, items, pool_figure_available);

        for _ in 0..4 {
            network_effects.extend(
                ItemInteractionEffectNetworkPlan::plan_all_for_connection_ids(
                    &effects,
                    connection_id,
                    username,
                    tickets,
                    room_connection_ids,
                    items,
                ),
            );

            let mut room_user_effects = ItemInteractionEffectExecutor::apply_all(user, &effects);
            room_user_effects.extend(ItemInteractionEffectRoomExecutor::apply_all(
                user,
                &effects,
                mapping,
                items,
                occupants,
                pool_figure_available,
                tickets,
            ));
            user.set_needs_update(true);

            if !room_user_effects.iter().any(|effect| {
                matches!(
                    effect,
                    crate::game::room::entity::RoomUserEffect::TriggerCurrentItem { .. }
                )
            }) {
                break;
            }

            effects = Self::stopped_item_effects(user, items, pool_figure_available);
        }

        network_effects
    }

    fn stopped_item_effects(
        user: &mut RoomUser,
        items: &[Item],
        pool_figure_available: bool,
    ) -> Vec<ItemInteractionEffect> {
        let item = items
            .iter()
            .filter(|item| item.has_entity_collision(user.position().x(), user.position().y()))
            .max_by(|left, right| left.total_height().total_cmp(&right.total_height()));

        let Some(item) = item else {
            user.set_current_item_id(None);
            return vec![
                ItemInteractionEffect::RemoveStatus {
                    status: "sit".to_owned(),
                },
                ItemInteractionEffect::RemoveStatus {
                    status: "lay".to_owned(),
                },
            ];
        };
        if !item.can_walk(pool_figure_available) {
            user.set_current_item_id(None);
            return BlankInteractor.on_stopped_walking(item, user.position());
        };

        user.set_current_item_id(Some(item.id()));
        let behaviour = item.definition().behaviour();
        if behaviour.can_sit_on_top() {
            ChairInteractor.on_stopped_walking(item, user.position())
        } else if behaviour.can_lay_on_top() {
            BedInteractor.on_stopped_walking(item, user.position())
        } else {
            match item.definition().sprite() {
                "poolBooth" => PoolChangeBoothInteractor.on_stopped_walking(item, user.position()),
                "poolQueue" => PoolQueueInteractor.on_stopped_walking(item, user.position()),
                "poolLift" => PoolLiftInteractor.on_stopped_walking(item, user.position()),
                "poolEnter" => {
                    PoolLadderInteractor::new(true).on_stopped_walking(item, user.position())
                }
                "poolExit" => {
                    PoolLadderInteractor::new(false).on_stopped_walking(item, user.position())
                }
                _ if behaviour.is_teleporter() => {
                    TeleporterInteractor.on_stopped_walking(item, user.position())
                }
                _ => BlankInteractor.on_stopped_walking(item, user.position()),
            }
        }
    }

    // Java `RoomUser` equivalents: the tick state and the pathfind for
    // scheduler-initiated walks.
    fn room_user_tick_state(user: &RoomUser) -> RoomUserTickState {
        let mut state = RoomUserTickState::new(user.entity_id())
            .walking(user.is_walking())
            .needs_update(user.needs_update())
            .look_reset_time(user.look_reset_time())
            .rotations(user.position().rotation(), user.position().head_rotation())
            .time_until_next_drink(user.time_until_next_drink());

        for status in user.statuses().values() {
            state = state.with_status(status.clone());
        }

        state
    }

    fn apply_walk_to_effects(
        application: &mut RoseauApplicationRuntime,
        room_users: &mut Vec<RoomUser>,
        sessions: &[(i32, i32, bool, RoomUser)],
        scheduler_effects: &[SchedulerEffect],
        mapping: &mut RoomMapping,
        occupants: &[RoomOccupant],
        items: &[Item],
        model: &RoomModel,
    ) {
        let walk_to_targets: Vec<(i32, i32, i32)> = scheduler_effects
            .iter()
            .filter_map(|effect| match effect {
                SchedulerEffect::WalkTo { entity_id, x, y } => {
                    Some((*entity_id, *x, *y))
                }
                _ => None,
            })
            .collect();

        for (entity_id, x, y) in walk_to_targets {
            let is_player = sessions
                .iter()
                .any(|(_, user_id, _, _)| *user_id == entity_id);

            if is_player {
                let Some((connection_id, _, _, _)) =
                    sessions.iter().find(|(_, user_id, _, _)| *user_id == entity_id)
                else {
                    continue;
                };
                let Some(session_state) = application
                    .game_mut()
                    .player_manager_mut()
                    .get_mut(*connection_id)
                else {
                    continue;
                };
                let Some(session_user) = session_state.room_user_mut() else {
                    continue;
                };
                let path = Self::scheduler_walk_path(
                    session_user,
                    x,
                    y,
                    model,
                    occupants,
                    mapping,
                    items,
                );
                if session_user.walk_to(x, y, path) {
                    // Java `walkTo` marks the entity for an update.
                    session_user.set_needs_update(true);
                    if let Some(persisted) = room_users
                        .iter_mut()
                        .find(|user| user.entity_id() == entity_id)
                    {
                        *persisted = session_user.clone();
                    }
                }
            } else if let Some(persisted) = room_users
                .iter_mut()
                .find(|user| user.entity_id() == entity_id)
            {
                let path = Self::scheduler_walk_path(
                    persisted,
                    x,
                    y,
                    model,
                    occupants,
                    mapping,
                    items,
                );
                if persisted.walk_to(x, y, path) {
                    persisted.set_needs_update(true);
                }
            }
        }
    }

    fn scheduler_walk_path(
        user: &RoomUser,
        x: i32,
        y: i32,
        model: &RoomModel,
        occupants: &[RoomOccupant],
        mapping: &RoomMapping,
        items: &[Item],
    ) -> Vec<Position> {
        make_path(
            user.position(),
            Position::new(x, y, 0.0),
            model.map_size_x(),
            model.map_size_y(),
            |_, position, _| {
                mapping.is_valid_tile(user.entity_id(), position.x(), position.y(), items, occupants, false)
            },
        )
    }

    // Java `room.send(new SHOWPROGRAM(...))` broadcasts the camera /
    // lamp / floor programs to every player in the room.
    fn room_show_program_network_effects(
        scheduler_effects: &[SchedulerEffect],
        room_connection_ids: &[i32],
    ) -> Vec<PlayerNetworkEffect> {
        let mut packets = Vec::new();

        for effect in scheduler_effects {
            let parameters: Vec<String> = match effect {
                SchedulerEffect::ShowProgram(parameters) => parameters.clone(),
                SchedulerEffect::TargetCamera { username, .. } => vec![
                    "cam1".to_owned(),
                    "targetcamera".to_owned(),
                    username.clone(),
                ],
                SchedulerEffect::SetCamera(camera_type) => vec![
                    "cam1".to_owned(),
                    "setcamera".to_owned(),
                    camera_type.to_string(),
                ],
                _ => continue,
            };

            let packet = ShowProgram::new(parameters).compose().get();
            for connection_id in room_connection_ids {
                packets.push(PlayerNetworkEffect::WriteResponse {
                    connection_id: *connection_id,
                    packet: packet.clone(),
                });
            }
        }

        packets
    }

    fn room_walk_network_effects(
        scheduler_effects: &[SchedulerEffect],
        room_connection_ids: &[i32],
        room_users: &[RoomUser],
    ) -> Vec<PlayerNetworkEffect> {
        let status_entity_ids = scheduler_effects
            .iter()
            .filter_map(|effect| match effect {
                SchedulerEffect::SendStatus(entity_ids) => Some(entity_ids.as_slice()),
                _ => None,
            })
            .flatten()
            .copied()
            .collect::<Vec<_>>();

        let status_entities = status_entity_ids
            .iter()
            .filter_map(|entity_id| {
                room_users
                    .iter()
                    .find(|user| user.entity_id() == *entity_id)
                    .map(RoomUser::status_entity)
            })
            .collect::<Vec<_>>();

        if status_entities.is_empty() {
            return Vec::new();
        }

        let packet = Status::new(status_entities).compose().get();
        room_connection_ids
            .iter()
            .map(|connection_id| PlayerNetworkEffect::WriteResponse {
                connection_id: *connection_id,
                packet: packet.clone(),
            })
            .collect()
    }

    fn public_room_transition_network_effects(
        application: &mut RoseauApplicationRuntime,
        room_dao: &dyn RoomDao,
        item_dao: &dyn ItemDao,
        connection_id: i32,
        from_room_id: i32,
        to_room_id: i32,
        door_position: crate::game::room::model::Position,
    ) -> Result<Vec<PlayerNetworkEffect>, DaoError> {
        if application
            .game()
            .room_manager()
            .get_room_by_id(to_room_id)
            .is_none()
        {
            if let Some(room_data) = room_dao.room(to_room_id, true)? {
                application
                    .game_mut()
                    .room_manager_mut()
                    .add(RoomSummary::new(room_data));
            }
        }

        let Some(room_data) = application
            .game()
            .room_manager()
            .get_room_by_id(to_room_id)
            .map(|room| room.data().clone())
        else {
            return Ok(Vec::new());
        };
        if room_data.room_type() != RoomType::Public {
            return Ok(Vec::new());
        }

        let Some(session) = application
            .game()
            .player_manager()
            .players()
            .get(&connection_id)
            .cloned()
        else {
            return Ok(Vec::new());
        };
        let base_port = i32::from(
            application
                .startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let target_port = room_data.server_port(base_port);

        let mut room_items = item_dao
            .room_items(room_data.id())?
            .into_values()
            .collect::<Vec<_>>();
        room_items.sort_by_key(Item::id);
        let mut passive_objects = item_dao
            .public_room_items(room_data.model_name(), room_data.id())?
            .into_values()
            .collect::<Vec<_>>();
        passive_objects.sort_by_key(Item::id);

        let existing_room_users = application
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|other| other.connection_id() != connection_id)
            .filter(|other| {
                other
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == room_data.id())
            })
            .map(|session| room_user_from_session(session, room_data.id()))
            .collect::<Vec<_>>();

        let mut packets = Vec::new();
        let room_model = room_dao.model(room_data.model_name())?;
        // Java `USERS.write()`: the pool figure is appended for `PLAYER`
        // entities only when `room.getData().getModel().hasPool()`.
        let has_pool = room_model.as_ref().is_some_and(|model| model.has_pool());
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
        packets.push(
            Users::new(existing_room_users.iter().map(|user| user.user_entry()))
                .with_has_pool(has_pool)
                .compose()
                .get(),
        );
        packets.push(
            Status::new(existing_room_users.iter().map(|user| user.status_entity()))
                .compose()
                .get(),
        );

        let mut current_user = room_user_from_session(&session, room_data.id());
        current_user.set_room_id(room_data.id());
        current_user.set_position(door_position);
        current_user.force_stop_walking();
        // Java reads `entity.getDetails().getPoolFigure()` at compose time, so
        // refresh the snapshot from the player's details.
        current_user.set_pool_figure(
            (!session.details().pool_figure().is_empty())
                .then(|| session.details().pool_figure().to_owned()),
        );
        packets.push(
            Users::new([current_user.user_entry()])
                .with_has_pool(has_pool)
                .compose()
                .get(),
        );
        packets.push(Status::new([current_user.status_entity()]).compose().get());

        if let Some(session) = application
            .game_mut()
            .player_manager_mut()
            .get_mut(connection_id)
        {
            session.set_server_port(target_port);
            session.set_room_user(current_user);
        }
        application
            .startup_runtime_mut()
            .update_connection_context(connection_id, |context| {
                context.set_main_server_connection(false);
                context.set_in_room(true);
                context.set_room_model_name(room_data.model_name());
                context.set_current_room_name(room_data.name());
            });

        let mut network_effects = application
            .game()
            .player_manager()
            .players()
            .values()
            .filter(|other| other.connection_id() != connection_id)
            .filter(|other| {
                other
                    .room_user()
                    .is_some_and(|room_user| room_user.room_id() == from_room_id)
            })
            .map(|other| PlayerNetworkEffect::WriteResponse {
                connection_id: other.connection_id(),
                packet: Logout::new(session.details().username()).compose().get(),
            })
            .collect::<Vec<_>>();
        network_effects.extend(packets.into_iter().map(|packet| {
            PlayerNetworkEffect::WriteResponse {
                connection_id,
                packet,
            }
        }));
        Ok(network_effects)
    }

    fn remove_disconnected_player_sessions(
        application: &mut RoseauApplicationRuntime,
        report: &crate::runtime::RoseauApplicationTickRunReport,
    ) {
        let Some(tick) = report.server_outcome().tick() else {
            return;
        };

        for connection_id in tick.removed_connection_ids() {
            application
                .game_mut()
                .player_manager_mut()
                .remove(*connection_id);
        }
    }

    // Connections closed by the server itself (e.g. a GOTOFLAT kick) are
    // removed from the runtime immediately, before the next tick's
    // `removed_connection_ids`, so their player sessions are removed here.
    fn remove_server_closed_player_sessions(application: &mut RoseauApplicationRuntime) {
        let closed_ids = application
            .startup_runtime_mut()
            .tcp_runtime_mut()
            .map(|runtime| runtime.drain_server_closed_connection_ids())
            .unwrap_or_default();

        for connection_id in closed_ids {
            application
                .game_mut()
                .player_manager_mut()
                .remove(connection_id);
        }
    }
}

#[derive(Clone, Copy)]
pub struct IncomingDaoSet<'a> {
    player: &'a dyn PlayerDao,
    room: &'a dyn RoomDao,
    catalogue: &'a dyn CatalogueDao,
    inventory: &'a dyn InventoryDao,
    item: &'a dyn ItemDao,
    navigator: &'a dyn NavigatorDao,
    messenger: &'a dyn MessengerDao,
}

impl<'a> IncomingDaoSet<'a> {
    pub fn new(
        player: &'a dyn PlayerDao,
        room: &'a dyn RoomDao,
        catalogue: &'a dyn CatalogueDao,
        inventory: &'a dyn InventoryDao,
        item: &'a dyn ItemDao,
        navigator: &'a dyn NavigatorDao,
        messenger: &'a dyn MessengerDao,
    ) -> Self {
        Self {
            player,
            room,
            catalogue,
            inventory,
            item,
            navigator,
            messenger,
        }
    }
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

#[cfg(test)]
#[path = "roseau_application_loop_runner_tests.rs"]
mod tests;
