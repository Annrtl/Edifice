use std::path::PathBuf;

use crate::{modules::module_file::ModuleFile, origins::origin::Origin};

use super::{get_cache_path, git::download_repository, registry_file::RegistryFile};

pub struct Registry {
    pub uri: String,
    pub cache_path: PathBuf,
    pub registry_file: RegistryFile,
}

pub fn update(uri: String, cache_path: PathBuf) -> Result<(), String> {
    // Check if the registry is a local directory
    if !uri.starts_with("git@") {
        return Ok(());
    }

    // Else consider it as a git repository
    match download_repository(uri.clone(), cache_path.clone(), None) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    Ok(())
}

impl Registry {
    pub fn new(uri: String) -> Result<Registry, String> {
        let mut registry_cache_path: PathBuf;

        // If uri is not a local directory, create a subdirectory with the hash of the uri
        if !uri.starts_with("git@") {
            registry_cache_path = PathBuf::from(uri.clone());
        } else {
            let cache_path = match get_cache_path() {
                Ok(data) => data,
                Err(err) => return Err(err),
            };

            registry_cache_path = cache_path.join("registries");

            let crc = crc32fast::hash(uri.as_bytes());

            #[cfg(debug_assertions)]
            println!("Hash of uri {}: {:x}", uri, crc);

            registry_cache_path = registry_cache_path.join(format!("{:x}", crc));

            update(uri.clone(), registry_cache_path.clone())?;
        }

        let registry_file_path = registry_cache_path.join("registry.toml");

        let registry_file = match RegistryFile::new(registry_file_path) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        Ok(Registry {
            uri,
            cache_path: registry_cache_path,
            registry_file,
        })
    }

    pub fn update(&self) -> Result<(), String> {
        update(self.uri.clone(), self.cache_path.clone())
    }

    pub fn get_origins(&self) -> Result<Vec<Origin>, String> {
        self.registry_file.get_origins()
    }

    pub fn get_modulefiles(&self) -> Result<Vec<ModuleFile>, String> {
        let origins = match self.get_origins() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let mut modulefiles: Vec<ModuleFile> = Vec::new();

        for mut origin in origins {
            #[cfg(debug_assertions)]
            println!("Getting modulefile of origin: {}", origin.uri);

            let modulefile = match origin.get_modulefile() {
                Ok(data) => data,
                Err(err) => return Err(err),
            };
            modulefiles.push(modulefile);
        }

        Ok(modulefiles)
    }
}
