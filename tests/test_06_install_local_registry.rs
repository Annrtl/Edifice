mod common;
use common::{
    clean_test_space, get_modules_path, get_test_path, init_context, run_command, set_cache_path,
    set_local_registry,
};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_install_local_registry() {
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
    let output = run_command(&vec!["install"], None);
    assert!(output.status.success());

    // Check module.lock exists
    let lockfile = match get_test_path() {
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
