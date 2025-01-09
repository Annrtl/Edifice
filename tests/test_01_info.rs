mod common;
use common::{clean_test_space, init_context, run_command, set_cache_path, set_local_registry};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_info() {
    init_context(function_name!());

    // Setup environment
    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    match set_local_registry() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set local registry: {}", err),
    };

    match set_cache_path() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set cache path: {}", err),
    };

    // Vérifier que l'exécution est réussie
    let output = run_command(&vec!["info"], None);
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };
    println!("{}", stdout);
    assert!(stdout.contains("edifice"));
    assert!(stdout.contains("0.1.0"));
}
