use std::collections::HashMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::origins::origin::Origin;

use super::module::Module;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleFileContent {
    pub module: Module,
    pub dependencies: Option<HashMap<String, VersionReq>>,
    pub origin: Option<Origin>,
}
