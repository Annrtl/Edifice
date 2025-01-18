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
    pub content: Option<DatasetFileContent>,
}

impl DatasetFile {
    pub fn save(&self) -> Result<(), String> {

        let content = match toml::to_string(&self.content) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to serialize dataset file: {}", err)),
        };

        let path = self.path.clone();

        match std::fs::write(path, content) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to write dataset file: {}", err)),
        }
    }
}