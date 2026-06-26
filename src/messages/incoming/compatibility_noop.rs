use crate::messages::{IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompatibilityNoop;

impl IncomingEvent for CompatibilityNoop {
    fn handle(&self, _context: &mut IncomingContext, _request: &dyn ClientMessage) {}
}

#[cfg(test)]
#[path = "compatibility_noop_tests.rs"]
mod tests;
