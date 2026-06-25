use crate::game::commands::{Command, CommandContext, CommandEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SitCommand;

impl Command for SitCommand {
    fn handle(&self, context: &CommandContext, _message: &str) -> Vec<CommandEffect> {
        let Some(room_user) = context.room_user() else {
            return Vec::new();
        };

        if !room_user.is_in_room()
            || room_user.is_walking()
            || room_user.contains_status("sit")
            || !matches!(room_user.rotation(), 0 | 2 | 4 | 6)
        {
            return Vec::new();
        }

        vec![
            CommandEffect::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
            CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: format!(" {}", room_user.tile_height() as i32),
                infinite: true,
                duration: -1,
            },
            CommandEffect::MarkRoomNeedsUpdate,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::commands::RoomUserCommandState;

    #[test]
    fn records_sit_effects_for_stationary_room_user() {
        let context =
            CommandContext::with_room_user(RoomUserCommandState::new(true, false, 2, 3.7));

        assert_eq!(
            SitCommand.handle(&context, ":sit"),
            vec![
                CommandEffect::RemoveRoomStatus {
                    key: "dance".to_owned()
                },
                CommandEffect::SetRoomStatus {
                    key: "sit".to_owned(),
                    value: " 3".to_owned(),
                    infinite: true,
                    duration: -1,
                },
                CommandEffect::MarkRoomNeedsUpdate,
            ]
        );
    }

    #[test]
    fn ignores_sit_when_room_user_cannot_sit() {
        for room_user in [
            RoomUserCommandState::new(false, false, 2, 3.0),
            RoomUserCommandState::new(true, true, 2, 3.0),
            RoomUserCommandState::new(true, false, 1, 3.0),
            RoomUserCommandState::new(true, false, 2, 3.0).with_status("sit"),
        ] {
            let context = CommandContext::with_room_user(room_user);
            assert!(SitCommand.handle(&context, ":sit").is_empty());
        }
    }
}
