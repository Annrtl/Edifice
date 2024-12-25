mod common;
use common::{
    clean_test_space, get_test_path, init_context, run_command, set_both_providers, set_cache_path,
};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_add_module() {
    init_context(function_name!());

    // Setup environment
    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    match set_both_providers() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set local provider: {}", err),
    };

    match set_cache_path() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set cache path: {}", err),
    };

    // Run the test
    let output = run_command(&vec!["add"], Some(true));
    assert!(!output.status.success());

    // Run the test
    let output = run_command(&vec!["add", "local", "--dry"], None);
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    assert!(stdout.contains("Resolved version of module local: 1.0.1"));

    // Run the test
    let output = run_command(&vec!["add", "local@1.0.0", "--dry"], None);
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    assert!(stdout.contains("Resolved version of module local: 1.0.0"));

    // Check module file DO NOT contains the local module
    let module_file_path = match get_test_path() {
        Ok(data) => data.join("module.toml"),
        Err(err) => panic!("Error getting test path: {:?}", err),
    };

    let module_file_content = match std::fs::read_to_string(&module_file_path) {
        Ok(data) => data,
        Err(err) => panic!("Error reading module file: {:?}", err),
    };

    assert!(!module_file_content.contains("local = "));

    // Run the test
    let output = run_command(&vec!["add", "local"], None);
    assert!(output.status.success());

    // Check module file contains the local module
    let module_file_path = match get_test_path() {
        Ok(data) => data.join("module.toml"),
        Err(err) => panic!("Error getting test path: {:?}", err),
    };

    let module_file_content = match std::fs::read_to_string(&module_file_path) {
        Ok(data) => data,
        Err(err) => panic!("Error reading module file: {:?}", err),
    };

    assert!(module_file_content.contains("local = \"^1.0.1\""));
}
