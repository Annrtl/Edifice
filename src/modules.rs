use module_file::ModuleFile;

use crate::{origins::get_main_origin, registries::get_registries};

pub mod dfs;
pub mod lock;
pub mod lock_file;
pub mod lock_file_content;
pub mod lock_package;
pub mod module;
pub mod module_file;
pub mod module_file_content;

pub fn get_modulefiles() -> Result<Vec<ModuleFile>, String> {
    let registries = match get_registries() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut modulefiles: Vec<ModuleFile> = Vec::new();

    for registry in registries {
        let registry_modulefiles = match registry.get_modulefiles() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        modulefiles.extend(registry_modulefiles);
    }

    let mut main_origin = match get_main_origin() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let main_modulefile = match main_origin.get_modulefile() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    modulefiles.push(main_modulefile);

    Ok(modulefiles)
}
