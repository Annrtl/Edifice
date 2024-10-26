use semver::{Version, VersionReq};

pub mod parser;
pub mod dfs;

#[derive(Debug)]
pub struct Module {
    name: String,
    version: Version,
    requirements: Vec<Requirement>,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    module: String,
    constraint: VersionReq,
}