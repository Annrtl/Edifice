mod common;
use common::{
    clean_test_space, get_test_path, run_command, set_cache_path, set_local_provider, init_context,
};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_update_local_provider() {
    init_context(function_name!());

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
    let output = run_command(&vec!["update"], None);

    assert!(output.status.success());

    // Check lockfile exists
    let test_path = match get_test_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    let lockfile_path = test_path.join("module.lock");

    assert!(std::path::Path::new(&lockfile_path).exists());
}
