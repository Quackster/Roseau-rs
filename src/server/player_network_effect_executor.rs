use crate::server::{PlayerNetwork, PlayerNetworkEffect};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PlayerNetworkEffectExecutor {
    skipped_effects: Vec<PlayerNetworkEffect>,
}

impl PlayerNetworkEffectExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply<N: PlayerNetwork>(&mut self, network: &mut N, effect: PlayerNetworkEffect) {
        if effect.connection_id() != network.connection_id() {
            self.skipped_effects.push(effect);
            return;
        }

        match effect {
            PlayerNetworkEffect::WriteResponse { packet, .. } => {
                network.send_packet(&packet);
            }
            PlayerNetworkEffect::CloseConnection { .. } => {
                network.close();
            }
        }
    }

    pub fn apply_all<N: PlayerNetwork>(
        &mut self,
        network: &mut N,
        effects: impl IntoIterator<Item = PlayerNetworkEffect>,
    ) {
        for effect in effects {
            self.apply(network, effect);
        }
    }

    pub fn skipped_effects(&self) -> &[PlayerNetworkEffect] {
        &self.skipped_effects
    }
}

#[cfg(test)]
#[path = "player_network_effect_executor_tests.rs"]
mod tests;
