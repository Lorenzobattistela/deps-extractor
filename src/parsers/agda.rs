use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use regex::Regex;
use crate::language::{DependencyParser, Dependency, Language};

pub struct AgdaParser;

impl AgdaParser {
    fn parse_recursive(&self, file: &PathBuf, language: &Language, visited: &mut HashSet<PathBuf>, dependencies: &mut HashSet<Dependency>) -> Result<(), String> {
        if !visited.insert(file.clone()) {
            return Ok(());
        }

        let content = fs::read_to_string(file)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let import_regex = Regex::new(r"open\s+import\s+([\w.]+)")
            .map_err(|e| format!("Failed to create regex: {}", e))?;

        for cap in import_regex.captures_iter(&content) {
            let import_path = cap.get(1).unwrap().as_str();
            
            if self.is_local_import(import_path) {
                let absolute_path = self.resolve_local_path(file, import_path)?;
                dependencies.insert(Dependency::Local(absolute_path.clone()));
                
                self.parse_recursive(&absolute_path, language, visited, dependencies)?;
            } else if language.include_external {
                dependencies.insert(Dependency::External(import_path.to_string()));
            }
        }

        Ok(())
    }

    fn is_local_import(&self, import_path: &str) -> bool {
        // This is a simplified check. You might need to adjust this based on your project structure
        !import_path.starts_with("Agda.") && !import_path.starts_with("Data.") && !import_path.starts_with("Relation.")
    }

    fn resolve_local_path(&self, current_file: &Path, import_path: &str) -> Result<PathBuf, String> {
        let parent_dir = current_file.parent()
            .ok_or_else(|| "Failed to get parent directory".to_string())?;
        
        let mut path_components: Vec<&str> = import_path.split('.').collect();
        path_components.push("agda");
        let file_name = path_components.join(".");
        
        let resolved_path = parent_dir.join(file_name);
        
        resolved_path.canonicalize()
            .map_err(|e| format!("Failed to resolve path: {}", e))
    }
}

impl DependencyParser for AgdaParser {
    fn parse(&self, file: &PathBuf, language: &Language) -> Result<Vec<Dependency>, String> {
        let mut visited = HashSet::new();
        let mut dependencies = HashSet::new();
        self.parse_recursive(file, language, &mut visited, &mut dependencies)?;
        Ok(dependencies.into_iter().collect())
    }
}
