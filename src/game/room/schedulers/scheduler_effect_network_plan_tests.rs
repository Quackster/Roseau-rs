use super::scheduler_effect_network_plan::*;
use crate::game::player::PlayerDetails;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn manager() -> PlayerManager {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(70, 30000, details(7, "Alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "Bob")));
    manager
}

fn room_user(entity_id: i32, name: &str) -> RoomUser {
    let mut user = RoomUser::new(entity_id, name, "hd-100", "hello", None::<String>);
    user.set_room_id(42);
    user
}

#[test]
fn broadcasts_batched_status_for_matching_room_users() {
    let mut alice = room_user(7, "Alice");
    alice.set_status("mv", " 1,2,0", true, -1);
    let bob = room_user(8, "Bob");

    let effects = SchedulerEffectNetworkPlan::plan(
        &SchedulerEffect::SendStatus(vec![7, 8, 9]),
        &[7, 8],
        &[alice, bob],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#STATUS \rAlice 0,0,0,0,0/mv 1,2,0/\rBob 0,0,0,0,0/##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#STATUS \rAlice 0,0,0,0,0/mv 1,2,0/\rBob 0,0,0,0,0/##".to_owned(),
            },
        ]
    );
}

#[test]
fn broadcasts_show_program_and_lido_camera_effects() {
    let effects = SchedulerEffectNetworkPlan::plan_all(
        &[
            SchedulerEffect::ShowProgram(vec![
                "lamp".to_owned(),
                "setlamp".to_owned(),
                "2".to_owned(),
            ]),
            SchedulerEffect::TargetCamera {
                player_id: 7,
                username: "Alice".to_owned(),
            },
            SchedulerEffect::SetCamera(1),
        ],
        &[7],
        &[],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOWPROGRAM\rlamp setlamp 2##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOWPROGRAM\rcam1 targetcamera Alice##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOWPROGRAM\rcam1 setcamera 1##".to_owned(),
            },
        ]
    );
}

#[test]
fn ignores_non_network_scheduler_effects() {
    let effects = SchedulerEffectNetworkPlan::plan(
        &SchedulerEffect::MarkNeedsUpdate { entity_id: 7 },
        &[7],
        &[room_user(7, "Alice")],
        &manager(),
    );

    assert!(effects.is_empty());
}
