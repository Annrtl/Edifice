use std::{env, path::PathBuf};

use registry::Registry;

pub mod git;
pub mod registry;
pub mod registry_file;
pub mod registry_file_content;
pub mod registry_module;
pub mod registry_module_origin;

pub fn get_cache_path() -> Result<PathBuf, String> {
    let home_path = match home::home_dir() {
        Some(data) => data,
        None => return Err("Failed to get home directory".to_string()),
    };

    let cache_dir = match env::var("EDIFICE_CACHE") {
        Ok(data) => PathBuf::from(data),
        Err(_) => home_path.join(".cache/edifice"),
    };

    Ok(cache_dir)
}

pub fn update_registries() -> Result<(), String> {
    let registries = match get_registries() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for registry in registries {
        match registry.update() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

pub fn get_registries() -> Result<Vec<Registry>, String> {
    let registries_env = match env::var("EDIFICE_REGISTRIES") {
        Ok(data) => data,
        Err(_) => return Ok(Vec::new()),
    };

    let registries_paths = registries_env
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut registries: Vec<Registry> = Vec::new();

    for registry_path in registries_paths {
        let registry = match Registry::new(registry_path) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        registries.push(registry);
    }

    Ok(registries)
}
