mod cli;
mod language;
mod parsers;

use crate::cli::parse_cli;
use crate::language::{Language, LanguageFactory};

fn main() {
    let args = parse_cli();

    let language = Language {
        name: args.lang.clone(),
        file_extensions: vec![format!(".{}", args.lang)],
        include_external: args.include_external,
    };

    let parser = match LanguageFactory::get_parser(&args.lang) {
        Some(p) => p,
        None => {
            eprintln!("Unsupported language: {}", args.lang);
            std::process::exit(1);
        }
    };

    match parser.parse(&args.file, &language) {
        Ok(deps) => {
            for dep in deps {
                println!("{:?}", dep);
            }
        }
        Err(e) => {
            eprintln!("Error parsing dependencies: {}", e);
            std::process::exit(1);
        }
    }
}

