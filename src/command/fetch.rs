use crate::provider::update_providers_cache;

/// Update the providers cache directory
pub fn fetch() -> Result<(), String> {
    update_providers_cache()
}
