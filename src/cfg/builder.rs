mod module;

use std::collections::{HashMap, HashSet};
use digraph_rs::DiGraph;
use crate::cfg::{CFGError, CFGraph, CFGStep, Node};
use crate::parser::Ast;

trait BuilderFn<T> {
    fn apply(builder: &mut Builder, component:T) -> CFGStep<&mut Builder>;
}

struct Builder {
    generator: usize,
    ctx_stack: Vec<usize>,
    cfg: CFGraph,
}

impl Builder {
    fn build(mut self, ast:Ast) -> CFGStep<CFGraph>{
        Ok(self.cfg)
    }
    fn create() -> Builder {
        Builder {
            generator: 0,
            ctx_stack: vec![],
            cfg: CFGraph { graph: DiGraph::new() },
        }
    }
}

impl Builder {
    fn next(&mut self) -> usize {
        self.generator += 1;
        self.generator
    }
    fn last(&self) -> usize {
        self.generator
    }

    fn add_node(&mut self, node: Node) -> CFGStep {
        let id = &self.next();
        self.cfg.graph.add_node(id.clone(), node);
        Ok(())
    }
    fn add_node_in(&mut self, id: usize, node: Node) -> CFGStep {
        if self.cfg.graph.node_by_id(&id).is_some() {
            Err(CFGError::cause("the given id is already prsented"))
        } else {
            self.cfg.graph.add_node(id, node);
            Ok(())
        }
    }
    fn add_edge(&mut self, to: usize) -> CFGStep {
        self.cfg.graph.add_bare_edge(self.generator, to);
        Ok(())
    }
    fn add_edge_from(&mut self, from: usize, to: usize) -> CFGStep {
        self.cfg.graph.add_bare_edge(from, to);
        Ok(())
    }

    fn push_ctx(&mut self, ctx: usize) -> CFGStep {
        &self.ctx_stack.push(ctx);
        Ok(())
    }
    fn pop_ctx(&mut self) -> CFGStep<Option<usize>> {
        Ok(self.ctx_stack.pop())
    }
    fn peek_ctx(&self) -> CFGStep<usize> {
        self.ctx_stack
            .last()
            .map(Clone::clone)
            .ok_or(CFGError::cause("no contexts"))
    }
    fn peek_parent_ctx(&self) -> CFGStep<usize> {
        self.ctx_stack
            .get(self.ctx_stack.len() - 2)
            .map(Clone::clone)
            .ok_or(CFGError::cause("no contexts"))
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.cfg.graph.node_by_id_mut(&id)
    }
    pub fn get(&mut self, id: usize) -> Option<&mut Node> {
        self.cfg.graph.node_by_id_mut(&id)
    }
}


#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;
    use crate::parser::{Ast, parse, parse_root};
    use crate::parser::tests::load;


    #[test]
    fn smoke() {
        let ast = load("examples/smoke/frontend.sv");
        println!("{:?}", ast.tree);
    }
}
