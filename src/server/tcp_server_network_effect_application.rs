use crate::server::{PlayerNetworkEffect, TcpServerRuntime};

impl TcpServerRuntime {
    pub fn apply_network_effect(&mut self, effect: PlayerNetworkEffect) -> bool {
        if let Some(connection) = self
            .connections
            .iter_mut()
            .find(|connection| connection.connection_id() == effect.connection_id())
        {
            connection.apply_network_effect(effect);
            return true;
        }

        false
    }

    pub fn apply_network_effects(
        &mut self,
        effects: impl IntoIterator<Item = PlayerNetworkEffect>,
    ) -> Vec<PlayerNetworkEffect> {
        let mut unapplied = Vec::new();

        for effect in effects {
            if !self.apply_network_effect(effect.clone()) {
                unapplied.push(effect);
            }
        }

        unapplied
    }
}
