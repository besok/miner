use sv_parser::{RefNode, WhiteSpace};
use crate::cfg::processor::Processor;
use crate::cfg::CFGStep;

pub trait Operation {
    fn execute(self, builder: &mut Processor) -> CFGStep;
}

fn skip() -> CFGStep {
    Ok(())
}


impl Operation for WhiteSpace {
    fn execute(self, _builder: &mut Processor) -> CFGStep {
        skip()
    }
}

impl Operation for RefNode<'_> {
    fn execute(self, processor: &mut Processor) -> CFGStep {
        match self {
            RefNode::SourceText(ref txt) => {
                skip()
            }
            RefNode::WhiteSpace(_) => skip(),
            RefNode::ModuleKeyword(m) => {
                skip()
            }
            _ => {
                skip()
            }
        }
    }
}
