mod common;
use common::{
    clean_test_space, get_cache_path, get_registry_hash, init_context, run_command, set_cache_path,
    set_local_registry,
};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_fetch_local_registry() {
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

    // Run the test
    let output = run_command(&vec!["fetch"], None);
    assert!(output.status.success());

    // Check registries are updated in the cache
    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    let not_expected_modules = vec!["local", "wb_streamer"];

    for not_expected_module in not_expected_modules {
        let cache_path_module = format!(
            "{}/registries/{:x}/{}",
            cache_path.display(),
            get_registry_hash(),
            not_expected_module
        );
        assert!(
            !std::path::Path::new(&cache_path_module).exists(),
            "{}",
            cache_path_module
        );
    }
}
