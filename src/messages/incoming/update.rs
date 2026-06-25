use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Update;

impl IncomingEvent for Update {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let body = request.get_message_body();
        if body.contains("ph_figure") {
            let pool_figure = body.chars().skip(10).collect::<String>();
            context.record(IncomingCommand::UpdatePoolFigure { pool_figure });
            return;
        }

        let Some(password) = split_field_value(request, 1) else {
            return;
        };
        let Some(mut email) = split_field_value(request, 2) else {
            return;
        };
        let Some(figure) = prefixed_field_value(request, 3, "figure=") else {
            return;
        };
        let Some(mut mission) = prefixed_field_value(request, 7, "customData=") else {
            return;
        };
        let Some(sex) = split_field_value(request, 9) else {
            return;
        };

        mission = filter_input(&mission);

        if email.chars().count() > 256 {
            email = email.chars().take(256).collect();
        }

        if mission.chars().count() > 100 {
            mission = mission.chars().take(100).collect();
        }

        if password.chars().count() < 3 || figure.chars().count() < 3 {
            return;
        }

        if !(4..=6).contains(&sex.chars().count()) {
            return;
        }

        context.record(IncomingCommand::UpdateProfile {
            password,
            email,
            figure,
            mission,
            sex,
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
