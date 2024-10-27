use std::{env, path::PathBuf};

use git::download_repository;

pub mod git;

pub fn get_providers() -> Result<Vec<String>, String> {
    let providers = match env::var("HYDRA_PROVIDERS") {
        Ok(data) => data,
        Err(_) => {
            return Err("No git provider found".to_string());
        }
    };

    Ok(providers
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}

fn get_cache_path() -> Result<PathBuf, String> {
    let home_path = match home::home_dir() {
        Some(data) => data,
        None => return Err("Failed to get home directory".to_string()),
    };

    let cache_dir = match env::var("HYDRA_CACHE") {
        Ok(data) => PathBuf::from(data),
        Err(_) => home_path.join(".cache/hydra"),
    };

    Ok(cache_dir)
}

pub fn update_providers_cache() -> Result<(), String> {
    let providers = match get_providers() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for provider in providers {
        // Get the project name.
        let provider_name = match provider.split("/").last() {
            Some(data) => data,
            None => return Err("Failed to get project name".to_string()),
        };

        // Get the cache directory.
        let cache_path = match get_cache_path() {
            Ok(data) => data.join("providers"),
            Err(err) => return Err(err),
        };

        let providers_cache_path = cache_path.join(provider_name);

        match download_repository(provider, providers_cache_path) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
