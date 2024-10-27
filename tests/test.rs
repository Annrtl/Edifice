use std::{env, fs, path::PathBuf, process::Command};

#[test]
fn test_no_command() {
    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };
    assert!(stdout.contains("No command provided"));
}

#[test]
fn test_show() {
    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(&["show"])
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
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_fetch() {
    // Set HYDRA_PROVIDERS environment variable
    env::set_var(
        "HYDRA_PROVIDERS",
        "git@github.com:Annrtl/hydra_registry.git",
    );

    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Set cache directory
    let cache_path = tests_path.join("cache");

    env::set_var("HYDRA_CACHE", cache_path.clone());

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

    // Check directory doesn't already exist
    if std::path::Path::new(&cache_path).exists() {
        println!("Removing cache directory");
        match std::fs::remove_dir_all(cache_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove cache directory: {}", err),
        }
    }
}

#[test]
fn test_check() {
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

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };
    println!("{}", stdout);
    assert!(stdout.contains("hydra"));
    assert!(stdout.contains("0.1.0"));
}
