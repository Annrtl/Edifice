use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

pub mod dfs;
pub mod lock;
pub mod parser;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub uri: String,
    pub commit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleFile {
    pub module: Module,
    pub dependencies: Option<HashMap<String, VersionReq>>,
    pub provider: Option<Provider>,
}

#[derive(Serialize, Deserialize)]
pub struct LockPackage {
    pub name: String,
    pub version: Version,
    pub uri: String,
    pub commit: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LockFile {
    pub version: u8,
    pub packages: Vec<LockPackage>,
}
