mod common;

use common::{clean_test_space, get_test_path, init_context, run_command};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_new() {
    init_context(function_name!());

    // Setup environment
    match clean_test_space() {
        Ok(_) => (),
        Err(err) => panic!("Failed to clean test space: {}", err),
    };

    // Create a new project
    let output = run_command(&vec!["new", "test_new"], None);
    assert!(output.status.success());

    // Check if the project directory and all its components was created
    let project_path = match get_test_path() {
        Ok(path) => path.join("test_new"),
        Err(_) => panic!("Failed to get test path"),
    };

    assert!(project_path.exists());
    assert!(project_path.join(".gitignore").exists());
    assert!(project_path.join("dataset.toml").exists());
    assert!(project_path.join("module.toml").exists());
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("target.toml").exists());

    // Check module.toml content
    let module_content = match std::fs::read_to_string(project_path.join("module.toml")) {
        Ok(content) => content,
        Err(err) => panic!("Failed to read module.toml: {}", err),
    };
    assert!(module_content.contains("name = \"test_new\""));
}
