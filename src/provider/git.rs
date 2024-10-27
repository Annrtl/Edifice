use git2::{Cred, RemoteCallbacks};
use home;
use std::{env, path::PathBuf};

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

pub fn update_cache() -> Result<(), String> {
    let providers = match get_providers() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Get private home directory.
    let home = match home::home_dir() {
        Some(data) => data,
        None => return Err("Failed to get home directory".to_string()),
    };

    // Get the SSH key path.
    let private_key_path = home.join(".ssh/id_rsa");

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            match username_from_url {
                Some(username) => username,
                None => "git",
            },
            None,
            private_key_path.as_path(),
            None,
        )
    });

    // Prepare fetch options.
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    for provider in providers {
        // Get the project name.
        let project_name = match provider.split("/").last() {
            Some(data) => data,
            None => return Err("Failed to get project name".to_string()),
        };

        // Get the cache directory.
        let cache_path = match get_cache_path() {
            Ok(data) => data.join(project_name),
            Err(err) => return Err(err),
        };

        if !cache_path.exists() {
            // Clone the project.
            print!("Cloning repository {project_name} ({provider}) into {provider} ... ");
            match builder.clone(&provider, &cache_path) {
                Ok(_) => {
                    println!("Ok");
                }
                Err(err) => {
                    println!("Failed");
                    return Err(format!("Failed to clone repository: {err}"));
                }
            };
        }
    }
    Ok(())
}
