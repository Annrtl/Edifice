use crate::modules::lock::lock;
use crate::modules::lock_package::LockPackage;
use crate::modules::module_file_content::ModuleFileContent;
use crate::registries::get_registries;

use super::check::check;

/// Build the graph from scratch and lock the module
pub fn update() -> Result<(), String> {
    // Check the current requirement are working
    let current_modules = match check() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Create a vector to store the modules
    let mut selected_modules: Vec<LockPackage> = Vec::new();

    let registries = match get_registries() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for registry in registries {
        let registry_modulefiles = match registry.get_modulefiles() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let mut modulefiles_content: Vec<ModuleFileContent> = Vec::new();

        for modulefile in registry_modulefiles {
            let modulefile_content = match modulefile.content {
                Some(data) => data,
                None => {
                    return Err("Module file content is empty".to_string());
                }
            };
            modulefiles_content.push(modulefile_content);
        }

        for modulefile_content in modulefiles_content {
            if !current_modules.contains(&(
                modulefile_content.module.name.clone(),
                modulefile_content.module.version.clone(),
            )) {
                println!("Skipped");
                continue;
            }

            let package_name = modulefile_content.module.name;
            let package_version = modulefile_content.module.version;
            let package_uri = match modulefile_content.origin {
                Some(ref data) => data.uri.clone(),
                None => String::from(""),
            };
            let package_commit = match modulefile_content.origin {
                Some(data) => data.commit,
                None => Some("".to_string()),
            };

            selected_modules.push(LockPackage {
                name: package_name,
                version: package_version,
                uri: package_uri,
                commit: package_commit,
            });
        }
    }

    match lock(selected_modules) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
