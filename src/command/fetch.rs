use crate::provider::update_providers_cache;

pub fn fetch() -> Result<(), String> {
    update_providers_cache()
}
