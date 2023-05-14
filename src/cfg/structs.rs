trait Id {
    type Id;
    fn id(&self) -> Self::Id;
}

pub enum Node {}

pub struct Ctx {
    pub id: usize,
}

impl Id for Ctx {
    type Id = usize;

    fn id(&self) -> Self::Id {
        self.id
    }
}
