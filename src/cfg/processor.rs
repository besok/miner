mod module;
mod common;

use std::collections::{HashMap, HashSet};
use digraph_rs::DiGraph;
use sv_parser::WhiteSpace;
use crate::cfg::{CFGError, CFGraph, CFGStep, Node};
use crate::cfg::processor::common::Operation;
use crate::cfg::structs::{End, File, NId};
use crate::parser::Ast;

struct CtxStack {
    inner: Vec<NId>,
}

impl CtxStack {
    fn close(&mut self) -> CFGStep<NId> {
        Ok(
            self.inner
                .pop()
                .ok_or(CFGError::cause("no context to close"))?
        )
    }

    fn empty(&self) -> CFGStep<bool> {
        Ok(self.inner.is_empty())
    }
    fn push(&mut self, ctx_id: NId) -> CFGStep {
        &self.inner.push(ctx_id);
        Ok(())
    }
    fn pop(&mut self) -> CFGStep<Option<NId>> {
        Ok(self.inner.pop())
    }
    fn peek(&self) -> CFGStep<NId> {
        self.inner
            .last()
            .map(Clone::clone)
            .ok_or(CFGError::cause("no contexts"))
    }
}

struct Processor {
    gen: NId,
    ctx: CtxStack,
    cfg: CFGraph,
}

impl Processor {
    fn process(mut self, ast: Ast) -> CFGStep<CFGraph> {
        let tree = ast.tree;
        let file = File::new(self.next(), ast.path);

        self.add_ctx_node(file.into())?;

        for node in tree.into_iter() {
            node.execute(&mut self)?;
        }

        let id = self.close_ctx()?;
        self.add_edge(id)?;

        Ok(self.cfg)
    }
    fn create() -> Processor {
        Processor {
            gen: NId::default(),
            ctx: CtxStack { inner: vec![] },
            cfg: CFGraph { graph: DiGraph::new() },
        }
    }
}

impl Processor {
    fn next(&mut self) -> NId {
        self.gen = self.gen.incr();
        self.gen
    }
    fn prev(&self) -> NId {
        NId(self.gen.0 - 1)
    }
    fn last(&self) -> NId {
        self.gen
    }
    fn close_ctx(&mut self) -> CFGStep<NId> {
        let target = self.ctx.close()?;
        let id = self.next();
        self.add_node(End { id, target }.into())
    }
    fn add_ctx_node(&mut self, node: Node) -> CFGStep<NId> {
        self.ctx.push(node.id()).and_then(|_| self.add_node(node))
    }

    fn add_node(&mut self, node: Node) -> CFGStep<NId> {
        let id = node.id();
        self.cfg.graph.add_node(id.clone(), node);
        Ok(id.clone())
    }
    fn add_node_with(&mut self, id: NId, node: Node) -> CFGStep<NId> {
        if self.cfg.graph.node_by_id(&id).is_some() {
            Err(CFGError::cause("the given id is already presented"))
        } else {
            self.cfg.graph.add_node(id.clone(), node);
            Ok(id)
        }
    }

    fn add_edge(&mut self, to: NId) -> CFGStep {
        self.cfg.graph.add_bare_edge(self.prev(), to);
        Ok(())
    }
    fn add_edge_from(&mut self, from: NId, to: NId) -> CFGStep {
        self.cfg.graph.add_bare_edge(from, to);
        Ok(())
    }

    pub fn get_mut(&mut self, id: NId) -> Option<&mut Node> {
        self.cfg.graph.node_by_id_mut(&id)
    }
    pub fn get(&mut self, id: NId) -> Option<&mut Node> {
        self.cfg.graph.node_by_id_mut(&id)
    }
}


#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;
    use digraph_rs::visualizer::dot::ToStringProcessor;
    use crate::cfg::processor::Processor;
    use crate::parser::{Ast, parse, parse_root};
    use crate::parser::tests::load;


    fn graphs(name:&str) -> PathBuf {
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("graphs");
        f.push(name);
        f
    }

    #[test]
    fn smoke() {
        let ast = load("examples/smoke/frontend.sv");
        println!("{:?}", ast.tree);
    }

    #[test]
    fn builder_smoke() {
        let ast = load("examples/smoke/frontend.sv");
        let processor = Processor::create();
        let cfg = processor.process(ast).unwrap();


        cfg.visualize_to_file(graphs("example1.svg")).unwrap();
    }
}
