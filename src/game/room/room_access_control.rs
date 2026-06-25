use crate::game::player::{PlayerDetails, PlayerManager};
use crate::game::room::settings::RoomState;
use crate::game::room::{Room, RoomEffect, RoomEntryOutcome};

impl Room {
    pub fn ring_doorbell(
        &self,
        visitor: &PlayerDetails,
        players: &[PlayerDetails],
    ) -> Vec<RoomEffect> {
        players
            .iter()
            .filter(|player| self.has_rights(player, false, false))
            .map(|player| RoomEffect::SendDoorbell {
                user_id: player.id(),
                username: visitor.username().to_owned(),
            })
            .collect()
    }

    pub fn try_flat(
        &self,
        player: &PlayerDetails,
        players: &[PlayerDetails],
        password: &str,
        has_room_all_rights: bool,
    ) -> RoomEntryOutcome {
        if self.has_rights(player, has_room_all_rights, false) {
            return RoomEntryOutcome::LetIn;
        }

        if self.data.state() == RoomState::Password && password != self.data.password() {
            return RoomEntryOutcome::IncorrectPassword;
        }

        if self.data.state() == RoomState::Doorbell {
            let effects = self.ring_doorbell(player, players);
            return if effects.is_empty() {
                RoomEntryOutcome::IncorrectPassword
            } else {
                RoomEntryOutcome::Doorbell(effects)
            };
        }

        RoomEntryOutcome::LetIn
    }

    pub fn let_user_in(
        &self,
        player: &PlayerDetails,
        target: Option<&PlayerDetails>,
        has_room_all_rights: bool,
    ) -> Vec<RoomEffect> {
        if !self.has_rights(player, has_room_all_rights, false) {
            return Vec::new();
        }

        let Some(target) = target else {
            return Vec::new();
        };

        vec![RoomEffect::LetUserIn {
            user_id: target.id(),
            room_id: self.data.id(),
        }]
    }

    pub fn kick_user(
        &self,
        player_manager: &PlayerManager,
        sender: &PlayerDetails,
        target: Option<&PlayerDetails>,
        has_room_all_rights: bool,
    ) -> Vec<RoomEffect> {
        if !self.has_rights(sender, has_room_all_rights, false) {
            return Vec::new();
        }

        let Some(target) = target else {
            return Vec::new();
        };

        if player_manager.has_permission(target.rank(), "room_kick_any_user")
            && !player_manager.has_permission(sender.rank(), "room_kick_any_user")
        {
            return Vec::new();
        }

        vec![RoomEffect::KickUser {
            user_id: target.id(),
        }]
    }

    pub fn has_rights(
        &self,
        player: &PlayerDetails,
        has_room_all_rights: bool,
        owner_check_only: bool,
    ) -> bool {
        has_room_all_rights
            || self.data.owner_id() == player.id()
            || (!owner_check_only && self.rights.contains(&player.id()))
    }

    pub fn give_user_rights(&mut self, player: &PlayerDetails) -> Vec<RoomEffect> {
        if self.rights.contains(&player.id()) {
            return Vec::new();
        }

        self.rights.push(player.id());
        vec![
            RoomEffect::SendControllerPrivileges {
                user_id: player.id(),
            },
            RoomEffect::SetRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
                value: String::new(),
            },
            RoomEffect::MarkRoomUserForUpdate {
                user_id: player.id(),
            },
            RoomEffect::SaveRights {
                room_id: self.data.id(),
                rights: self.rights.clone(),
            },
        ]
    }

    pub fn assign_user_rights(
        &mut self,
        sender: &PlayerDetails,
        target: Option<&PlayerDetails>,
        has_room_all_rights: bool,
    ) -> Vec<RoomEffect> {
        if !self.has_rights(sender, has_room_all_rights, true) {
            return Vec::new();
        }

        let Some(target) = target else {
            return Vec::new();
        };

        self.give_user_rights(target)
    }

    pub fn remove_user_rights(&mut self, player: &PlayerDetails) -> Vec<RoomEffect> {
        if !self.rights.contains(&player.id()) {
            return Vec::new();
        }

        self.rights.retain(|id| *id != player.id());
        vec![
            RoomEffect::SendNoControllerPrivileges {
                user_id: player.id(),
            },
            RoomEffect::RemoveRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
            },
            RoomEffect::RemoveRoomUserStatus {
                user_id: player.id(),
                key: "mod".to_owned(),
            },
            RoomEffect::MarkRoomUserForUpdate {
                user_id: player.id(),
            },
            RoomEffect::SaveRights {
                room_id: self.data.id(),
                rights: self.rights.clone(),
            },
        ]
    }

    pub fn revoke_user_rights(
        &mut self,
        sender: &PlayerDetails,
        target: Option<&PlayerDetails>,
        has_room_all_rights: bool,
    ) -> Vec<RoomEffect> {
        if !self.has_rights(sender, has_room_all_rights, true) {
            return Vec::new();
        }

        let Some(target) = target else {
            return Vec::new();
        };

        self.remove_user_rights(target)
    }

    pub fn refresh_flat_privileges(
        &self,
        player: &PlayerDetails,
        has_room_all_rights: bool,
        enter_room: bool,
    ) -> Vec<RoomEffect> {
        let mut effects = Vec::new();
        effects.extend(rank_status_effect(player));

        if self.data.owner_id() == player.id() || has_room_all_rights {
            effects.push(RoomEffect::SendOwnerPrivileges {
                user_id: player.id(),
            });
            effects.push(RoomEffect::SetRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
                value: " useradmin".to_owned(),
            });
        } else if self.has_rights(player, false, false) || self.data.has_all_super_user() {
            effects.push(RoomEffect::SendControllerPrivileges {
                user_id: player.id(),
            });
            effects.push(RoomEffect::SetRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
                value: String::new(),
            });
        } else {
            effects.push(RoomEffect::RemoveRoomUserStatus {
                user_id: player.id(),
                key: "flatctrl".to_owned(),
            });
            effects.push(RoomEffect::RemoveRoomUserStatus {
                user_id: player.id(),
                key: "mod".to_owned(),
            });
            effects.push(RoomEffect::SendNoControllerPrivileges {
                user_id: player.id(),
            });
        }

        if !enter_room {
            effects.push(RoomEffect::MarkRoomUserForUpdate {
                user_id: player.id(),
            });
        }

        effects
    }
}

fn rank_status_effect(player: &PlayerDetails) -> Option<RoomEffect> {
    let value = match player.rank() {
        2 => " 1",
        3 => " 2",
        4 => " 3",
        5 => " A",
        _ => return None,
    };

    Some(RoomEffect::SetRoomUserStatus {
        user_id: player.id(),
        key: "mod".to_owned(),
        value: value.to_owned(),
    })
}
