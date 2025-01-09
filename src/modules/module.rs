use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    pub name: String,
    pub version: Version,
}
