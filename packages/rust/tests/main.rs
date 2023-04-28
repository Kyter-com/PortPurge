use portpurge::utils;

#[cfg(unix)]
#[test]
fn return_no_results_on_port_unix() {
    let result = utils::unix_find_pids_on_port(1234);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[cfg(windows)]
#[test]
fn return_no_results_on_port_windows() {
    let result = utils::windows_find_pids_on_port(1234);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
