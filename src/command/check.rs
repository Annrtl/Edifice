use semver::Version;

use crate::module::parser::get_module_file;

/// Check the satisfiability of the module
pub fn check() -> Result<Vec<(String, Version)>, String> {
    let top_module = match get_module_file(None) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let resolve_modules = match top_module.solve() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for module in &resolve_modules {
        println!("Module: {}:{} is resolved", module.0, module.1);
    }

    Ok(resolve_modules)
}
