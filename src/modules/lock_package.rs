use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LockPackage {
    pub name: String,
    pub version: Version,
    pub uri: String,
    pub commit: Option<String>,
}
