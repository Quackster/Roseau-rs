use super::*;
use crate::messages::IncomingCommand;
use crate::protocol::DecodeError;
use crate::server::{secret_decode, PlayerNetworkEffect, Rc4Cipher, SessionLifecycleEffect};

#[test]
fn opens_and_closes_connection_through_handler_effects() {
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(true, false);
    let mut effect_executor = ServerConnectionEffectExecutor::new();
    let mut driver = ServerConnectionDriver::new(7, "/127.0.0.1:37120");

    driver.open(
        &mut server_handler,
        &connection_handler,
        &mut effect_executor,
    );

    assert!(server_handler.session_manager().has_session(7));
    assert_eq!(
        effect_executor.network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 7,
            packet: "#HELLO##".to_owned(),
        }]
    );

    driver.close(
        &mut server_handler,
        &connection_handler,
        &mut effect_executor,
    );

    assert!(!server_handler.session_manager().has_session(7));
    assert!(effect_executor
        .session_effects()
        .contains(&SessionLifecycleEffect::RemovePlayer { connection_id: 7 }));
}

#[test]
fn buffers_partial_reads_and_dispatches_complete_requests() {
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, true);
    let mut effect_executor = ServerConnectionEffectExecutor::new();
    let mut driver = ServerConnectionDriver::new(9, "/127.0.0.1:37120")
        .with_context(IncomingContext::new().in_room(true));

    driver
        .read_bytes(
            b"0010CHAT",
            37120,
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        )
        .unwrap();

    assert_eq!(driver.buffered_len(), 8);
    assert!(driver.context().commands().is_empty());

    driver
        .read_bytes(
            b" hello",
            37120,
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        )
        .unwrap();

    assert_eq!(
        effect_executor.pending_incoming_commands()[0].commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
    assert_eq!(
        effect_executor.packet_logs(),
        &["[9] Received: CHAT hello".to_owned()]
    );
}

#[test]
fn closes_connection_on_decode_error() {
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, false);
    let mut effect_executor = ServerConnectionEffectExecutor::new();
    let mut driver = ServerConnectionDriver::new(11, "/127.0.0.1:37120");

    let error = driver
        .read_bytes(
            b"ABCDCHAT hello",
            37120,
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        )
        .unwrap_err();

    assert_eq!(error, DecodeError::InvalidLength);
    assert_eq!(
        effect_executor.network_effects(),
        &[PlayerNetworkEffect::CloseConnection { connection_id: 11 }]
    );
}

#[test]
fn decrypts_rc4_hex_frames_after_version_check() {
    let mut server_handler = ServerHandler::new(vec![37120], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, true);
    let mut effect_executor = ServerConnectionEffectExecutor::new();
    let mut driver = ServerConnectionDriver::new(12, "/127.0.0.1:37120");

    driver
        .read_bytes(
            b"0012VERSIONCHECK",
            37120,
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        )
        .unwrap();

    assert!(driver.rc4_enabled());

    let rc4_key = secret_decode(driver.context().rc4_secret_key_value().unwrap());
    let mut cipher = Rc4Cipher::new(rc4_key);
    let encrypted = cipher.encipher_hex(b"0014KEYENCRYPTED 1");
    driver
        .read_bytes(
            encrypted,
            37120,
            &mut server_handler,
            &connection_handler,
            &mut effect_executor,
        )
        .unwrap();

    assert_eq!(
        effect_executor.packet_logs(),
        &[
            "[12] Received: VERSIONCHECK ".to_owned(),
            "[12] Received: KEYENCRYPTED 1".to_owned()
        ]
    );
}

#[test]
fn keeps_rc4_stream_state_separate_per_connection() {
    let mut server_handler = ServerHandler::new(vec![37120, 37119, 37121], "127.0.0.1");
    let connection_handler = ServerConnectionHandler::new(false, true);
    let mut effect_executor = ServerConnectionEffectExecutor::new();
    let mut main = ServerConnectionDriver::new(21, "/127.0.0.1:37120");
    let mut private = ServerConnectionDriver::new(22, "/127.0.0.1:37119")
        .with_context(IncomingContext::new().main_server_connection(false));
    let mut public = ServerConnectionDriver::new(23, "/127.0.0.1:37121")
        .with_context(IncomingContext::new().main_server_connection(false));

    for driver in [&mut main, &mut private, &mut public] {
        driver
            .read_bytes(
                b"0012VERSIONCHECK",
                37120,
                &mut server_handler,
                &connection_handler,
                &mut effect_executor,
            )
            .unwrap();
        assert!(driver.rc4_enabled());
    }

    for (driver, port) in [
        (&mut main, 37120),
        (&mut private, 37119),
        (&mut public, 37121),
    ] {
        let rc4_key = secret_decode(driver.context().rc4_secret_key_value().unwrap());
        let mut cipher = Rc4Cipher::new(rc4_key);
        driver
            .read_bytes(
                cipher.encipher_hex(b"0014KEYENCRYPTED 1"),
                port,
                &mut server_handler,
                &connection_handler,
                &mut effect_executor,
            )
            .unwrap();
    }

    assert_eq!(
        effect_executor.packet_logs(),
        &[
            "[21] Received: VERSIONCHECK ".to_owned(),
            "[22] Received: VERSIONCHECK ".to_owned(),
            "[23] Received: VERSIONCHECK ".to_owned(),
            "[21] Received: KEYENCRYPTED 1".to_owned(),
            "[22] Received: KEYENCRYPTED 1".to_owned(),
            "[23] Received: KEYENCRYPTED 1".to_owned(),
        ]
    );
}
