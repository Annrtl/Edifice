use tabled::{Table, Tabled};

use crate::origins::get_main_origin;

/// Display the module information
#[derive(Tabled)]
struct ModuleDispay {
    name: String,
    version: String,
}

pub fn info() -> Result<(), String> {
    let mut main_origin = match get_main_origin() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let module_file = match main_origin.get_modulefile() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let modulefile_content = match module_file.content {
        Some(data) => data,
        None => return Err("Main module file has empty content".to_string()),
    };

    let table_rows: Vec<ModuleDispay> = vec![ModuleDispay {
        name: modulefile_content.module.name,
        version: modulefile_content.module.version.to_string(),
    }];

    let table = Table::new(&table_rows);
    println!("{}", table);

    Ok(())
}
