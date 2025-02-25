use std::{collections::HashMap, path::PathBuf};

use semver::VersionReq;

use crate::origins::get_main_origin;

/// Add a module to the project
pub fn add(module: String, dry_run: bool) -> Result<(), String> {
    // Get the module file
    let mut main_origin = match get_main_origin() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut module_file = match main_origin.get_modulefile() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Get subsection of the module
    let module_sections = module.split("@").collect::<Vec<&str>>();

    // Get the module name
    let module_name = module_sections[0];

    // Check if module has a version
    let module_version = match module_sections.len() > 1 {
        true => match VersionReq::parse(format!("={}", module_sections[1]).as_str()) {
            Ok(data) => data,
            Err(_) => return Err("Invalid version".to_string()),
        },
        false => VersionReq::STAR,
    };

    // Add dependency to the module file
    let mut modulefile_content = match module_file.clone().content {
        Some(data) => data,
        None => return Err("Module file content not found".to_string()),
    };

    let mut dependencies = modulefile_content.dependencies.unwrap_or(HashMap::new());
    dependencies.insert(module_name.to_string(), module_version);
    modulefile_content.dependencies = Some(dependencies);

    module_file.content = Some(modulefile_content.clone());

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
    let mut dependencies = modulefile_content.dependencies.unwrap_or(HashMap::new());
    dependencies.remove(module_name);
    dependencies.insert(module_name.to_string(), module_version_req);
    modulefile_content.dependencies = Some(dependencies);

    // Overwrite the module file
    let content = match toml::to_string(&modulefile_content) {
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
