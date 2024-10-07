use crate::core::parser::get_cores;

pub fn lock() {
    for core in get_cores() {
        println!("Locking {}", core.package.name);
    }
}
