use std::{collections::HashMap, path::PathBuf};

use semver::VersionReq;

use crate::module::parser::get_module_file;

/// Add a module to the project
pub fn add(module: String, dry_run: bool) -> Result<(), String> {
    let mut module_file = match get_module_file(None) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Get subsection of the module
    let module_sections = module.split("@").collect::<Vec<&str>>();

    // Get the module name
    let module_name = module_sections[0];

    // Check if module has a version
    let module_version = match module_sections.len() > 1 {
        true => match VersionReq::parse(module_sections[1]) {
            Ok(data) => data,
            Err(_) => return Err("Invalid version".to_string()),
        },
        false => VersionReq::STAR,
    };

    // Add dependency to the module file
    let mut dependencies = module_file.dependencies.unwrap_or(HashMap::new());
    dependencies.insert(module_name.to_string(), module_version);
    module_file.dependencies = Some(dependencies);

    let resolved_modules = match module_file.solve() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Get the resolved version of the module
    let module_version = match resolved_modules
        .iter()
        .find(|(name, _)| name == module_name)
    {
        Some((_, version)) => version,
        None => return Err("Module not resolved".to_string()),
    };

    // If dry run is enabled, display the resolved version
    if dry_run {
        println!(
            "Resolved version of module {}: {}",
            module_name, module_version
        );
        return Ok(());
    }

    // Update the module file with the resolved version
    let module_version_req = match VersionReq::parse(&module_version.to_string()) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error parsing version: {:?}", err)),
    };

    // Update dependency version
    let mut dependencies = module_file.dependencies.unwrap_or(HashMap::new());
    dependencies.remove(module_name);
    dependencies.insert(module_name.to_string(), module_version_req);
    module_file.dependencies = Some(dependencies);

    // Overwrite the module file
    let content = match toml::to_string(&module_file) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error serializing module file: {:?}", err)),
    };

    let module_file_path = PathBuf::from("module.toml");

    match std::fs::write(module_file_path, content) {
        Ok(_) => (),
        Err(err) => return Err(format!("Error writing module file: {:?}", err)),
    }

    Ok(())
}
