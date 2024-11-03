use crate::module::parser::get_module_files;
use crate::module::LockPackage;
use crate::module::lock::lock;
use crate::provider::get_providers_cache_path;

use super::check::check;

/// Build the graph from scratch and lock the module
pub fn update() -> Result<(), String> {
    let modules = match check() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut packages: Vec<LockPackage> = Vec::new();

    let providers_cache_path = match get_providers_cache_path() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let provided_modules = match get_module_files(Some(providers_cache_path)) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for module in provided_modules {
        println!("{}:{}", module.module.name, module.module.version);
        if ! modules.contains(&(module.module.name.clone(), module.module.version.clone())) {
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
