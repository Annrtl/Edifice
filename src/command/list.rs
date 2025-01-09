use regex::Regex;
use tabled::{Table, Tabled};

use crate::{
    modules::module_file_content::ModuleFileContent,
    origins::get_main_origin,
    registries::{get_registries, update_registries},
};

/// Display the module information
#[derive(Tabled)]
struct ModuleDispay {
    name: String,
    version: String,
}

pub fn list(regex_pattern: Option<String>) -> Result<(), String> {
    match update_registries() {
        Ok(_) => println!("Registries cache updated"),
        Err(err) => eprintln!("Failed to update registries cache: {}", err),
    }

    let regex_pattern = match regex_pattern {
        Some(data) => data,
        None => ".*".to_string(),
    };

    let pattern = match Regex::new(&regex_pattern) {
        Ok(data) => data,
        Err(err) => {
            return Err(format!("Failed to create regex: {}", err));
        }
    };

    let registries = match get_registries() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for registry in registries {
        #[cfg(debug_assertions)]
        println!("Analysing registry: {}", registry.uri);

        let registry_modulefiles = match registry.get_modulefiles() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let mut modulefiles_content: Vec<ModuleFileContent> = Vec::new();

        for modulefile in registry_modulefiles {
            #[cfg(debug_assertions)]
            println!("Analysing modulefile: {}", modulefile.path.display());

            modulefiles_content.push(match modulefile.content {
                Some(data) => data,
                None => {
                    return Err("Module file content is empty".to_string());
                }
            });
        }

        let table_rows: Vec<ModuleDispay> = modulefiles_content
            .iter()
            .map(|content| ModuleDispay {
                name: content.module.name.clone(),
                version: content.module.version.to_string(),
            })
            .filter(|content| pattern.is_match(&content.name))
            .collect();

        let table = Table::new(&table_rows);
        println!("Modules of {}", registry.uri);
        println!("{}", table);
    }

    let mut main_origin = match get_main_origin() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let main_modulefile = match main_origin.get_modulefile() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let main_modulefile_content = match main_modulefile.content {
        Some(data) => data,
        None => {
            return Err("Module file content is empty".to_string());
        }
    };

    let table_row = ModuleDispay {
        name: main_modulefile_content.module.name.clone(),
        version: main_modulefile_content.module.version.to_string(),
    };

    if pattern.is_match(&table_row.name) {
        let table = Table::new(&[table_row]);
        println!("Modules of {}", main_origin.uri);
        println!("{}", table);
    }

    Ok(())
}
