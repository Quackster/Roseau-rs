use crate::dao::mysql::{RoomQueries, SqlExecutionPlan};
use crate::dao::{CreateRoom, RoomChatlog};
use crate::game::player::PlayerDetails;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomCommandQueries;

impl RoomCommandQueries {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        owner: &PlayerDetails,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::CreateFlat {
                room_name,
                room_model,
                state,
                show_owner_name,
                ..
            } => Some(
                RoomQueries::create_room(&CreateRoom::new(
                    owner,
                    room_name,
                    "",
                    room_model,
                    *state,
                    *show_owner_name,
                ))
                .insert_returning_id_plan(),
            ),
            IncomingExecutionEffect::DeleteFlat { room_id } => {
                Some(RoomQueries::delete_room(*room_id).execute_plan())
            }
            IncomingExecutionEffect::UpdateFlat {
                room_id,
                room_name,
                state,
                show_owner_name,
            } => Some(
                RoomQueries::update_flat(*room_id, room_name, *state, *show_owner_name)
                    .execute_plan(),
            ),
            IncomingExecutionEffect::SetFlatInfo {
                room_id,
                description,
                password,
                all_super_user,
            } => Some(
                RoomQueries::update_flat_info(*room_id, description, password, *all_super_user)
                    .execute_plan(),
            ),
            _ => None,
        }
    }

    pub fn apply_decoration_plan(
        effect: &IncomingExecutionEffect,
        room_id: i32,
        data: &str,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::ApplyDecoration { decoration, .. } => {
                Some(RoomQueries::update_decoration(room_id, decoration, data)?.execute_plan())
            }
            _ => None,
        }
    }

    pub fn read_plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::GetFlatInfo { room_id }
            | IncomingExecutionEffect::TryFlat { room_id, .. } => {
                Some(RoomQueries::room(*room_id).read_plan())
            }
            _ => None,
        }
    }

    pub fn chatlog_plan(
        effect: &IncomingExecutionEffect,
        username: &str,
        room_id: i32,
        now: i64,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::Talk { mode, message }
                if matches!(mode.as_str(), "CHAT" | "SHOUT") =>
            {
                let chatlog = RoomChatlog::new(username, room_id, mode, message);
                Some(RoomQueries::save_chatlog(&chatlog, now).execute_plan())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

    fn owner() -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(7, "alice", "hello", "hd-100");
        details
    }

    #[test]
    fn maps_create_flat_effect_to_room_insert_plan() {
        let plan = RoomCommandQueries::plan(
            &IncomingExecutionEffect::CreateFlat {
                floor: "first floor".to_owned(),
                room_name: "My room".to_owned(),
                room_model: "model_a".to_owned(),
                state: 1,
                show_owner_name: true,
            },
            &owner(),
        )
        .unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::InsertReturningId);
        assert_eq!(
            plan.sql(),
            "INSERT INTO rooms (name, description, owner_id, model, state, show_owner_name, room_type) VALUES (?, ?, ?, ?, ?, ?, ?)"
        );
        assert_eq!(
            plan.parameters(),
            &[
                SqlParameter::Text("My room".to_owned()),
                SqlParameter::Text(String::new()),
                SqlParameter::Integer(7),
                SqlParameter::Text("model_a".to_owned()),
                SqlParameter::Integer(1),
                SqlParameter::Bool(true),
                SqlParameter::Integer(0),
            ]
        );
    }

    #[test]
    fn maps_delete_and_update_flat_effects_to_room_mutation_plans() {
        let delete = RoomCommandQueries::plan(
            &IncomingExecutionEffect::DeleteFlat { room_id: 42 },
            &owner(),
        )
        .unwrap();
        let update = RoomCommandQueries::plan(
            &IncomingExecutionEffect::UpdateFlat {
                room_id: 42,
                room_name: "Renamed".to_owned(),
                state: 2,
                show_owner_name: false,
            },
            &owner(),
        )
        .unwrap();

        assert_eq!(delete.kind(), SqlExecutionKind::Execute);
        assert_eq!(delete.sql(), "DELETE FROM rooms WHERE id = ?");
        assert_eq!(delete.parameters(), &[SqlParameter::Integer(42)]);
        assert_eq!(
            update.sql(),
            "UPDATE rooms SET name = ?, state = ?, show_owner_name = ? WHERE id = ?"
        );
        assert_eq!(
            update.parameters(),
            &[
                SqlParameter::Text("Renamed".to_owned()),
                SqlParameter::Integer(2),
                SqlParameter::Bool(false),
                SqlParameter::Integer(42),
            ]
        );
    }

    #[test]
    fn maps_set_flat_info_effect_to_room_info_update_plan() {
        let plan = RoomCommandQueries::plan(
            &IncomingExecutionEffect::SetFlatInfo {
                room_id: 42,
                description: "new desc".to_owned(),
                password: "open".to_owned(),
                all_super_user: true,
            },
            &owner(),
        )
        .unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::Execute);
        assert_eq!(
            plan.sql(),
            "UPDATE rooms SET description = ?, password = ?, allsuperuser = ? WHERE id = ?"
        );
        assert_eq!(
            plan.parameters(),
            &[
                SqlParameter::Text("new desc".to_owned()),
                SqlParameter::Text("open".to_owned()),
                SqlParameter::Bool(true),
                SqlParameter::Integer(42),
            ]
        );
    }

    #[test]
    fn maps_decoration_effect_to_room_property_update_plan() {
        let wallpaper = RoomCommandQueries::apply_decoration_plan(
            &IncomingExecutionEffect::ApplyDecoration {
                decoration: "wallpaper".to_owned(),
                item_id: 77,
            },
            42,
            "101",
        )
        .unwrap();
        let floor = RoomCommandQueries::apply_decoration_plan(
            &IncomingExecutionEffect::ApplyDecoration {
                decoration: "floor".to_owned(),
                item_id: 77,
            },
            42,
            "201",
        )
        .unwrap();

        assert_eq!(wallpaper.kind(), SqlExecutionKind::Execute);
        assert_eq!(
            wallpaper.sql(),
            "UPDATE rooms SET wallpaper = ? WHERE id = ?"
        );
        assert_eq!(
            wallpaper.parameters(),
            &[
                SqlParameter::Text("101".to_owned()),
                SqlParameter::Integer(42)
            ]
        );
        assert_eq!(floor.sql(), "UPDATE rooms SET floor = ? WHERE id = ?");
        assert_eq!(
            RoomCommandQueries::apply_decoration_plan(
                &IncomingExecutionEffect::ApplyDecoration {
                    decoration: "ceiling".to_owned(),
                    item_id: 77,
                },
                42,
                "301",
            ),
            None
        );
    }

    #[test]
    fn ignores_non_room_mutation_effects() {
        assert_eq!(
            RoomCommandQueries::plan(&IncomingExecutionEffect::GoToFlat, &owner()),
            None
        );
    }

    #[test]
    fn maps_room_lookup_effects_to_room_read_plans() {
        let flat_info =
            RoomCommandQueries::read_plan(&IncomingExecutionEffect::GetFlatInfo { room_id: 42 })
                .unwrap();
        let try_flat = RoomCommandQueries::read_plan(&IncomingExecutionEffect::TryFlat {
            room_id: 84,
            password: "secret".to_owned(),
        })
        .unwrap();

        assert_eq!(flat_info.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(flat_info.sql(), "SELECT * FROM rooms WHERE id = ? LIMIT 1");
        assert_eq!(flat_info.parameters(), &[SqlParameter::Integer(42)]);
        assert_eq!(try_flat.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(try_flat.sql(), "SELECT * FROM rooms WHERE id = ? LIMIT 1");
        assert_eq!(try_flat.parameters(), &[SqlParameter::Integer(84)]);
    }

    #[test]
    fn ignores_runtime_only_room_read_effects() {
        assert_eq!(
            RoomCommandQueries::read_plan(&IncomingExecutionEffect::GetUnitUsers {
                room_name: "pool".to_owned(),
            }),
            None
        );
        assert_eq!(
            RoomCommandQueries::read_plan(&IncomingExecutionEffect::GoToFlat),
            None
        );
    }

    #[test]
    fn maps_chat_and_shout_effects_to_chatlog_insert_plans() {
        let chat = RoomCommandQueries::chatlog_plan(
            &IncomingExecutionEffect::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            },
            "alice",
            42,
            1234,
        )
        .unwrap();
        let shout = RoomCommandQueries::chatlog_plan(
            &IncomingExecutionEffect::Talk {
                mode: "SHOUT".to_owned(),
                message: "louder".to_owned(),
            },
            "alice",
            42,
            1234,
        )
        .unwrap();

        assert_eq!(chat.kind(), SqlExecutionKind::Execute);
        assert_eq!(
            chat.sql(),
            "INSERT INTO room_chatlogs (user, room_id, timestamp, message_type, message) VALUES (?, ?, ?, ?, ?)"
        );
        assert_eq!(
            chat.parameters(),
            &[
                SqlParameter::Text("alice".to_owned()),
                SqlParameter::Integer(42),
                SqlParameter::Long(1234),
                SqlParameter::Integer(0),
                SqlParameter::Text("hello".to_owned()),
            ]
        );
        assert_eq!(
            shout.parameters(),
            &[
                SqlParameter::Text("alice".to_owned()),
                SqlParameter::Integer(42),
                SqlParameter::Long(1234),
                SqlParameter::Integer(1),
                SqlParameter::Text("louder".to_owned()),
            ]
        );
    }

    #[test]
    fn ignores_non_chatlog_talk_effects() {
        assert_eq!(
            RoomCommandQueries::chatlog_plan(
                &IncomingExecutionEffect::Talk {
                    mode: "WHISPER".to_owned(),
                    message: "alice secret".to_owned(),
                },
                "bob",
                42,
                1234,
            ),
            None
        );
        assert_eq!(
            RoomCommandQueries::chatlog_plan(&IncomingExecutionEffect::GoAway, "bob", 42, 1234),
            None
        );
    }
}
