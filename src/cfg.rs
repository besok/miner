use std::io::ErrorKind;
use std::path::PathBuf;
use digraph_rs::DiGraph;
use digraph_rs::visualizer::dot::ToStringProcessor;
use sv_parser::Error;
use crate::cfg::structs::{NId, Node};

mod processor;
mod structs;

#[derive(Debug)]
pub struct CFGError {
    cause: String,
}


impl CFGError {
    fn cause(txt: &str) -> CFGError {
        CFGError { cause: txt.to_string() }
    }
}

impl From<std::io::Error> for CFGError {
    fn from(value: std::io::Error) -> Self {
        CFGError { cause: value.to_string() }
    }
}


type CFGStep<T = ()> = Result<T, CFGError>;

struct CFGraph {
    graph: DiGraph<NId, Node>,
}

impl CFGraph {
    pub fn visualize_to_file(&self, path: PathBuf) -> CFGStep<String> {
        let path = path.to_str().ok_or(CFGError::cause("the path to visualize does not exist"))?;
        Ok(self.graph.visualize().to_dot_file(path, ToStringProcessor)?)
    }
}

