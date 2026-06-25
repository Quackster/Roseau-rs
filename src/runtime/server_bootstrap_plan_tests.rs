use super::*;

fn plan(bind_ip: &str, raw_config_ip: &str) -> ServerBootstrapPlan {
    ServerBootstrapPlan::new(
        bind_ip,
        raw_config_ip,
        37120,
        37119,
        "roseau::server::ServerHandler",
        DatabaseEngine::MySql,
        vec![37120, 37119],
    )
}

#[test]
fn reports_numeric_config_ip_as_advertised_listen_address() {
    let plan = plan("127.0.0.1", "127.0.0.1");

    assert_eq!(plan.advertised_ip(Some("192.168.0.20")), "127.0.0.1");
    assert_eq!(
        plan.listen_status(true, Some("192.168.0.20")),
        RoseauStartupStatus::Listening {
            server_ip: "127.0.0.1".to_owned(),
            server_port: 37120,
        }
    );
}

#[test]
fn reports_resolved_hostname_after_wildcard_bind() {
    let plan = plan("0.0.0.0", "roseau.local");

    assert_eq!(plan.advertised_ip(Some("10.0.0.25")), "10.0.0.25");
    assert_eq!(
        plan.listen_status(true, Some("10.0.0.25")).log_line(),
        "Server is listening on 10.0.0.25:37120"
    );
}

#[test]
fn reports_listen_failure_on_primary_server_port() {
    let plan = plan("0.0.0.0", "roseau.local");

    assert_eq!(
        plan.listen_status(false, Some("10.0.0.25")),
        RoseauStartupStatus::ListenFailed { server_port: 37120 }
    );
}

#[test]
fn reports_startup_status_from_listen_outcome() {
    let plan = plan("127.0.0.1", "127.0.0.1");
    let listen_plan = crate::server::ServerListenPlan::new(plan.bind_ip(), plan.ports().to_vec());

    assert_eq!(
        plan.listen_outcome_status(
            &ServerListenOutcome::success_for_plan(&listen_plan),
            Some("10.0.0.25")
        ),
        RoseauStartupStatus::Listening {
            server_ip: "127.0.0.1".to_owned(),
            server_port: 37120,
        }
    );
    assert_eq!(
        plan.listen_outcome_status(
            &ServerListenOutcome::failure_for_plan(&listen_plan, "127.0.0.1:37119"),
            Some("10.0.0.25")
        ),
        RoseauStartupStatus::ListenFailed { server_port: 37120 }
    );
}
