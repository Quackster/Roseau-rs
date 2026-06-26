use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::messages::outgoing::{EncryptionOn, SecretKey};
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

const V1_SECRET_KEY_LENGTH: usize = 62;
const V1_SECRET_KEY_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
static V1_SECRET_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionCheck;

impl IncomingEvent for VersionCheck {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        let secret_key = generate_v1_secret_key();
        context.set_rc4_secret_key(secret_key.clone());
        context.send(EncryptionOn.compose());
        context.send(SecretKey::new(secret_key).compose());
    }
}

fn generate_v1_secret_key() -> String {
    let mut bytes = [0u8; V1_SECRET_KEY_LENGTH];

    if std::fs::File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .is_err()
    {
        fill_fallback_random_bytes(&mut bytes);
    }

    bytes
        .into_iter()
        .map(|byte| {
            V1_SECRET_KEY_ALPHABET[usize::from(byte) % V1_SECRET_KEY_ALPHABET.len()] as char
        })
        .collect()
}

fn fill_fallback_random_bytes(bytes: &mut [u8]) {
    let counter = V1_SECRET_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0);
    let mut state = nanos ^ counter.rotate_left(17) ^ u64::from(std::process::id());

    for byte in bytes {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }
}

#[cfg(test)]
#[path = "version_check_tests.rs"]
mod tests;
