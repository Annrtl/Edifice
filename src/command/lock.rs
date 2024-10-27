use crate::module::parser::get_module_file;

pub fn lock() -> Result<(), String> {
    let module_file = match get_module_file(None) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };
    println!("Locking {}", module_file.module.name);

    Ok(())
}
