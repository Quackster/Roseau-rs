use crate::server::{PlayerNetworkEffect, TcpServerRuntime};

impl TcpServerRuntime {
    pub fn apply_network_effect(&mut self, effect: PlayerNetworkEffect) -> bool {
        let Some(index) = self
            .connections
            .iter()
            .position(|connection| connection.connection_id() == effect.connection_id())
        else {
            return false;
        };

        match effect {
            PlayerNetworkEffect::WriteResponse { .. } => {
                self.connections[index].apply_network_effect(effect);
            }
            PlayerNetworkEffect::CloseConnection { .. } => {
                let _ = self.close_connection(index);
                let _ = self.remove_connection(index);
            }
        }

        true
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
