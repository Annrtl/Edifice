use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn get_tests_path() -> Result<PathBuf, std::io::Error> {
    // Get test path
    fs::canonicalize(PathBuf::from("tests"))
}

pub fn create_module(content: String) -> Result<(), String> {
    // Get test path
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get test path: {}", err)),
    };

    // Set module path
    let module_path = tests_path.join("module.toml");

    // Check if module file exists
    if module_path.exists() {
        match fs::remove_file(&module_path) {
            Ok(_) => (),
            Err(err) => return Err(format!("Failed to remove module file: {}", err)),
        }
    }

    // Create module file
    let mut module_file = match File::create(&module_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create module file: {}", err)),
    };
    module_file.write_all(content.as_bytes()).unwrap();
    Ok(())
}

pub fn create_generic_module() -> Result<(), String> {
    let content = r#"
        [module]
        name = "hydra"
        version = "0.1.0"

        [dependencies]
        wb_streamer = "1.1.0"
    "#;
    create_module(content.to_string())
}

pub fn create_local_module() -> Result<(), String> {
    let content = r#"
        [module]
        name = "hydra"
        version = "0.1.0"

        [dependencies]
        local = "1.0.0"
    "#;
    create_module(content.to_string())
}

#[allow(dead_code)]
pub fn set_git_provider() -> Result<(), String> {
    env::set_var(
        "HYDRA_PROVIDERS",
        //"git@github.com:Annrtl/hydra_registry.git",
        "git@github.com:Annrtl/fusesoc-cores.git",
    );

    // Create module that use module only remote modules
    create_generic_module()?;

    Ok(())
}

#[allow(dead_code)]
pub fn get_provider_hash() -> u32 {
    let provider = match env::var("HYDRA_PROVIDERS") {
        Ok(provider) => provider,
        Err(err) => panic!("Failed to get provider: {}", err),
    };
    crc32fast::hash(provider.as_bytes())
}

#[allow(dead_code)]
pub fn set_cache_path() -> Result<(), std::io::Error> {
    let test_path = get_tests_path()?;
    let cache_path = test_path.join("cache");
    env::set_var("HYDRA_CACHE", cache_path.clone());
    Ok(())
}

#[allow(dead_code)]
pub fn get_cache_path() -> Result<PathBuf, std::io::Error> {
    let path_string = match env::var("HYDRA_CACHE") {
        Ok(path) => path,
        Err(err) => panic!("Failed to get cache path: {}", err),
    };

    let path = PathBuf::from(path_string);
    Ok(path)
}

#[allow(dead_code)]
pub fn get_modules_path() -> Result<PathBuf, std::io::Error> {
    let test_path = get_tests_path()?;
    let modules_path = test_path.join("modules");
    Ok(modules_path)
}

#[allow(dead_code)]
pub fn set_local_provider() -> Result<(), String> {
    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    env::set_var(
        "HYDRA_PROVIDERS",
        //"git@github.com:Annrtl/hydra_registry.git",
        format!("{}/local_provider", tests_path.display()),
    );

    // Create module that use module local
    create_local_module()?;

    Ok(())
}

#[allow(dead_code)]
pub fn clean_test_space() -> Result<(), String> {
    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get test path: {}", err)),
    };

    // Clean cache
    let cache_path = tests_path.join("cache");
    let _ = fs::remove_dir_all(&cache_path);

    // Clean modules
    let modules_path = tests_path.join("modules");
    let _ = fs::remove_dir_all(&modules_path);

    //Clean module.lock
    let module_lock_path = tests_path.join("module.lock");
    let _ = fs::remove_file(&module_lock_path);

    Ok(())
}

#[allow(dead_code)]
pub fn run_command(args: &[&str], exp_fail: Option<bool>) -> std::process::Output {
    let exp_fail = match exp_fail {
        Some(val) => val,
        None => false,
    };

    // Get test path
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Run the command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&tests_path)
        .args(args)
        .output()
        .expect("Failed to execute binary");

    // write stdout to hydra.stdout
    let stdout = match String::from_utf8(output.clone().stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    let mut log_file = File::create(tests_path.join("hydra.stdout")).unwrap();
    log_file.write_all(&stdout.as_bytes()).unwrap();

    // write stderr to hydra.stderr
    let stderr = match String::from_utf8(output.clone().stderr) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stderr: {}", err),
    };

    let mut log_file = File::create(tests_path.join("hydra.stderr")).unwrap();
    log_file.write_all(&stderr.as_bytes()).unwrap();

    // Assert the command was successful
    assert!(output.status.success() == !exp_fail);

    output
}
