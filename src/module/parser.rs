use glob::glob;
use std::{fs, path::PathBuf};

use crate::module::ModuleFile;

pub fn get_module_file(path: Option<PathBuf>) -> Result<ModuleFile, String> {
    let module_file_path = match path {
        Some(path) => path,
        None => PathBuf::from("module.toml"),
    };

    let module_file_path_path = module_file_path.clone();

    let module_file_path_str = match module_file_path_path.to_str() {
        Some(data) => data,
        None => return Err("Error getting path string".to_string()),
    };

    // Lire le contenu du fichier TOML
    let content = match fs::read_to_string(module_file_path) {
        Ok(data) => data,
        Err(err) => {
            return Err(format!(
                "Error reading file {}: {}",
                module_file_path_str, err
            ))
        }
    };

    // Désérialiser le fichier TOML en une structure Rust
    let module_file = match toml::from_str::<ModuleFile>(content.as_str()) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error deserializing module file: {:?}", err)),
    };

    Ok(module_file)
}

pub fn get_module_files(path: Option<PathBuf>) -> Result<Vec<ModuleFile>, String> {
    let module_files_path = match path {
        Some(path) => path,
        None => PathBuf::from("./"),
    };

    let module_files_path = match module_files_path.to_str() {
        Some(data) => data,
        None => return Err("Error getting path string".to_string()),
    };

    let mut module_files = Vec::new();

    #[cfg(debug_assertions)]
    println!("Module files path: {}", module_files_path);

    // Chercher les fichiers module.toml dans le répertoire courant
    let glob_pattern = format!("{}/**/module.toml", module_files_path);
    for entry in glob(glob_pattern.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let module_file = match get_module_file(Some(path)) {
                    Ok(data) => data,
                    Err(err) => return Err(err),
                };
                module_files.push(module_file);
            }
            Err(err) => return Err(format!("Error reading glob entry: {:?}", err)),
        }
    }

    Ok(module_files)
}
