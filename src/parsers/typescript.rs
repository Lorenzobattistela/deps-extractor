use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use regex::Regex;
use crate::language::{DependencyParser, Dependency, Language};

pub struct TypeScriptParser;

impl TypeScriptParser {
    fn parse_recursive(&self, file: &PathBuf, language: &Language, visited: &mut HashSet<PathBuf>, dependencies: &mut HashSet<Dependency>) -> Result<(), String> {
        if !visited.insert(file.clone()) {
            // We've already visited this file, skip to avoid cycles
            return Ok(());
        }

        let content = fs::read_to_string(file)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let import_regex = Regex::new(r#"(?:import|export)(?:\s+type)?(?:\s+(?:[\w\s{},*]+)\s+from\s+)?['"]([^'"]+)['"]|(?:const|let|var)\s+\w+\s*=\s*require\(['"]([^'"]+)['"]\)"#)
            .map_err(|e| format!("Failed to create regex: {}", e))?;

        for cap in import_regex.captures_iter(&content) {
            let import_path = cap.get(1).or_else(|| cap.get(2)).unwrap().as_str();
            
            if import_path.starts_with('.') {
                let absolute_path = resolve_local_path(file, import_path)?;
                dependencies.insert(Dependency::Local(absolute_path.clone()));
                
                self.parse_recursive(&absolute_path, language, visited, dependencies)?;
            } else if language.include_external {
                dependencies.insert(Dependency::External(import_path.to_string()));
            }
        }

        Ok(())
    }
}

impl DependencyParser for TypeScriptParser {
    fn parse(&self, file: &PathBuf, language: &Language) -> Result<Vec<Dependency>, String> {
        let mut visited = HashSet::new();
        let mut dependencies = HashSet::new();
        self.parse_recursive(file, language, &mut visited, &mut dependencies)?;
        Ok(dependencies.into_iter().collect())
    }
}

fn resolve_local_path(current_file: &Path, import_path: &str) -> Result<PathBuf, String> {
    let parent_dir = current_file.parent()
        .ok_or_else(|| "Failed to get parent directory".to_string())?;
    
    let mut resolved_path = parent_dir.join(import_path);
    if resolved_path.extension().is_none() {
        // Try .ts, .tsx, .js, .jsx extensions
        for ext in &[".ts", ".tsx", ".js", ".jsx"] {
            resolved_path.set_extension(ext.trim_start_matches('.'));
            if resolved_path.exists() {
                break;
            }
        }
    }
    
    resolved_path.canonicalize()
        .map_err(|e| format!("Failed to resolve path: {}", e))
}
