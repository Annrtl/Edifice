use std::{env, path::PathBuf};

use git::download_repository;

use crate::module::{parser::get_module_files, ModuleFile};

pub mod git;

pub fn get_providers() -> Result<Vec<String>, String> {
    let providers = match env::var("EDIFICE_PROVIDERS") {
        Ok(data) => data,
        Err(_) => {
            return Err(
                "No provider URI found from environment variable: EDIFICE_PROVIDERS".to_string(),
            );
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

    let cache_dir = match env::var("EDIFICE_CACHE") {
        Ok(data) => PathBuf::from(data),
        Err(_) => home_path.join(".cache/edifice"),
    };

    Ok(cache_dir)
}

pub fn get_providers_sync_paths() -> Result<Vec<PathBuf>, String> {
    let providers = match get_providers() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut providers_sync_paths = Vec::new();

    for provider in providers {
        if provider.starts_with("/") {
            providers_sync_paths.push(PathBuf::from(provider));
            continue;
        }

        let provider_cache_path = match get_provider_cache_path(&provider) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        providers_sync_paths.push(provider_cache_path);
    }

    Ok(providers_sync_paths)
}

pub fn get_providers_cache_path() -> Result<PathBuf, String> {
    let cache_path = match get_cache_path() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    Ok(cache_path.join("providers"))
}

fn get_provider_cache_path(provider: &str) -> Result<PathBuf, String> {
    let providers_cache_path = match get_providers_cache_path() {
        Ok(data) => data,
        Err(_) => return Err("Failed to get providers cache path".to_string()),
    };

    let provider_checksum = crc32fast::hash(provider.as_bytes());

    let provider_cache_path = providers_cache_path.join(provider_checksum.to_string());
    Ok(provider_cache_path)
}

pub fn update_providers_cache() -> Result<(), String> {
    let providers = match get_providers() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for provider in providers {
        // Do not cache local providers.
        if provider.starts_with("/") {
            println!("Not caching local provider: {}", provider);
            continue;
        }

        let provider_cache_path = match get_provider_cache_path(&provider) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        match download_repository(provider, provider_cache_path, None) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

pub fn get_providers_modules_path() -> Result<Vec<PathBuf>, String> {
    let providers = match get_providers() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut providers_modules_path = Vec::new();

    providers_modules_path.push(PathBuf::from("."));

    for provider in providers {
        if provider.starts_with("/") {
            providers_modules_path.push(PathBuf::from(provider));
            continue;
        }

        let provider_cache_path = match get_provider_cache_path(&provider) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        providers_modules_path.push(provider_cache_path);
    }

    Ok(providers_modules_path)
}

pub fn get_providers_modules() -> Result<Vec<ModuleFile>, String> {
    let providers_modules_path = match get_providers_modules_path() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut providers_module_files = Vec::new();

    for provider_modules_path in providers_modules_path {
        let modules_files = match get_module_files(Some(provider_modules_path)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        for module_file in modules_files {
            providers_module_files.push(module_file);
        }
    }

    Ok(providers_module_files)
}
