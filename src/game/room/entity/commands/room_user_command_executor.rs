use crate::game::commands::CommandEffect;
use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::{calculate_direction, Position};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUserCommandExecutor;

impl RoomUserCommandExecutor {
    pub fn apply(user: &mut RoomUser, effect: &IncomingExecutionEffect) -> Vec<RoomUserEffect> {
        match effect {
            IncomingExecutionEffect::SetRoomStatus {
                key,
                value,
                visible,
                timeout,
            } => {
                user.set_status(key, value, *visible, i64::from(*timeout));
                Vec::new()
            }
            IncomingExecutionEffect::RemoveRoomStatus { key } => {
                user.remove_status(key);
                Vec::new()
            }
            IncomingExecutionEffect::MarkRoomNeedsUpdate => {
                user.set_needs_update(true);
                Vec::new()
            }
            IncomingExecutionEffect::ResetAfkTimer => {
                user.reset_afk_timer();
                Vec::new()
            }
            IncomingExecutionEffect::LookTo { x, y } => {
                Self::look_to(user, *x, *y);
                Vec::new()
            }
            IncomingExecutionEffect::Talk { mode, message } => vec![user.chat(mode, message)],
            IncomingExecutionEffect::Command(command) => Self::apply_command(user, command),
            _ => Vec::new(),
        }
    }

    pub fn apply_all(
        user: &mut RoomUser,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<RoomUserEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(user, effect))
            .collect()
    }

    fn apply_command(user: &mut RoomUser, command: &CommandEffect) -> Vec<RoomUserEffect> {
        match command {
            CommandEffect::RemoveRoomStatus { key } => {
                user.remove_status(key);
                Vec::new()
            }
            CommandEffect::SetRoomStatus {
                key,
                value,
                infinite,
                duration,
            } => {
                user.set_status(key, value, *infinite, *duration);
                Vec::new()
            }
            CommandEffect::MarkRoomNeedsUpdate => {
                user.set_needs_update(true);
                Vec::new()
            }
            CommandEffect::SendAlert(_) | CommandEffect::ReloadItemDefinitions => Vec::new(),
        }
    }

    fn look_to(user: &mut RoomUser, x: i32, y: i32) {
        if user.contains_status("lay")
            || user.contains_status("sit")
            || user.position().is_match(Position::new(x, y, 0.0))
            || user.is_walking()
        {
            return;
        }

        let rotation = calculate_direction(user.position().x(), user.position().y(), x, y) as i32;

        if rotation != user.position().rotation() {
            let mut position = user.position();
            position.set_rotation(rotation);
            user.set_position(position);
            user.set_needs_update(true);
        }
    }
}
