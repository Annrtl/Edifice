use crate::module::lock::lock;

use super::check::check;

/// Build the graph from scratch and lock the module
pub fn update() -> Result<(), String> {
    let modules = match check() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    match lock(modules) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
