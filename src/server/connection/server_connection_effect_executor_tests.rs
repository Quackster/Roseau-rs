use super::*;
use crate::messages::IncomingCommand;
use crate::protocol::NettyRequest;
use crate::server::ServerConnectionHandler;

#[test]
fn applies_open_and_close_effects_to_server_state() {
    let connection_handler = ServerConnectionHandler::new(true, false);
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let mut executor = ServerConnectionEffectExecutor::new();

    executor.apply_all(
        &mut server_handler,
        IncomingContext::new(),
        connection_handler.channel_open(7, "/127.0.0.1:37120"),
    );

    assert!(server_handler.session_manager().has_session(7));
    assert_eq!(
        executor.network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 7,
            packet: "#HELLO##".to_owned(),
        }]
    );
    assert_eq!(
        executor.connection_logs(),
        &["[7] Connection from 127.0.0.1".to_owned()]
    );

    executor.apply_all(
        &mut server_handler,
        IncomingContext::new(),
        connection_handler.channel_closed(7, "/127.0.0.1:37120"),
    );

    assert!(!server_handler.session_manager().has_session(7));
    assert!(executor
        .session_effects()
        .contains(&SessionLifecycleEffect::RemovePlayer { connection_id: 7 }));
}

#[test]
fn dispatches_request_effects_through_server_message_handler() {
    let connection_handler = ServerConnectionHandler::new(false, true);
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let mut executor = ServerConnectionEffectExecutor::new();
    let effects = connection_handler.message_received(
        9,
        37120,
        Some(NettyRequest::from_content("CHAT hello")),
    );

    let _context = executor.apply_all(
        &mut server_handler,
        IncomingContext::new().in_room(true),
        effects,
    );

    assert_eq!(
        executor.packet_logs(),
        &["[9] Received: CHAT hello".to_owned()]
    );
    assert_eq!(
        executor.pending_incoming_commands()[0].commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
}

#[test]
fn dispatches_immediate_response_packets_to_current_connection() {
    let connection_handler = ServerConnectionHandler::new(false, true);
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let mut executor = ServerConnectionEffectExecutor::new();
    let effects = connection_handler.message_received(
        9,
        37120,
        Some(NettyRequest::from_content("VERSIONCHECK")),
    );

    let context = executor.apply_all(&mut server_handler, IncomingContext::new(), effects);

    assert!(context.sent().is_empty());
    let secret_key = context.rc4_secret_key_value().unwrap();
    assert_eq!(secret_key.len(), 62);
    assert_eq!(executor.network_effects().len(), 2);
    assert_eq!(
        executor.network_effects()[0],
        PlayerNetworkEffect::WriteResponse {
            connection_id: 9,
            packet: "#ENCRYPTION_ON##".to_owned(),
        }
    );
    assert_eq!(
        executor.network_effects()[1],
        PlayerNetworkEffect::WriteResponse {
            connection_id: 9,
            packet: format!("#SECRET_KEY\r{secret_key}##"),
        }
    );
}

#[test]
fn applies_exception_effect_as_network_close() {
    let connection_handler = ServerConnectionHandler::new(false, false);
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let mut executor = ServerConnectionEffectExecutor::new();

    executor.apply_all(
        &mut server_handler,
        IncomingContext::new(),
        connection_handler.exception_caught(3),
    );

    assert_eq!(
        executor.network_effects(),
        &[PlayerNetworkEffect::CloseConnection { connection_id: 3 }]
    );
}
