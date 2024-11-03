use super::{LockFile, LockPackage};

pub fn lock(packages: Vec<LockPackage>) -> Result<(), String> {
    // Create a new lock file
    let mut lock_file = LockFile {
        version: 1,
        packages: Vec::new(),
    };

    // Insert all resolved modules into the lock file
    for package in packages {
        lock_file.packages.push(package);
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
