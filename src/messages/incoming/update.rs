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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    fn update_body() -> String {
        [
            "name=alice".to_owned(),
            "password=secret".to_owned(),
            "email=a@example.com".to_owned(),
            "figure=sd=001/0".to_owned(),
            "directMail=0".to_owned(),
            "birthday=08.08.1997".to_owned(),
            "phonenumber=+44".to_owned(),
            "customData=hello\u{000b}world".to_owned(),
            "has_read_agreement=1".to_owned(),
            "sex=Male".to_owned(),
        ]
        .join("\r")
    }

    #[test]
    fn records_profile_update() {
        let mut context = IncomingContext::new();
        Update.handle(
            &mut context,
            &NettyRequest::from_content(&format!("UPDATE {}", update_body())),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UpdateProfile {
                password: "secret".to_owned(),
                email: "a@example.com".to_owned(),
                figure: "sd=001/0".to_owned(),
                mission: "hello world".to_owned(),
                sex: "Male".to_owned(),
            }]
        );
    }

    #[test]
    fn preserves_java_split_and_substring_update_field_semantics() {
        let body = update_body()
            .replace("password=secret", "password=sec=ret")
            .replace("email=a@example.com", "email=a=b@example.com")
            .replace("figure=sd=001/0", "figure=sd=001/0=tail")
            .replace("customData=hello\u{000b}world", "customData=hello=world")
            .replace("sex=Male", "sex=Male=ignored");
        let mut context = IncomingContext::new();
        Update.handle(
            &mut context,
            &NettyRequest::from_content(&format!("UPDATE {body}")),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UpdateProfile {
                password: "sec".to_owned(),
                email: "a".to_owned(),
                figure: "sd=001/0=tail".to_owned(),
                mission: "hello=world".to_owned(),
                sex: "Male".to_owned(),
            }]
        );
    }

    #[test]
    fn records_pool_figure_update() {
        let mut context = IncomingContext::new();
        Update.handle(
            &mut context,
            &NettyRequest::from_content("UPDATE ph_figure=abc"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UpdatePoolFigure {
                pool_figure: "abc".to_owned(),
            }]
        );
    }
}
