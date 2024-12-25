use tabled::{Table, Tabled};

use crate::module::parser::get_module_file;

/// Display the module information
#[derive(Tabled)]
struct ModuleDispay {
    name: String,
    version: String,
}

pub fn info() -> Result<(), String> {
    let module_file = match get_module_file(None) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let table_rows: Vec<ModuleDispay> = vec![ModuleDispay {
        name: module_file.module.name,
        version: module_file.module.version.to_string(),
    }];

    let table = Table::new(&table_rows);
    println!("{}", table);

    Ok(())
}
