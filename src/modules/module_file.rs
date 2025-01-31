use std::{fs, path::PathBuf};

use semver::Version;

use crate::{
    dataset_file::DatasetFile, modules::{dfs::Graph, get_modulefiles}, registries::update_registries
};

use super::module_file_content::ModuleFileContent;

#[derive(Clone, Debug)]
pub struct ModuleFile {
    pub path: PathBuf,
    pub is_loaded: bool,
    pub content: Option<ModuleFileContent>,
    pub datasetfile: DatasetFile,
}

impl ModuleFile {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        // Get parent directory of the module file
        let parent_dir = match path.parent() {
            Some(data) => data,
            None => return Err("Error getting parent directory".to_string()),
        };

        // Get dataset file path
        let datasetfile_path = parent_dir.join("dataset.toml");

        // Create a new dataset file
        let datasetfile = DatasetFile {
            path: datasetfile_path,
            is_loaded: false,
            content: None,
        };

        // Create a new module file
        let mut modulefile = ModuleFile {
            path,
            is_loaded: false,
            content: None,
            datasetfile: datasetfile,
        };

        // Load the module file
        match modulefile.load() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        Ok(modulefile)
    }

    fn load(&mut self) -> Result<(), String> {
        // Get path string (To be print in error messages)
        let path_str = match self.path.to_str() {
            Some(data) => data,
            None => return Err("Error getting path string".to_string()),
        };

        // Read the module TOML content
        let content = match fs::read_to_string(self.path.clone()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error reading modulefile {}: {}", path_str, err)),
        };

        // Unserialize the TOML file into struct
        let module_file_content = match toml::from_str::<ModuleFileContent>(content.as_str()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error deserializing module file: {:?}", err)),
        };

        // Set the content of the module file
        self.content = Some(module_file_content);

        // Get modulefile parent directory
        let parent_dir = match self.path.parent() {
            Some(data) => data,
            None => return Err("Error getting parent directory".to_string()),
        };

        // Get dataset file path
        let datasetfile_path = parent_dir.join("dataset.toml");

        // Check if the dataset file exists
        if datasetfile_path.exists() {
            // Load the dataset file
            let datasetfile = DatasetFile::new(datasetfile_path);

            // Set the dataset file
            self.datasetfile = datasetfile;
        }

        // Set the is_loaded flag
        self.is_loaded = true;

        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        let path_str = match self.path.to_str() {
            Some(data) => data,
            None => return Err("Error getting path string".to_string()),
        };

        // Sérialiser la structure Rust en fichier TOML
        let content = match toml::to_string_pretty(&self.content) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error serializing module file: {:?}", err)),
        };

        // Écrire le contenu dans le fichier
        match fs::write(self.path.clone(), content) {
            Ok(_) => (),
            Err(err) => return Err(format!("Error writing module file {}: {}", path_str, err)),
        }

        Ok(())
    }

    pub fn solve(&self) -> Result<Vec<(String, Version)>, String> {
        match update_registries() {
            Ok(_) => println!("Registries cache updated"),
            Err(err) => eprintln!("Failed to update registries cache: {}", err),
        }

        let registries_module_files = match get_modulefiles() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        #[cfg(debug_assertions)]
        for module_file in &registries_module_files {
            println!("Module file: {:?}", module_file);
        }

        let mut graph = Graph::new();
        let mut registries_module_files_ref =
            registries_module_files.iter().collect::<Vec<&ModuleFile>>();

        let content = match &self.content {
            Some(data) => data,
            None => return Err("Module file content is empty".to_string()),
        };

        #[cfg(debug_assertions)]
        println!(
            "Top module: {}:{}",
            content.module.name, content.module.version
        );

        registries_module_files_ref.push(self);
        graph.loads_modules(registries_module_files_ref);

        #[cfg(debug_assertions)]
        println!("Modules loaded in the graph");

        let resolve_modules =
            match graph.dfs(content.module.name.clone(), content.module.version.clone()) {
                Ok(data) => data,
                Err(err) => return Err(err),
            };

        #[cfg(debug_assertions)]
        for module in &resolve_modules {
            println!("Module: {}:{} is resolved", module.0, module.1);
        }

        Ok(resolve_modules)
    }
}
