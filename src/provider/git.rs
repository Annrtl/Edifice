use std::{env, path::PathBuf};
use git2::Repository;
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
    
    for provider in providers {
        
        let cache_dir = home_dir.join(".cache/hydra");
        let repo = Repository::clone(&provider, cache_dir);
    
    }
    Ok(())
}