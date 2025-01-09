use std::path::PathBuf;

use super::lock_file_content::LockFileContent;

pub struct LockFile {
    pub path: PathBuf,
    pub content: LockFileContent,
}
