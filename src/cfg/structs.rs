trait Id {
    fn id(&self) -> usize;
}

trait Ctx: Id {}
#[derive(Debug)]
pub struct ModuleCtx {
    pub id: usize,
}

impl Id for ModuleCtx {
    fn id(&self) -> usize {
        self.id
    }
}

impl Ctx for ModuleCtx {}

#[derive(Debug)]
pub enum Node {
    OpenModuleCtx(ModuleCtx),
    CloseCtx(usize)
}

