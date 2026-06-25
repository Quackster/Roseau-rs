use crate::game::{GameTickEffect, RoomAfkState};
use crate::runtime::{RoseauApplicationRuntime, RoseauApplicationTickOutcome};
use crate::server::ServerSocketBinder;

impl RoseauApplicationRuntime {
    pub fn run_tick<B: ServerSocketBinder>(
        &mut self,
        binder: &B,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
        main_server_players: impl IntoIterator<Item = (i32, i32)>,
        room_afk_states: &mut [RoomAfkState],
    ) -> RoseauApplicationTickOutcome {
        let raw_config_ip = self
            .startup_runtime()
            .startup_plan()
            .server_plan()
            .raw_config_ip()
            .to_owned();
        let game_effects = if let Some(variables) = self.game().variables().cloned() {
            self.game_mut().scheduler_mut().tick(
                &variables,
                main_server_players,
                room_afk_states,
                &raw_config_ip,
            )
        } else {
            Vec::<GameTickEffect>::new()
        };
        let server_outcome = self.startup_runtime_mut().run_loop_step(
            binder,
            listener_index,
            accept_connection,
            max_bytes,
        );

        RoseauApplicationTickOutcome::new(game_effects, server_outcome)
    }
}
