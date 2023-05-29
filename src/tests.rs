#![cfg(test)]

use std::fs;

use super::*;

#[test]
fn no_log_file_set() {
    init_logger(None);

    info!("Main", "Hello World!");
}
#[test]
fn log_file_set() {
    let mut log_file_path = PathBuf::from("logs/main.log");

    init_logger(Some(log_file_path.clone()));

    info!("Main", "Hello World!");

    assert!(
        log_file_path.exists(),
        "The log file should have been created by now."
    );
    // remove the file from the path
    log_file_path.pop();
    fs::remove_dir_all(log_file_path)
        .unwrap_or_else(|erro| panic!("Could not remove the log file. Error: {erro}"));
}