#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointUsage;

impl RoseauApplicationEntrypointUsage {
    pub fn new() -> Self {
        Self
    }

    pub fn text(&self) -> &'static str {
        "Usage: roseau-rs [options]\n\nOptions:\n  --main-config <path>\n  --hotel-config <path>\n  --max-ticks <count>\n  --first-connection-id <id>\n  --listener-index <index>\n  --max-bytes <bytes>\n  --accept-connection\n  --no-accept-connection\n  -h, --help"
    }

    pub fn requested(args: &[String]) -> bool {
        args.iter()
            .any(|argument| argument == "--help" || argument == "-h")
    }
}

#[cfg(test)]
mod tests {
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
}
