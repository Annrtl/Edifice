mod common;
use common::{clean_test_space, init_context, run_command, set_cache_path, set_local_registry};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_list_local_registry() {
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
    let output = run_command(&vec!["list"], None);
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    // Check output
    assert!(stdout.contains("| edifice"));
    assert!(stdout.contains("| wb_streamer"));
    assert!(stdout.contains("| local"));

    // Run the test
    let output = run_command(&vec!["list", "wb"], None);
    assert!(output.status.success());

    // Vérifier le contenu de la sortie standard
    let stdout = match String::from_utf8(output.stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    // Check output
    assert!(!stdout.contains("| edifice"));
    assert!(stdout.contains("| wb_streamer"));
    assert!(!stdout.contains("| local"));
}
