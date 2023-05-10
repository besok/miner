use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use sv_parser::{Defines, Error, parse_lib, parse_sv, SyntaxTree};

#[derive(Debug)]
pub enum ParserError {
    FileError(std::io::Error),
    ParseError(Error),
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        ParserError::FileError(value)
    }
}


impl From<Error> for ParserError {
    fn from(value: Error) -> Self {
        ParserError::ParseError(value)
    }
}

pub struct Ast {
    path: PathBuf,
    tree: SyntaxTree,
    defines: Defines,
}

impl Ast {
    pub fn new(path: PathBuf, tree: SyntaxTree, defines: Defines) -> Self {
        Self { path, tree, defines }
    }
}

pub fn parse(path: PathBuf, included: Vec<PathBuf>) -> Result<Ast, ParserError> {
    let mut paths: Vec<PathBuf> = included;

    let root = if path.is_dir() {
        path.clone()
    } else {
        path.parent()
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "The parent for the file is should be presented"))?
            .to_path_buf()
    };

    if !paths.contains(&root) {
        paths.push(root);
    }

    let (tree, defines) = parse_sv(
        path.clone(),
        &HashMap::new(),
        &paths,
        false, false,
    )?;

    Ok(Ast::new(path, tree, defines))
}

pub fn parse_root(path: PathBuf) -> Result<Ast, ParserError> {
    parse(path, vec![])
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;
    use crate::parser::{Ast, parse, parse_root};

    pub fn load(path: &str) -> Ast {
        let mut ex = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        for next in path.split("//") {
            match next {
                ".." => ex = ex.parent().unwrap().into(),
                n => ex.push(n),
            }
        }
        parse_root(ex).unwrap()
    }

    #[test]
    fn smoke() {
        let ast = load("examples/smoke/CacheFlushManager.sv");
        println!("{:?}", ast.tree);
        println!("----- ----");
        println!("{:?}", ast.defines);
    }
}