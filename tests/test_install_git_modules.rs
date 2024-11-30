use std::{env, process::Command};

mod common;
use common::{set_git_provider, set_cache_path, get_tests_path};

use serial_test::serial;

#[test]
#[serial]
fn test_install_git_modules() {
    // Setup env
    let _ = set_git_provider();
    let _ = set_cache_path();
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    println!("Running install ...");

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["install"])
        .output()
        .expect("Failed to execute binary");

    println!("Ran install ...");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

}