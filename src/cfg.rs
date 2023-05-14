use digraph_rs::DiGraph;
use crate::cfg::structs::Node;

mod builder;
mod structs;

#[derive(Debug)]
pub struct CFGError {
    cause: String,
}

impl CFGError {
    fn cause(txt:&str) -> CFGError{
        CFGError{ cause:txt.to_string() }
    }
}

type CFGStep<T=()> = Result<T,CFGError>;

struct CFGraph {
    graph: DiGraph<usize, Node>,
}

