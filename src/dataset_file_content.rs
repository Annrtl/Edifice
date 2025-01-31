use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::dataset::Dataset;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatasetFileContent {
    pub dataset_api: Version,
    pub dataset: Option<HashMap<String, Dataset>>,
}
