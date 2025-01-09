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
        #[cfg(debug_assertions)]
        println!("Getting dataset file of modulefile {:?}", modulefile);

        let datasetfile = match modulefile.datasetfile {
            Some(data) => data,
            None => continue,
        };

        println!("Dataset file: {:?}", datasetfile);
    }

    #[cfg(debug_assertions)]
    println!("Built OK");

    Ok(())
}
