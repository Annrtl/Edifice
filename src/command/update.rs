use crate::module::lock::lock;

use super::check::check;

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
