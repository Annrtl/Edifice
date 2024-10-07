use tabled::{Tabled, Table};

use crate::core::parser::get_cores;

#[derive(Tabled)]
struct CoreDispay {
    name: String,
    version: String,
}

pub fn show() {
    let cores = get_cores();

    let mut table_rows: Vec<CoreDispay> = Vec::new();

    for core in cores {
        table_rows.push(CoreDispay {
            name: core.package.name,
            version: core.package.version,
        });
    }

    let table = Table::new(&table_rows);
    println!("{}", table);
}
