use tabled::{Table, Tabled};

use crate::{
    module::parser::get_module_files,
    provider::{get_providers_modules_path, update_providers_cache},
};

/// Display the module information
#[derive(Tabled)]
struct ModuleDispay {
    name: String,
    version: String,
}

pub fn list() -> Result<(), String> {
    match update_providers_cache() {
        Ok(_) => println!("Providers cache updated"),
        Err(err) => eprintln!("Failed to update providers cache: {}", err),
    }

    let providers_modules_path = match get_providers_modules_path() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for provider_modules_path in providers_modules_path {
        let providers_module_files = match get_module_files(Some(provider_modules_path.clone())) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let table_rows: Vec<ModuleDispay> = providers_module_files
            .iter()
            .map(|module_file| ModuleDispay {
                name: module_file.module.name.clone(),
                version: module_file.module.version.to_string(),
            })
            .collect();

        let table = Table::new(&table_rows);
        println!("Modules of {}", provider_modules_path.display());
        println!("{}", table);
    }

    Ok(())
}
