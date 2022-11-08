use assert_cmd::prelude::*;
use kvs::KvStore;
use predicates::str::contains;
use std::proccess::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn client_no_args_test() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("kvs-client").unwrap();
    cmd.current_dir(&temp_dir).assert().failure();
}
