use semver::Version;

use crate::{module::dfs::Graph, provider::{get_providers_modules, update_providers_cache}};

use super::ModuleFile;

impl ModuleFile {
    pub fn solve(&self) -> Result<Vec<(String, Version)>, String> {
        match update_providers_cache() {
            Ok(_) => println!("Providers cache updated"),
            Err(err) => eprintln!("Failed to update providers cache: {}", err),
        }
    
        let providers_module_files = match get_providers_modules() {
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
    
        #[cfg(debug_assertions)]
        println!(
            "Top module: {}:{}",
            self.module.name, self.module.version
        );
    
        providers_module_files_ref.push(self);
        graph.loads_modules(providers_module_files_ref);
    
        #[cfg(debug_assertions)]
        println!("Modules loaded in the graph");
    
        let resolve_modules = match graph.dfs(self.module.name.clone(), self.module.version.clone()) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
    
        for module in &resolve_modules {
            println!("Module: {}:{} is resolved", module.0, module.1);
        }
    
        Ok(resolve_modules)
    }
}