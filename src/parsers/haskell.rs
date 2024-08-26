use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use regex::Regex;
use crate::language::{DependencyParser, Dependency, Language};

pub struct HaskellParser;

impl HaskellParser {
    fn parse_recursive(&self, file: &PathBuf, language: &Language, visited: &mut HashSet<PathBuf>, dependencies: &mut HashSet<Dependency>) -> Result<(), String> {
        if !visited.insert(file.clone()) {
            return Ok(());
        }

        let content = fs::read_to_string(file)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let import_regex = Regex::new(r"(?m)^\s*import\s+(qualified\s+)?([\w.]+)")
            .map_err(|e| format!("Failed to create regex: {}", e))?;

        for cap in import_regex.captures_iter(&content) {
            let import_path = cap.get(2).unwrap().as_str();
            
            match self.resolve_local_path(file, import_path) {
                Ok(absolute_path) => {
                    dependencies.insert(Dependency::Local(absolute_path.clone()));
                    self.parse_recursive(&absolute_path, language, visited, dependencies)?;
                },
                Err(_) => {
                    if language.include_external {
                        dependencies.insert(Dependency::External(import_path.to_string()));
                    }
                }
            }
        }

        Ok(())
    }

    fn resolve_local_path(&self, current_file: &Path, import_path: &str) -> Result<PathBuf, String> {
        let path_components: Vec<&str> = import_path.split('.').collect();
        let mut possible_paths = vec![];

        // Start from the current file's directory and its ancestors
        let mut current_dir = current_file.parent().unwrap().to_path_buf();
        loop {
            possible_paths.push(current_dir.clone());
            if !current_dir.pop() {
                break;
            }
        }

        for start_path in possible_paths {
            // Try the dot-separated path
            let mut dot_path = start_path.clone();
            for component in &path_components {
                dot_path.push(component);
            }
            dot_path.set_extension("hs");
            if dot_path.exists() {
                return Ok(dot_path.canonicalize().map_err(|e| format!("Failed to canonicalize path: {}", e))?);
            }

            // Try the directory structure
            let mut dir_path = start_path;
            for (i, component) in path_components.iter().enumerate() {
                dir_path.push(component);
                if i == path_components.len() - 1 {
                    dir_path.set_extension("hs");
                    if dir_path.exists() {
                        return Ok(dir_path.canonicalize().map_err(|e| format!("Failed to canonicalize path: {}", e))?);
                    }
                }
            }
        }

        Err(format!("Could not resolve local path for import: {}", import_path))
    }
}

impl DependencyParser for HaskellParser {
    fn parse(&self, file: &PathBuf, language: &Language) -> Result<Vec<Dependency>, String> {
        let mut visited = HashSet::new();
        let mut dependencies = HashSet::new();
        self.parse_recursive(file, language, &mut visited, &mut dependencies)?;
        Ok(dependencies.into_iter().collect())
    }
}
