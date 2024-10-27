use glob::glob;
use std::{fs, path::PathBuf};

use crate::module::ModuleFile;

pub fn get_module_file(path: Option<PathBuf>) -> ModuleFile {
    let module_file_path = match path {
        Some(path) => path,
        None => PathBuf::from("module.toml"),
    };

    // Lire le contenu du fichier TOML
    let content = match fs::read_to_string(module_file_path) {
        Ok(data) => data,
        Err(err) => panic!("Error reading file: {:?}", err),
    };

    // Désérialiser le fichier TOML en une structure Rust
    let module_file = match toml::from_str(&content) {
        Ok(module_file) => module_file,
        Err(err) => panic!(
            "Error converting toml content to module_file struct: {:?}",
            err.message()
        ),
    };

    module_file
}

pub fn get_module_files(path: Option<PathBuf>) -> Vec<ModuleFile> {
    let module_files_path = match path {
        Some(path) => path,
        None => PathBuf::from("./"),
    };

    let module_files_path = match module_files_path.canonicalize() {
        Ok(data) => data,
        Err(err) => panic!("Error reading file: {:?}", err),
    };

    let module_files_path = match module_files_path.to_str() {
        Some(data) => data,
        None => panic!("Error reading file: {:?}", "Error reading file"),
    };

    let mut module_files = Vec::new();

    // Chercher les fichiers module.toml dans le répertoire courant
    let glob_pattern = format!("{}/**/module.toml", module_files_path);
    for entry in glob(glob_pattern.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let module_file = get_module_file(Some(path));
                module_files.push(module_file);
            }
            Err(err) => panic!("Error reading file: {:?}", err),
        }
    }

    module_files
}
