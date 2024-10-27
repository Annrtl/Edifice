use std::path::PathBuf;

use git2::{Cred, RemoteCallbacks};

pub fn download_repository(uri: String, path: PathBuf) -> Result<(), String> {
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

    if !path.exists() {
        // Clone the project.
        let provider_cache_path_str = match path.to_str() {
            Some(data) => data,
            None => return Err("Failed to get provider cache path".to_string()),
        };

        // Display message.
        print!("Cloning repository {uri} into {provider_cache_path_str} ... ");

        // Clone the repository.
        match builder.clone(&uri, &path) {
            Ok(_) => {
                println!("Ok");
            }
            Err(err) => {
                println!("Failed");
                return Err(format!("Failed to clone repository: {err}"));
            }
        };
    }
    Ok(())
}
