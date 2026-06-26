use crate::game::player::PlayerNameApproval;
use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Register;

impl IncomingEvent for Register {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(name) = split_field_value(request, 0) else {
            return;
        };
        let Some(password) = split_field_value(request, 1) else {
            return;
        };
        let Some(mut email) = split_field_value(request, 2) else {
            return;
        };
        let Some(figure) = prefixed_field_value(request, 3, "figure=") else {
            return;
        };
        let Some(birthday) = split_field_value(request, 5) else {
            return;
        };
        let Some(mut mission) = prefixed_field_value(request, 7, "customData=") else {
            return;
        };
        let Some(sex) = split_field_value(request, 9) else {
            return;
        };

        mission = filter_input(&mission);

        if name.chars().count() < 3 || password.chars().count() < 3 || figure.chars().count() < 3 {
            return;
        }

        if name.chars().count() > 50 {
            return;
        }

        if email.chars().count() > 256 {
            email = truncate_chars(&email, 256);
        }

        if mission.chars().count() > 100 {
            mission = truncate_chars(&mission, 100);
        }

        if !(4..=6).contains(&sex.chars().count()) {
            return;
        }

        if !PlayerNameApproval::evaluate(&name, context.username_chars_value()).is_approved() {
            return;
        }

        context.record(IncomingCommand::RegisterPlayer {
            name,
            password,
            email,
            mission,
            figure,
            sex,
            birthday,
        });
    }
}

fn split_field_value(request: &dyn ClientMessage, index: usize) -> Option<String> {
    request
        .get_argument_with(index, "\r")
        .and_then(|field| field.split('=').nth(1))
        .map(ToOwned::to_owned)
}

fn prefixed_field_value(request: &dyn ClientMessage, index: usize, prefix: &str) -> Option<String> {
    request
        .get_argument_with(index, "\r")
        .and_then(|field| field.strip_prefix(prefix))
        .map(ToOwned::to_owned)
}

fn truncate_chars(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

#[cfg(test)]
#[path = "register_tests.rs"]
mod tests;
