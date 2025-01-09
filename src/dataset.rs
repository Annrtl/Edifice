use std::{collections::HashMap, path::PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

pub mod load;
pub mod tree;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dataset {
    pub dataset_type: String,
    pub sources: Option<Vec<PathBuf>>,
    pub include_directories: Option<Vec<PathBuf>>,
    pub compilation_options: Option<Vec<String>>,
    pub prepend: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatasetFileContent {
    pub dataset_api: Version,
    pub dataset: Option<HashMap<String, Dataset>>,
}

#[derive(Clone, Debug)]
pub struct DatasetFile {
    pub path: PathBuf,
    pub content: DatasetFileContent,
}
