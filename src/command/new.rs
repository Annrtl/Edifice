use std::{env, fs};

use semver::Version;

use crate::{dataset, modules::{module, module_file::{self}, module_file_content}, target_file};

pub fn new(name: String) -> Result<(), String> {
    
    // Check if the directory exists
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(format!("Failed to get current directory: {}", err)),
    };

    let module_dir = current_dir.join(&name);

    if module_dir.exists() {
        return Err(format!("Directory {} already exists", module_dir.display()));
    }

    // Create the directory

    match fs::create_dir(&module_dir) {
        Ok(_) => println!("Directory {} created", module_dir.display()),
        Err(err) => return Err(format!("Failed to create directory {}: {}", module_dir.display(), err)),
    }

    // Create the module file

    let modulefile_path = module_dir.join("module.toml");

    let modulefile = module_file::ModuleFile{
        path: modulefile_path,
        content: Some(module_file_content::ModuleFileContent {
            module: module::Module {
                name: name.clone(),
                version: Version::new(0, 1, 0),
            },
            dependencies: None,
            origin: None,
        }),
        datasetfile: None,
    };

    modulefile.save()?;

    // Create the dataset file

    let datasetfile_path = module_dir.join("dataset.toml");

    let datasetfile = dataset::DatasetFile{
        path: datasetfile_path,
        content: Some(dataset::DatasetFileContent {
            dataset_api: Version::new(0, 1, 0),
            dataset: None,
        }),
    };

    datasetfile.save()?;
    
    // Create the target file
    
    let targetfile_path = module_dir.join("target.toml");

    let targetfile = target_file::TargetFile{
        path: targetfile_path,
    };

    targetfile.save()?;

    // Create the src directory

    let src_dir = module_dir.join("src");

    match fs::create_dir(&src_dir) {
        Ok(_) => println!("Directory {} created", src_dir.display()),
        Err(err) => return Err(format!("Failed to create directory {}: {}", src_dir.display(), err)),
    }

    // Create Readme file

    let readme_path = module_dir.join("README.md");

    match fs::write(&readme_path, format!("# {}\n", name)) {
        Ok(_) => println!("File {} created", readme_path.display()),
        Err(err) => return Err(format!("Failed to create file {}: {}", readme_path.display(), err)),
    }
    
    // Create the gitignore file

    let gitignore_path = module_dir.join(".gitignore");

    match fs::write(&gitignore_path, "build\n") {
        Ok(_) => println!("File {} created", gitignore_path.display()),
        Err(err) => return Err(format!("Failed to create file {}: {}", gitignore_path.display(), err)),
    }

    Ok(())

}