use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dataset {
    pub dataset_type: String,
    pub sources: Option<Vec<PathBuf>>,
    pub include_directories: Option<Vec<PathBuf>>,
    pub compilation_options: Option<Vec<String>>,
    pub prepend: Option<Vec<String>>,
}