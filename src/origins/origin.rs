use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    modules::module_file::ModuleFile,
    registries::{get_cache_path, git::clone_and_checkout},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Origin {
    pub uri: String,
    pub commit: Option<String>,
    pub cache_path: Option<PathBuf>,
}

pub fn update(uri: String, commit: Option<String>, cache_path: PathBuf) -> Result<(), String> {
    // Check if the registry is a local directory
    if !uri.starts_with("git@") {
        return Ok(());
    }

    // Else consider it as a git repository
    match clone_and_checkout(uri.as_str(), cache_path.clone(), commit) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    Ok(())
}

impl Origin {
    pub fn set_cache_path(&mut self) -> Result<(), String> {
        let origin_cache_path: PathBuf;

        if !self.uri.starts_with("git@") {
            origin_cache_path = PathBuf::from(self.uri.clone());
        } else {
            let crc = crc32fast::hash(self.uri.as_bytes());

            #[cfg(debug_assertions)]
            println!("Hash of uri {}: {:x}", self.uri, crc);

            let cache_path = match get_cache_path() {
                Ok(data) => data,
                Err(err) => return Err(err),
            };

            let origins_cache_path = cache_path.join("origins");

            origin_cache_path = origins_cache_path.join(format!("{:x}", crc));

            update(
                self.uri.clone(),
                self.commit.clone(),
                origin_cache_path.clone(),
            )?;
        }

        self.cache_path = Some(origin_cache_path);

        Ok(())
    }

    pub fn get_modulefile(&mut self) -> Result<ModuleFile, String> {
        if self.cache_path.is_none() {
            match self.set_cache_path() {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }

        let cache_path = match &self.cache_path {
            Some(data) => data,
            None => return Err("Failed to get cache path".to_string()),
        };

        let modulefile_path = cache_path.join("module.toml");

        let modulefile = match ModuleFile::new(modulefile_path) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        Ok(modulefile)
    }
}
