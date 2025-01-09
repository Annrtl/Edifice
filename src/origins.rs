use origin::Origin;

pub mod origin;

pub fn get_main_origin() -> Result<Origin, String> {
    let current_dir = match std::env::current_dir() {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    let origin = Origin {
        uri: "origin".to_string(),
        commit: None,
        cache_path: Some(current_dir),
    };

    Ok(origin)
}
