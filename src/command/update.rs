use crate::module::lock::lock;
use crate::module::parser::get_module_files;
use crate::module::{LockPackage, ModuleFile};
use crate::provider::get_providers_sync_paths;

use super::check::check;

/// Build the graph from scratch and lock the module
pub fn update() -> Result<(), String> {
    println!("Running Update");

    let modules = match check() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    println!("Modules: {:?}", modules);

    let mut packages: Vec<LockPackage> = Vec::new();

    let providers_sync_paths = match get_providers_sync_paths() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut provided_modules: Vec<ModuleFile> = Vec::new();

    for provider_sync_path in providers_sync_paths {
        let modules_files = match get_module_files(Some(provider_sync_path)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        for module_file in modules_files {
            provided_modules.push(module_file);
        }
    }

    for module in provided_modules {
        println!("{}:{}", module.module.name, module.module.version);
        if !modules.contains(&(module.module.name.clone(), module.module.version.clone())) {
            println!("Skipped");
            continue;
        }

        let package_name = module.module.name;
        let package_version = module.module.version;
        let package_uri = match module.provider {
            Some(ref data) => data.uri.clone(),
            None => String::from(""),
        };
        let package_commit = match module.provider {
            Some(data) => data.commit,
            None => Some("".to_string()),
        };

        packages.push(LockPackage {
            name: package_name,
            version: package_version,
            uri: package_uri,
            commit: package_commit,
        });
    }

    match lock(packages) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
