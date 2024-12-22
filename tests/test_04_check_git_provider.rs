mod common;
use common::{clean_test_space, run_command, set_cache_path, set_git_provider, set_test_name};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_check_git_provider() {
    // Set CARGO_TEST_NAME
    set_test_name(function_name!());

    // Setup environment
    match set_git_provider() {
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
    let output = run_command(&vec!["check"], None);
    assert!(output.status.success());
}
