use crate::integration::{mock_endpoint, register_test, EndpointOptions};

#[test]
fn command_monitors_run() {
    let _server = mock_endpoint(
        EndpointOptions::new("POST", "/api/0/monitors/foo-monitor/checkins/", 200)
            .with_response_file("monitors/post-monitors.json"),
    );
    if cfg!(windows) {
        register_test("monitors/monitors-run-win.trycmd");
    } else {
        register_test("monitors/monitors-run.trycmd");
    }
}

#[test]
fn command_monitors_run_env() {
    let _server = mock_endpoint(
        EndpointOptions::new("POST", "/api/0/monitors/foo-monitor/checkins/", 200)
            .with_response_file("monitors/post-monitors.json"),
    );
    register_test("monitors/monitors-run-env.trycmd");
}

#[test]
fn command_monitors_run_help() {
    register_test("monitors/monitors-run-help.trycmd");
}
