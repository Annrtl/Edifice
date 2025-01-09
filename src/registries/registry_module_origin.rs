use serde::Deserialize;

use crate::origins::origin::Origin;

#[derive(Deserialize)]
pub struct RegistryModuleOrigin {
    pub uri: String,
    pub commit: Option<String>,
}

impl RegistryModuleOrigin {
    pub fn get_origin(&self) -> Origin {
        Origin {
            uri: self.uri.clone(),
            commit: self.commit.clone(),
            cache_path: None,
        }
    }
}
