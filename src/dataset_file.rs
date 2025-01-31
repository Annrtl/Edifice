use std::path::PathBuf;

use crate::dataset_file_content::DatasetFileContent;

#[derive(Clone, Debug)]
pub struct DatasetFile {
    pub path: PathBuf,
    pub is_loaded: bool,
    pub content: Option<DatasetFileContent>,
}

impl DatasetFile {
    pub fn new(path: PathBuf) -> Self {
        // Create a new dataset file
        DatasetFile {
            path,
            is_loaded: false,
            content: None,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        // Serialize the dataset file content
        let content = match toml::to_string(&self.content) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to serialize dataset file: {}", err)),
        };

        // Write the dataset file content
        match std::fs::write(self.path.clone(), content) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to write dataset file: {}", err)),
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        // Check if the dataset file exists
        if ! self.path.exists() {
            return Err(format!("DatasetFile does not exist: {}", self.path.display()));
        }

        // Read the dataset file content
        let content = match std::fs::read_to_string(self.path.clone()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to read DatasetFile: {}", err)),
        };

        // Unserialize the dataset file into struct
        let dataset_file_content: DatasetFileContent = match toml::from_str(&content) {
            Ok(data) => data,
            Err(err) => return Err(format!("Failed to parse DatasetFile: {}", err)),
        };

        // Set the content of the dataset file
        self.content = Some(dataset_file_content);

        // Set the is_loaded flag
        self.is_loaded = true;

        Ok(())
    }
}