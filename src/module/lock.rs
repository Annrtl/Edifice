use std::collections::HashMap;

use semver::Version;
use serde::Serialize;

#[derive(Serialize)]
struct LockFile {
    version: u8,
    packages: HashMap<String, Version>,
}

pub fn lock(modules: Vec<(String, Version)>) -> Result<(), String> {
    // Create a new lock file
    let mut lock_file = LockFile {
        version: 1,
        packages: HashMap::new(),
    };

    // Insert all resolved modules into the lock file
    for (name, version) in modules {
        lock_file.packages.insert(name, version);
    }

    // Serialize the lock file
    let content = match toml::to_string(&lock_file) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error serializing lock file: {:?}", err)),
    };

    // Write the lock file
    match std::fs::write("module.lock", content) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error writing lock file: {:?}", err)),
    }
}
