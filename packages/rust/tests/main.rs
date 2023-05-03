#[cfg(test)]
mod tests {
    use portpurge::utils;
    use std::net::TcpListener;

    #[cfg(unix)]
    #[test]
    fn return_no_results_on_port_unix() {
        let result = utils::unix_find_pids_on_port(1234);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn return_results_on_port_unix() {
        // Spin up a temporary server on port 3000
        let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
        let result = utils::unix_find_pids_on_port(3001);

        // Close the server
        drop(listener);

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[cfg(windows)]
    #[test]
    fn return_no_results_on_port_windows() {
        let result = utils::windows_find_pids_on_port(1234);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
