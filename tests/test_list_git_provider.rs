use std::{env, fs, path::PathBuf, process::Command};

mod common;
use common::{set_git_provider, set_cache_path};

use serial_test::serial;

#[test]
#[serial]
fn test_list_git_provider() {
    // Setup env
    let _ = set_git_provider();
    let _ = set_cache_path();

    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Lancer le binaire
    Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["fetch"])
        .output()
        .expect("Failed to execute binary");

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["list"])
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };
    println!("{}", stdout);
    assert!(stdout.contains("hydra"));
    assert!(stdout.contains("wb_streamer"));
    assert!(!stdout.contains("local"));

}
