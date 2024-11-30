use std::{env, fs, path::PathBuf};


pub fn get_tests_path() -> Result<PathBuf, std::io::Error> {
    // Get test path
    fs::canonicalize(PathBuf::from("tests"))
}

#[allow(dead_code)]
pub fn set_git_provider() -> Result<(), String> {
    env::set_var(
        "HYDRA_PROVIDERS",
        //"git@github.com:Annrtl/hydra_registry.git",
        "git@github.com:Annrtl/fusesoc-cores.git",
    );
    Ok(())
}

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
pub fn set_local_provider() -> Result<(), String> {
    // Get test path
    let tests_path = match fs::canonicalize(PathBuf::from("tests")) {
        Ok(path) => path,
        Err(err) => panic!("Failed to get test path: {}", err),
    };
    
    env::set_var(
        "HYDRA_PROVIDERS",
        //"git@github.com:Annrtl/hydra_registry.git",
        format!("{}/test_provider", tests_path.display()),
    );
    Ok(())
}