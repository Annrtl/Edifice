use tabled::{Table, Tabled};

use crate::module::parser::get_module;

#[derive(Tabled)]
struct ModuleDispay {
    name: String,
    version: String,
}

pub fn show() {
    let module_file = get_module();

    let table_rows: Vec<ModuleDispay> = vec![ModuleDispay {
        name: module_file.module.name,
        version: module_file.module.version,
    }];

    let table = Table::new(&table_rows);
    println!("{}", table);
}
