use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde::Deserialize;

pub mod dfs;
pub mod parser;

#[derive(Deserialize, Debug)]
struct Rule {
    pub name: String,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct ModuleFile {
    pub rule: Rule,
    pub module: Module,
    pub dependencies: HashMap<String, VersionReq>,
}
