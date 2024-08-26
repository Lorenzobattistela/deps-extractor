use crate::parsers::{AgdaParser, HaskellParser, TypeScriptParser};
use std::path::PathBuf;
use std::hash::{Hash, Hasher};

pub struct Language {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub include_external: bool,
}

#[derive(Debug, Clone)]
pub enum Dependency {
    Local(PathBuf),
    External(String),
}


impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Dependency::Local(a), Dependency::Local(b)) => a == b,
            (Dependency::External(a), Dependency::External(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Dependency {}

impl Hash for Dependency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Dependency::Local(path) => path.hash(state),
            Dependency::External(name) => name.hash(state),
        }
    }
}

pub trait DependencyParser {
    fn parse(&self, file: &PathBuf, language: &Language) -> Result<Vec<Dependency>, String>;
}

pub struct LanguageFactory;


impl LanguageFactory {
    pub fn get_parser(lang: &str) -> Option<Box<dyn DependencyParser>> {
        match lang {
            "agda" => Some(Box::new(AgdaParser)),
            "haskell" => Some(Box::new(HaskellParser)),
            "ts" | "typescript" => Some(Box::new(TypeScriptParser)),
            _ => None,
        }
    }
}
