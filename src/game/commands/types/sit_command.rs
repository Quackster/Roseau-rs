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
