use super::server_connection_handler::*;

fn request(header: &str, body: &str) -> NettyRequest {
    NettyRequest::new(header, body)
}

#[test]
fn open_and_close_emit_session_and_logging_effects() {
    let handler = ServerConnectionHandler::new(true, false);

    assert_eq!(
        handler.channel_open(7, "/127.0.0.1:30001"),
        vec![
            ServerConnectionEffect::SendHello { connection_id: 7 },
            ServerConnectionEffect::AddSession { connection_id: 7 },
            ServerConnectionEffect::LogConnection {
                line: "[7] Connection from 127.0.0.1".to_owned(),
            },
        ]
    );
    assert_eq!(
        handler.channel_closed(7, "/127.0.0.1:30001"),
        vec![
            ServerConnectionEffect::RemoveSession { connection_id: 7 },
            ServerConnectionEffect::DisposePlayer { connection_id: 7 },
            ServerConnectionEffect::LogConnection {
                line: "[7] Disconnection from 127.0.0.1".to_owned(),
            },
        ]
    );
}

#[test]
fn message_received_redacts_password_like_java_handler() {
    let handler = ServerConnectionHandler::new(false, true);
    let request = request("LOGIN", "alice secret");

    let effects = handler.message_received(9, Some(request.clone()));

    assert_eq!(
        effects,
        vec![
            ServerConnectionEffect::LogPacket {
                line: "[9] Received: LOGIN alice".to_owned(),
            },
            ServerConnectionEffect::DispatchRequest {
                connection_id: 9,
                request,
            },
        ]
    );
}

#[test]
fn packet_logging_handles_update_and_regular_messages() {
    assert_eq!(
        ServerConnectionHandler::packet_log_line(1, &request("UPDATE", "figure=hd")),
        "[1] Received: UPDATE"
    );
    assert_eq!(
        ServerConnectionHandler::packet_log_line(1, &request("TALK", "hello")),
        "[1] Received: TALK hello"
    );
}

#[test]
fn ignores_missing_requests_and_closes_on_exception() {
    let handler = ServerConnectionHandler::new(false, true);

    assert!(handler.message_received(1, None).is_empty());
    assert_eq!(
        handler.exception_caught(1),
        vec![ServerConnectionEffect::CloseConnection { connection_id: 1 }]
    );
}
