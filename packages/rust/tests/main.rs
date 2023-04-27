use portpurge::utils;

#[test]
fn return_no_results_on_port() {
    let result = utils::unix_find_pids_on_port(1234);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
