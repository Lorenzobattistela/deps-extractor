
use std::path::PathBuf;

pub struct Language {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub include_external: bool,
}

#[derive(Debug)]
pub enum Dependency {
    Local(PathBuf),
    External(String),
}

pub trait DependencyParser {
    fn parse(&self, file: &PathBuf, language: &Language) -> Result<Vec<Dependency>, String>;
}

pub struct LanguageFactory;


impl LanguageFactory {
    pub fn get_parser(lang: &str) -> Option<Box<dyn DependencyParser>> {
        match lang {
            _ => None,
        }
    }
}
