use std::{env, fs, path::PathBuf, process::Command};

fn get_tests_path() -> Result<PathBuf, std::io::Error> {
    // Get test path
    fs::canonicalize(PathBuf::from("tests"))
}

fn set_provider() -> Result<(), String> {
    env::set_var(
        "HYDRA_PROVIDERS",
        //"git@github.com:Annrtl/hydra_registry.git",
        "git@github.com:Annrtl/fusesoc-cores.git",
    );
    Ok(())
}

fn set_cache_path() -> Result<(), std::io::Error> {
    let test_path = get_tests_path()?;
    let cache_path = test_path.join("cache");
    env::set_var("HYDRA_CACHE", cache_path.clone());
    Ok(())
}

fn get_cache_path() -> Result<PathBuf, std::io::Error> {
    let path_string = match env::var("HYDRA_CACHE") {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    let path = PathBuf::from(path_string);
    Ok(path)
}

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
    // Setup env
    let _ = set_provider();
    let _ = set_cache_path();

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
fn test_list() {
    // Setup env
    let _ = set_provider();
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

    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    // Remove cache directory
    if std::path::Path::new(&cache_path).exists() {
        println!("Removing cache directory");
        match std::fs::remove_dir_all(cache_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove cache directory: {}", err),
        }
    }
}

#[test]
fn test_fetch() {
    // Setup env
    let _ = set_provider();
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

    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    // Remove cache directory
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
    // Setup env
    let _ = set_provider();
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

    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    // Remove cache directory
    if std::path::Path::new(&cache_path).exists() {
        println!("Removing cache directory");
        match std::fs::remove_dir_all(cache_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove cache directory: {}", err),
        }
    }
}

#[test]
fn test_update() {
    // Setup env
    let _ = set_provider();
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

    if std::path::Path::new(&lockfile_path).exists() {
        match std::fs::remove_file(lockfile_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove lockfile: {}", err),
        }
    }

    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    // Remove cache directory
    if std::path::Path::new(&cache_path).exists() {
        println!("Removing cache directory");
        match std::fs::remove_dir_all(cache_path.clone()) {
            Ok(_) => (),
            Err(err) => panic!("Failed to remove cache directory: {}", err),
        }
    }
}
