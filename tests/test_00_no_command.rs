mod common;
use common::{run_command, set_test_name};

use function_name::named;
use serial_test::serial;

#[test]
#[serial]
#[named]
fn test_no_command() {
    // Set CARGO_TEST_NAME
    set_test_name(function_name!());

    // Vérifier que l'exécution est réussie
    let output = run_command(&vec![], Some(true));
    assert!(!output.status.success());

    // Vérifier le contenu de la sortie error

    let stderr = match String::from_utf8(output.stderr) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stderr: {}", err),
    };

    assert!(stderr.contains("No command provided"));
}
