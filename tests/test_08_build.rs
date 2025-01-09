mod common;
use common::{
    clean_test_space, create_local_dataset, init_context, run_command, set_cache_path,
    set_git_registry,
};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_build() {
    init_context(function_name!());

    // Setup environment
    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    match set_git_registry() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set local registry: {}", err),
    };

    match set_cache_path() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set cache path: {}", err),
    };

    match create_local_dataset() {
        Ok(_) => (),
        Err(err) => panic!("Failed to create local dataset: {}", err),
    }
    // Run the test
    let output = run_command(&vec!["build"], None);
    assert!(output.status.success());
}
