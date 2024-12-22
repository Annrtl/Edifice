mod common;
use common::{
    clean_test_space, get_cache_path, get_provider_hash, run_command, set_cache_path,
    set_local_provider,
};

use serial_test::serial;

#[test]
#[serial]
fn test_fetch_local_provider() {
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

    // Run the test
    let output = run_command(&vec!["fetch"], None);
    assert!(output.status.success());

    // Check providers are updated in the cache
    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    let not_expected_modules = vec!["local", "wb_streamer"];

    for not_expected_module in not_expected_modules {
        let cache_path_module = format!(
            "{}/providers/{}/{}",
            cache_path.display(),
            get_provider_hash(),
            not_expected_module
        );
        assert!(
            !std::path::Path::new(&cache_path_module).exists(),
            "{}",
            cache_path_module
        );
    }
}
