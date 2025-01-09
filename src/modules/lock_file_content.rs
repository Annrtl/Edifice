use serde::{Deserialize, Serialize};

use super::lock_package::LockPackage;

#[derive(Serialize, Deserialize)]
pub struct LockFileContent {
    pub version: u8,
    pub packages: Vec<LockPackage>,
}
