use super::*;
use std::io::Read;
use std::net::TcpListener;
use std::time::Duration;

#[test]
fn writes_encoded_responses_to_tcp_stream() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let mut client = TcpStream::connect(address).unwrap();
    let (server_stream, _) = listener.accept().unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let mut network = TcpPlayerNetwork::new(12, server_stream);
    network.send_response(NettyResponse::with_header("HELLO"));
    network.send_packet("#OK##");

    let mut bytes = [0; 13];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(&bytes, b"#HELLO###OK##");
    assert_eq!(network.connection_id(), 12);
    assert_eq!(network.server_port(), address.port());
    assert_eq!(network.last_error(), None);
}

#[test]
fn tracks_close_state() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let _client = TcpStream::connect(address).unwrap();
    let (server_stream, _) = listener.accept().unwrap();
    let mut network = TcpPlayerNetwork::new(7, server_stream);

    network.close();

    assert!(network.is_closed());
}

#[test]
fn reads_available_bytes_from_tcp_stream() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let mut client = TcpStream::connect(address).unwrap();
    let (server_stream, _) = listener.accept().unwrap();
    let mut network = TcpPlayerNetwork::new(4, server_stream);

    client.write_all(b"0010CHAT hello").unwrap();

    let mut buffer = [0; 64];
    let bytes_read = network.read_available(&mut buffer).unwrap();

    assert_eq!(&buffer[..bytes_read], b"0010CHAT hello");
    assert_eq!(network.last_error(), None);
}

#[test]
fn reports_nonblocking_idle_without_recording_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let _client = TcpStream::connect(address).unwrap();
    let (server_stream, _) = listener.accept().unwrap();
    let mut network = TcpPlayerNetwork::new(4, server_stream);
    let mut buffer = [0; 64];

    network.set_nonblocking(true).unwrap();

    assert_eq!(
        network.read_available_nonblocking(&mut buffer).unwrap(),
        None
    );
    assert_eq!(network.last_error(), None);
}
