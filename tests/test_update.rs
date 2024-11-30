use std::{env, process::Command};

mod common;
use common::{get_tests_path, set_cache_path, set_git_provider};

use serial_test::serial;

#[test]
#[serial]
fn test_update() {
    // Setup env
    let _ = set_git_provider();
    let _ = set_cache_path();
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["update"])
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

    // Check lockfile exists
    let lockfile_path = tests_path.join("module.lock");
    assert!(std::path::Path::new(&lockfile_path).exists());

}
