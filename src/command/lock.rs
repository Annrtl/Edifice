use crate::module::parser::get_module;

pub fn lock() {
    let module_file = get_module();
    println!("Locking {}", module_file.module.name);
}
