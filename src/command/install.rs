use crate::{command::update::update, provider::git::clone_and_checkout};

/// Install source from locked packages
pub fn install() -> Result<(), String> {

    // Check if lock file exists
    let lockfile_path = std::path::Path::new("module.lock");
    
    if ! lockfile_path.exists() {
        println!("Lock file not found");
        match update() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }

    // Read lock file
    let content = match std::fs::read_to_string(lockfile_path) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error reading lock file: {:?}", err)),
    };

    // Parse lock file
    let lock_file: crate::module::LockFile = match toml::from_str(&content) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error parsing lock file: {:?}", err)),
    };

    // Create modules directory
    let modules_path = std::path::Path::new("modules");
    if ! modules_path.exists() {
        match std::fs::create_dir(modules_path) {
            Ok(_) => (),
            Err(err) => return Err(format!("Error creating modules directory: {:?}", err)),
        }
    }

    // Install packages
    for package in &lock_file.packages {
        // Create package directory
        let package_path = modules_path.join(package.name.clone());
        
        // Check if package directory exists
        if ! package_path.exists() {
            match std::fs::create_dir(&package_path) {
                Ok(_) => (),
                Err(err) => return Err(format!("Error creating package directory: {:?}", err)),
            }
        }

        println!("Prepare to download package: {}", package.name);
        // Clone package
        match clone_and_checkout(&package.uri, package_path, package.commit.clone()) {
            Ok(_) => {
                println!("Downloaded package: {}", package.name);
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
