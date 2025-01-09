use std::path::PathBuf;

use crate::origins::origin::Origin;

use super::registry_file_content::RegistryFileContent;

pub struct RegistryFile {
    pub path: PathBuf,
    pub content: Option<RegistryFileContent>,
}

impl RegistryFile {
    pub fn new(path: PathBuf) -> Result<RegistryFile, String> {
        let mut registry_file = RegistryFile {
            path,
            content: None,
        };

        match registry_file.load() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        Ok(registry_file)
    }

    pub fn load(&mut self) -> Result<(), String> {
        let path_str = match self.path.to_str() {
            Some(data) => data,
            None => return Err("Error getting path string".to_string()),
        };

        // Read the TOML file content
        let content = match std::fs::read_to_string(self.path.clone()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error reading registryfile {}: {}", path_str, err)),
        };

        // Deserialize the TOML file into a Rust structure
        let registry_file_content = match toml::from_str::<RegistryFileContent>(content.as_str()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error deserializing registry file: {:?}", err)),
        };

        self.content = Some(registry_file_content);

        Ok(())
    }

    pub fn get_origins(&self) -> Result<Vec<Origin>, String> {
        let content = match &self.content {
            Some(data) => data,
            None => return Err("Registry file content not loaded".to_string()),
        };
        content.get_origins()
    }
}
