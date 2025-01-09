use std::collections::HashMap;

use serde::Deserialize;

use crate::origins::origin::Origin;

use super::registry_module::RegistryModule;

#[derive(Deserialize)]
pub struct RegistryFileContent {
    pub modules: HashMap<String, RegistryModule>,
}

impl RegistryFileContent {
    pub fn new() -> Result<RegistryFileContent, String> {
        Ok(RegistryFileContent {
            modules: HashMap::new(),
        })
    }

    pub fn get_origins(&self) -> Result<Vec<Origin>, String> {
        let mut origins: Vec<Origin> = Vec::new();

        for (_, module) in &self.modules {
            origins.push(module.get_origin());
        }

        Ok(origins)
    }
}
