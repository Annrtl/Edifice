mod common;
use common::{clean_test_space, run_command, set_cache_path, set_local_provider};

use serial_test::serial;

#[test]
#[serial]
fn test_show() {
    // Setup environment
    match set_local_provider() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set local provider: {}", err),
    };

    match set_cache_path() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set cache path: {}", err),
    };

    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    // Vérifier que l'exécution est réussie
    let output = run_command(&vec!["show"], None);
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
