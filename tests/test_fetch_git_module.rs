use std::{env, process::Command};

mod common;
use common::{get_cache_path, get_tests_path, set_cache_path, set_git_provider};

use serial_test::serial;

#[test]
#[serial]
fn test_fetch_git_module() {
    // Setup env
    let _ = set_git_provider();
    let _ = set_cache_path();
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };
    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    // Check directory doesn't already exist
    if std::path::Path::new(&cache_path).exists() {
        println!("Removing cache directory");
        match std::fs::remove_dir_all(cache_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove cache directory: {}", err),
        }
    }

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["fetch"])
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
    assert!(stdout.contains("Done"));

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["fetch"])
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
    assert!(stdout.contains("Done"));

}