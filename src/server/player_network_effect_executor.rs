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
mod tests {
    use super::*;
    use crate::server::RecordedPlayerNetwork;

    #[test]
    fn writes_and_closes_matching_network_effects() {
        let mut network = RecordedPlayerNetwork::new(7, 37120);
        let mut executor = PlayerNetworkEffectExecutor::new();

        executor.apply_all(
            &mut network,
            [
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 7,
                    packet: "#HELLO##".to_owned(),
                },
                PlayerNetworkEffect::CloseConnection { connection_id: 7 },
            ],
        );

        assert_eq!(network.sent_packets(), &["#HELLO##"]);
        assert!(network.is_closed());
        assert!(executor.skipped_effects().is_empty());
    }

    #[test]
    fn skips_effects_for_other_connections() {
        let mut network = RecordedPlayerNetwork::new(7, 37120);
        let mut executor = PlayerNetworkEffectExecutor::new();

        executor.apply(
            &mut network,
            PlayerNetworkEffect::WriteResponse {
                connection_id: 8,
                packet: "#HELLO##".to_owned(),
            },
        );

        assert!(network.sent_packets().is_empty());
        assert_eq!(
            executor.skipped_effects(),
            &[PlayerNetworkEffect::WriteResponse {
                connection_id: 8,
                packet: "#HELLO##".to_owned(),
            }]
        );
    }
}
