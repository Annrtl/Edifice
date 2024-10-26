use crate::provider::git::update_cache;

pub fn fetch() -> Result<(), String> {
    update_cache()
}
