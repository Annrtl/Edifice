use std::path::PathBuf;

use git2::{build::RepoBuilder, Cred, Oid, RemoteCallbacks, Repository};

fn get_builder() -> RepoBuilder<'static> {
    // Get private home directory.
    let home = home::home_dir().unwrap();

    // Get the SSH key path.
    let private_key_path = home.join(".ssh/id_rsa");

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
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

    builder
}

pub fn download_repository(
    uri: String,
    path: PathBuf,
    branch: Option<String>,
) -> Result<(), String> {
    // Get builder
    let mut builder = get_builder();

    // Check if the path exists.
    if !path.exists() {
        // Clone the project.
        let provider_cache_path_str = match path.to_str() {
            Some(data) => data,
            None => return Err("Failed to get provider cache path".to_string()),
        };

        // Display message.
        print!("Cloning repository {uri} into {provider_cache_path_str} ... ");

        // Set the branch.
        if branch.is_some() {
            builder.branch(branch.unwrap().as_str());
        }

        // Clone the repository.
        match builder.clone(&uri, &path) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("Failed to clone repository: {err}"));
            }
        };
    }
    Ok(())
}

pub fn clone_and_checkout(
    uri: &str,
    path: PathBuf,
    commit_hash: Option<String>,
) -> Result<(), String> {
    // Get builder
    let mut builder = get_builder();

    // Clone the repository.
    match builder.clone(&uri, &path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Failed to clone repository: {err}"));
        }
    };

    // Ouvrir le dépôt
    let repo = match Repository::open(&path) {
        Ok(repo) => repo,
        Err(err) => {
            return Err(format!("Failed to open repository: {err}"));
        }
    };

    match commit_hash {
        Some(hash) => {
            // Trouver l'ID de l'objet (le commit) correspondant au hash spécifié
            let oid = match Oid::from_str(&hash) {
                Ok(oid) => oid,
                Err(err) => {
                    return Err(format!("Failed to get object ID: {err}"));
                }
            };
            let object = match repo.find_object(oid, None) {
                Ok(object) => object,
                Err(err) => {
                    return Err(format!("Failed to find object: {err}"));
                }
            };

            // Effectuer le checkout sur le commit spécifié
            match repo.checkout_tree(&object, None) {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("Failed to checkout tree: {err}"));
                }
            };
            match repo.set_head_detached(oid) {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("Failed to set head detached: {err}"));
                }
            };

            println!("Repository cloned and checked out to commit {}", hash);
        }
        None => {
            println!("Repository cloned");
        }
    }

    Ok(())
}
