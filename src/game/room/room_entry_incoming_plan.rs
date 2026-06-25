use crate::game::player::PlayerDetails;
use crate::game::room::{Room, RoomEntryOutcome};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEntryIncomingPlan;

impl RoomEntryIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        room: &Room,
        player: &PlayerDetails,
        room_players: &[PlayerDetails],
        has_room_all_rights: bool,
    ) -> Vec<RoomEntryOutcome> {
        let IncomingExecutionEffect::TryFlat { password, .. } = effect else {
            return Vec::new();
        };

        vec![room.try_flat(player, room_players, password, has_room_all_rights)]
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        room: &Room,
        player: &PlayerDetails,
        room_players: &[PlayerDetails],
        has_room_all_rights: bool,
    ) -> Vec<RoomEntryOutcome> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room, player, room_players, has_room_all_rights))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::settings::{RoomState, RoomType};
    use crate::game::room::{RoomData, RoomEffect};

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn room(state: i32, password: &str) -> Room {
        let mut room = Room::new(RoomData::new(
            12,
            false,
            RoomType::Private,
            7,
            "owner",
            "Room",
            state,
            password,
            25,
            "desc",
            "model_a",
            "class",
            "wall",
            "floor",
            false,
            true,
        ));
        room.load(vec![8]);
        room
    }

    #[test]
    fn plans_password_room_entry_outcome() {
        let visitor = details(9, "visitor");
        let password_room = room(RoomState::Password.state_code(), "secret");

        let rejected = RoomEntryIncomingPlan::plan(
            &IncomingExecutionEffect::TryFlat {
                room_id: 12,
                password: "wrong".to_owned(),
            },
            &password_room,
            &visitor,
            &[],
            false,
        );
        let accepted = RoomEntryIncomingPlan::plan(
            &IncomingExecutionEffect::TryFlat {
                room_id: 12,
                password: "secret".to_owned(),
            },
            &password_room,
            &visitor,
            &[],
            false,
        );

        assert_eq!(rejected, vec![RoomEntryOutcome::IncorrectPassword]);
        assert_eq!(accepted, vec![RoomEntryOutcome::LetIn]);
    }

    #[test]
    fn plans_doorbell_entry_with_rights_recipients() {
        let visitor = details(9, "visitor");
        let owner = details(7, "owner");
        let controller = details(8, "controller");
        let doorbell_room = room(RoomState::Doorbell.state_code(), "");

        let outcomes = RoomEntryIncomingPlan::plan(
            &IncomingExecutionEffect::TryFlat {
                room_id: 12,
                password: String::new(),
            },
            &doorbell_room,
            &visitor,
            &[owner, controller],
            false,
        );

        assert_eq!(
            outcomes,
            vec![RoomEntryOutcome::Doorbell(vec![
                RoomEffect::SendDoorbell {
                    user_id: 7,
                    username: "visitor".to_owned(),
                },
                RoomEffect::SendDoorbell {
                    user_id: 8,
                    username: "visitor".to_owned(),
                },
            ])]
        );
    }

    #[test]
    fn lets_controller_or_all_rights_user_in_without_password() {
        let controller = details(8, "controller");
        let visitor = details(9, "visitor");
        let password_room = room(RoomState::Password.state_code(), "secret");

        let outcomes = RoomEntryIncomingPlan::plan_all(
            &[
                IncomingExecutionEffect::GoAway,
                IncomingExecutionEffect::TryFlat {
                    room_id: 12,
                    password: "wrong".to_owned(),
                },
            ],
            &password_room,
            &controller,
            &[],
            false,
        );
        let all_rights_outcomes = RoomEntryIncomingPlan::plan(
            &IncomingExecutionEffect::TryFlat {
                room_id: 12,
                password: "wrong".to_owned(),
            },
            &password_room,
            &visitor,
            &[],
            true,
        );

        assert_eq!(outcomes, vec![RoomEntryOutcome::LetIn]);
        assert_eq!(all_rights_outcomes, vec![RoomEntryOutcome::LetIn]);
    }

    #[test]
    fn ignores_unrelated_entry_effects() {
        let visitor = details(9, "visitor");
        let open_room = room(RoomState::Open.state_code(), "");

        assert!(RoomEntryIncomingPlan::plan(
            &IncomingExecutionEffect::GoAway,
            &open_room,
            &visitor,
            &[],
            false,
        )
        .is_empty());
    }
}
