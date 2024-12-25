mod common;
use common::{clean_test_space, init_context, run_command, set_cache_path, set_git_provider};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_list_git_provider() {
    init_context(function_name!());

    // Setup environment
    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    match set_git_provider() {
        Ok(_) => (),
        Err(err) => panic!("Failed to set git provider: {}", err),
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
    println!("{}", stdout);
    assert!(stdout.contains("edifice"));
    assert!(stdout.contains("wb_streamer"));
    assert!(!stdout.contains("local"));
}
