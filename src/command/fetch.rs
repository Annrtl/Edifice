use crate::registries::update_registries;

/// Update the registries cache directory
pub fn fetch() -> Result<(), String> {
    update_registries()
}
