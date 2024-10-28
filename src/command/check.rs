use semver::Version;

use crate::module::dfs::Graph;
use crate::module::parser::get_module_file;
use crate::module::parser::get_module_files;
use crate::module::ModuleFile;
use crate::provider::get_providers_cache_path;
use crate::provider::update_providers_cache;

pub fn check() -> Result<Vec<(String, Version)>, String> {
    match update_providers_cache() {
        Ok(_) => println!("Providers cache updated"),
        Err(err) => eprintln!("Failed to update providers cache: {}", err),
    }

    let providers_cache_path = match get_providers_cache_path() {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let providers_module_files = match get_module_files(Some(providers_cache_path)) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    #[cfg(debug_assertions)]
    for module_file in &providers_module_files {
        println!("Module file: {:?}", module_file);
    }

    let mut graph = Graph::new();
    let mut providers_module_files_ref =
        providers_module_files.iter().collect::<Vec<&ModuleFile>>();

    let top_module = match get_module_file(None) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    #[cfg(debug_assertions)]
    println!(
        "Top module: {}:{}",
        top_module.module.name, top_module.module.version
    );

    providers_module_files_ref.push(&top_module);
    graph.loads_modules(providers_module_files_ref);

    #[cfg(debug_assertions)]
    println!("Modules loaded in the graph");

    let resolve_modules = match graph.dfs(top_module.module.name, top_module.module.version) {
        Ok(data) => data,
        Err(err) => return Err(err.concat()),
    };

    Ok(resolve_modules)
}
