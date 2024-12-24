use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

struct Context {
    test_name: String,
    tests_path: PathBuf,
    test_path: PathBuf,
}

impl Context {
    pub fn new(test_name: &str) -> Self {
        let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
            Ok(path) => path,
            Err(err) => panic!("Failed to get tests path: {}", err),
        };
        let test_path = tests_path.join(test_name);
        Context {
            test_name: test_name.to_string(),
            tests_path: tests_path,
            test_path: test_path,
        }
    }
}

// Utiliser un Mutex pour garantir un accès thread-safe
static CONTEXT: Mutex<Option<Context>> = Mutex::new(None);

pub fn init_context(test_name: &str) {
    let mut ctx = CONTEXT.lock().unwrap();
    *ctx = Some(Context::new(test_name));
}

pub fn get_tests_path() -> Result<PathBuf, std::io::Error> {
    // Get test path
    let tests_path = CONTEXT.lock().unwrap().as_ref().unwrap().tests_path.clone();
    Ok(tests_path)
}

pub fn get_test_path() -> Result<PathBuf, std::io::Error> {
    // Get test path
    let test_path = CONTEXT.lock().unwrap().as_ref().unwrap().test_path.clone();
    // Check if test path exists
    if !test_path.exists() {
        fs::create_dir(&test_path)?;
    }
    Ok(test_path)
}

pub fn create_module(content: String) -> Result<(), String> {
    // Get test path
    let tests_path = match get_test_path() {
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
    let test_path = get_test_path()?;
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
    let test_path = get_test_path()?;
    let modules_path = test_path.join("modules");
    Ok(modules_path)
}

#[allow(dead_code)]
pub fn set_local_provider() -> Result<(), String> {
    // Get test path
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get test path: {}", err)),
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

pub fn set_both_providers() -> Result<(), String> {
    let tests_path = match get_tests_path() {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get test path: {}", err)),
    };

    let providers: Vec<String> = vec![
        format!("{}/local_provider", tests_path.display()),
        "git@github.com:Annrtl/fusesoc-cores.git".to_string(),
    ];

    let providers = providers.join(";");

    env::set_var("HYDRA_PROVIDERS", &providers);

    // Create module that use module only remote modules
    create_generic_module()?;
    Ok(())
}

#[allow(dead_code)]
pub fn clean_test_space() -> Result<(), String> {
    // Get test path
    let test_path = match get_test_path() {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get test path: {}", err)),
    };

    let _ = fs::remove_dir_all(&test_path);

    Ok(())
}

#[allow(dead_code)]
pub fn run_command(args: &[&str], exp_fail: Option<bool>) -> std::process::Output {
    let exp_fail = match exp_fail {
        Some(val) => val,
        None => false,
    };

    // Get test path
    let test_path = match get_test_path() {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };

    // Run the command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hydra"))
        .current_dir(&test_path)
        .args(args)
        .output()
        .expect("Failed to execute binary");

    // write stdout to hydra.stdout
    let stdout = match String::from_utf8(output.clone().stdout) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stdout: {}", err),
    };

    // Write stdout
    let stdout_file_path = test_path.join("hydra.stdout");
    let mut stdout_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(stdout_file_path)
        .expect("Failed to open stdout file");

    stdout_file
        .write("\n---\n---\n---\n\n".as_bytes())
        .expect("Failed to write to stdout file");
    stdout_file
        .write(&stdout.as_bytes())
        .expect("Failed to write to stdout file");

    // write stderr to hydra.stderr
    let stderr = match String::from_utf8(output.clone().stderr) {
        Ok(data) => data,
        Err(err) => panic!("Failed to get stderr: {}", err),
    };

    // Write stderr
    let stderr_file_path = test_path.join("hydra.stderr");
    let mut stderr_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(stderr_file_path)
        .expect("Failed to open stderr file");
    stderr_file
        .write("\n---\n---\n---\n\n".as_bytes())
        .expect("Failed to write to stdout file");
    stderr_file
        .write(&stderr.as_bytes())
        .expect("Failed to write to stderr file");

    // Assert the command was successful
    assert!(output.status.success() == !exp_fail);

    output
}
