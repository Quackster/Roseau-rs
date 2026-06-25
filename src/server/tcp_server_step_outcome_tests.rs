use super::*;

#[test]
fn exposes_common_connection_id_and_read_bytes() {
    let read = TcpServerStepOutcome::Read {
        connection_id: 7,
        bytes_read: 12,
    };
    let idle = TcpServerStepOutcome::Idle { connection_id: 6 };
    let closed = TcpServerStepOutcome::Closed { connection_id: 8 };

    assert_eq!(idle.connection_id(), 6);
    assert_eq!(idle.bytes_read(), None);
    assert_eq!(read.connection_id(), 7);
    assert_eq!(read.bytes_read(), Some(12));
    assert_eq!(closed.connection_id(), 8);
    assert_eq!(closed.bytes_read(), None);
}
