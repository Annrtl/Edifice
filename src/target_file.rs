use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct TargetFile {
    pub path: PathBuf,
}

impl TargetFile {
    pub fn save(&self) -> Result<(), String> {
        match std::fs::write(&self.path, "") {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to create file {}: {}", self.path.display(), err)),
        }
    }
    
}