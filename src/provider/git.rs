use std::{env, path::{Path, PathBuf}};
use git2::{Cred, RemoteCallbacks};
use home;

pub fn get_providers() -> Vec<String> {
    let providers_env_var: Result<String, env::VarError> = env::var("HYDRA_PROVIDERS");

    let mut providers_string: String = String::new();

    match providers_env_var {
        Ok(data) => {
            providers_string = data;
        }
        Err(_) => {
            println!("No git provider found");
        }
    }
    
    providers_string.split(";").map(|s| s.to_string()).collect::<Vec<String>>() 
}

pub fn update_cache() -> Result<(), String>{

    let home_dir: PathBuf = home::home_dir().unwrap();
    
    let providers = get_providers();

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    for provider in providers {
        // Get the project name.
        let project_name = provider.split("/").last().unwrap();
        
        // Get the cache directory.
        let cache_dir = home_dir.join(".cache/hydra").join(project_name);

        if ! cache_dir.exists() {
            // Clone the project.
            print!("Cloning repository {project_name} ({provider})into {provider} ... ");
            match builder.clone(
                &provider,
                &cache_dir,
            ){
                Ok(_) => {
                    println!("Ok");
                },
                Err(err) => {
                    println!("Failed");
                    return Err(format!("Failed to clone repository: {err}"));
                },
            };
        }
    }
    Ok(())
}