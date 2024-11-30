use std::{env, fs, path::PathBuf, process::Command};

mod common;
use common::{set_git_provider, set_cache_path};

use serial_test::serial;

#[test]
#[serial]
fn test_check() {
    // Setup env
    let _ = set_git_provider();
    let _ = set_cache_path();

    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["check"])
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

}