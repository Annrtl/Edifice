use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::Deserialize;

use crate::origins::origin::Origin;

use super::registry_module_origin::RegistryModuleOrigin;

#[derive(Deserialize)]
pub struct RegistryModule {
    pub name: String,
    pub version: Version,
    pub dependencies: Option<HashMap<String, VersionReq>>,
    pub origin: RegistryModuleOrigin,
}

impl RegistryModule {
    pub fn get_origin(&self) -> Origin {
        self.origin.get_origin()
    }
}
