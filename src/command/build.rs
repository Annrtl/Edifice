use crate::{modules::get_modulefiles, registries::update_registries};

pub fn build() -> Result<(), String> {
    #[cfg(debug_assertions)]
    println!("Updating registries");

    match update_registries() {
        Ok(_) => println!("Registries cache updated"),
        Err(err) => eprintln!("Failed to update registries cache: {}", err),
    }

    let modulefiles = match get_modulefiles() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    for modulefile in modulefiles {
        
        let mut datasetfile = modulefile.datasetfile;
        
        if ! datasetfile.is_loaded {
            match datasetfile.load() {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Failed to load dataset file: {}", err);
                    continue;
                },
            }
        }

        #[cfg(debug_assertions)] {
            let modulefile_name = match modulefile.content {
                Some(data) => format!("{}:{}", data.module.name, data.module.version),
                None => continue,
            };
            println!("Dataset of module {}: {}", modulefile_name, datasetfile.path.display());
        }

    }

    #[cfg(debug_assertions)]
    println!("Built OK");

    Ok(())
}
