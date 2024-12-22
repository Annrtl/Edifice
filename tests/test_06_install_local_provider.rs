mod common;
use common::{
    clean_test_space, get_modules_path, get_tests_path, run_command, set_cache_path,
    set_local_provider,
};

use serial_test::serial;

#[test]
#[serial]
fn test_install_local_provider() {
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
    let output = run_command(&vec!["install"], None);
    assert!(output.status.success());

    // Check module.lock exists
    let lockfile = match get_tests_path() {
        Ok(path) => path.join("module.lock"),
        Err(err) => panic!("Failed to get tests path: {}", err),
    };

    assert!(lockfile.exists(), "{}", lockfile.display());

    // Check modules are installed
    let modules_path = match get_modules_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get modules path: {}", err),
    };

    let expected_modules = vec!["local", "wb_streamer"];

    for expected_module in expected_modules {
        let module_path = format!("{}/{}", modules_path.display(), expected_module);
        assert!(
            std::path::Path::new(&module_path).exists(),
            "{}",
            module_path
        );
    }
}
