use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
pub struct LockPackage {
    pub name: String,
    pub version: Version,
    pub uri: String,
    pub commit: Option<String>,
}

#[derive(Serialize)]
pub struct LockFile {
    pub version: u8,
    pub packages: Vec<LockPackage>,
}