use crate::server::{PlayerNetwork, PlayerNetworkEffect};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PlayerNetworkEffectExecutor {
    skipped_effects: Vec<PlayerNetworkEffect>,
    packet_logs: Vec<String>,
    log_packets: bool,
}

impl PlayerNetworkEffectExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_log_packets(&mut self, log_packets: bool) {
        self.log_packets = log_packets;
    }

    pub fn apply<N: PlayerNetwork>(&mut self, network: &mut N, effect: PlayerNetworkEffect) {
        if effect.connection_id() != network.connection_id() {
            self.skipped_effects.push(effect);
            return;
        }

        match effect {
            PlayerNetworkEffect::WriteResponse { packet, .. } => {
                if self.log_packets {
                    self.packet_logs.push(sent_packet_log_line(packet.as_str()));
                }
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

    pub fn packet_logs(&self) -> &[String] {
        &self.packet_logs
    }
}

fn sent_packet_log_line(packet: &str) -> String {
    format!("SENT: {}", visible_controls(packet))
}

fn visible_controls(input: &str) -> String {
    (0..14).fold(input.to_owned(), |current, value| {
        let control = char::from_u32(value).unwrap_or_default().to_string();
        current.replace(&control, &format!("[{value}]"))
    })
}

#[cfg(test)]
#[path = "player_network_effect_executor_tests.rs"]
mod tests;
