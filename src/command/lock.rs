use crate::module::parser::get_module_file;

pub fn lock() {
    let module_file = get_module_file(None);
    println!("Locking {}", module_file.module.name);
}
