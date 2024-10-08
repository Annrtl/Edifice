use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Core {
    pub rule: Rule,
    pub package: Package,
}

pub fn get_cores() -> Vec<Core> {
    let files = std::fs::read_dir(".")
    .unwrap()
    .filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "toml") {
            Some(path)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    let mut cores: Vec<Core> = Vec::new();

    for file in files {
        println!("Found toml file {:?}", file);
        // Lire le contenu du fichier TOML
        let content_result: Result<String, std::io::Error> = fs::read_to_string(file);
        let content: String;

        match content_result {
            Ok(data) => {
                content = data;
            }
            Err(err) => {
                panic!("Error reading file: {:?}", err);
            }
        }

        // Désérialiser le fichier TOML en une structure Rust
        let data_str: &str = content.as_str();
        let core_result: Result<Core, toml::de::Error> = toml::from_str(data_str);
        let core: Core;

        match core_result {
            Ok(data) => {
                core = data;
            }
            Err(err) => {
                panic!("Error converting toml content to core struct: {:?}", err);
            }
        }

        cores.push(core);
    }
    cores
}