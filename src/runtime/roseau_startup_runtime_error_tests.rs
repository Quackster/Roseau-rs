use super::roseau_startup_runtime_error::*;

#[test]
fn formats_not_listening_error() {
    assert_eq!(
        RoseauStartupRuntimeError::NotListening.to_string(),
        "startup runtime is not listening"
    );
}
