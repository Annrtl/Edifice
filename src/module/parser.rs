use std::{collections::HashMap, fs, iter::Map};
use glob::glob;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct ModuleFile {
    pub rule: Rule,
    pub module: Module,
    pub dependencies: HashMap<String, String>,
}

pub fn get_module() -> ModuleFile
 {
    let module_file = match glob("module.toml") {
        Ok(files) => files,
        Err(_) => panic!("Error reading module.toml"),
    };

    //println!("Found toml file {:?}", module_file);

    let file = match module_file.into_iter().next() {
        Some(file) => match file {
            Ok(file) => file,
            Err(_) => panic!("Error reading file"),
        },
        None => panic!("No module.toml found"),    
    };

    println!("Found toml file {:?}", file);
    
    // Lire le contenu du fichier TOML
    let content = match fs::read_to_string(file) {
        Ok(data) => data,
        Err(err) => panic!("Error reading file: {:?}", err),
    };

    // Désérialiser le fichier TOML en une structure Rust
    let module_file = match toml::from_str(&content) {
        Ok(module_file) => module_file,
        Err(err) => panic!("Error converting toml content to module_file struct: {:?}", err.message()),
    };

    module_file

}