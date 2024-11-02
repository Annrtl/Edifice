use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::Deserialize;

pub mod dfs;
pub mod lock;
pub mod parser;

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct Provider {
    pub uri: String,
    pub commit: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ModuleFile {
    pub module: Module,
    pub dependencies: Option<HashMap<String, VersionReq>>,
    pub provider: Option<Provider>,
}
