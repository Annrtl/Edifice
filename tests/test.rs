use std::{env, process::Command};

#[test]
fn test_no_command() {
    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("No command provided"));
}

#[test]
fn test_show() {
    let test_dir = env::current_dir().unwrap().join("tests");
    // Lancer le binaire
    let output = Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&test_dir)
        .args(&["show"])
        .output()
        .expect("Failed to execute binary");

    // Vérifier que l'exécution est réussie
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
    assert!(stdout.contains("hydra"));
    assert!(stdout.contains("0.1.0"));
}
