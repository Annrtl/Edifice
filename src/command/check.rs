use semver::Version;

use crate::origins::get_main_origin;

/// Check the satisfiability of the module
pub fn check() -> Result<Vec<(String, Version)>, String> {
    let mut main_origin = match get_main_origin() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let module_file = match main_origin.get_modulefile() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let resolve_modules = match module_file.solve() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for module in &resolve_modules {
        println!("Module: {}:{} is resolved", module.0, module.1);
    }

    Ok(resolve_modules)
}
