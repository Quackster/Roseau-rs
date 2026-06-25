use super::*;

#[test]
fn reports_help_requests_and_usage_text() {
    assert!(RoseauApplicationEntrypointUsage::requested(&[
        "--help".to_owned()
    ]));
    assert!(RoseauApplicationEntrypointUsage::requested(&[
        "-h".to_owned()
    ]));
    assert!(!RoseauApplicationEntrypointUsage::requested(&[
        "--max-ticks".to_owned(),
        "1".to_owned(),
    ]));
    assert!(RoseauApplicationEntrypointUsage::new()
        .text()
        .contains("--main-config <path>"));
}
