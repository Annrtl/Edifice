use std::path::PathBuf;

use super::{DatasetFile, DatasetFileContent};

impl DatasetFile {
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let content = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to read file: {}", err)),
        };

        let dataset_file_content: DatasetFileContent = match toml::from_str(&content) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to parse file: {}", err)),
        };

        let dataset_file = DatasetFile {
            path: path.clone(),
            content: dataset_file_content,
        };

        Ok(dataset_file)
    }
}
