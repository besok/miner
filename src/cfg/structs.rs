use std::path::PathBuf;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
pub struct NId(pub usize);

impl NId {
    pub fn incr(&self) -> Self {
        NId(self.0 + 1)
    }
}

impl From<usize> for NId {
    fn from(value: usize) -> Self {
        NId(value)
    }
}

impl ToString for NId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    File(File),
    Module(Module),
    End(End),
}

impl Node {
    pub fn id(&self) -> NId {
        match self {
            Node::File(delegate) => delegate.id,
            Node::Module(delegate) => delegate.id,
            Node::End(delegate) => delegate.id,
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::File(delegate) => format!("file({})", delegate.id.to_string()),
            Node::Module(delegate) => format!("module({})", delegate.id.to_string()),
            Node::End(delegate) => format!("end of {}", delegate.target.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: NId,
    pub ctx:NId,
    pub name:String,

}

impl From<Module> for Node {
    fn from(value: Module) -> Self {
        Node::Module(value)
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: NId,
    pub name: String,
}

impl File {
    pub fn new(id: NId, name: PathBuf) -> Self {
        let name = name.to_str().map(|s|s.to_string()).unwrap_or(String::new());
        File { id, name }
    }
}

impl From<File> for Node {
    fn from(value: File) -> Self {
        Node::File(value)
    }
}

#[derive(Debug, Clone)]
pub struct End {
    pub id: NId,
    pub target: NId,
}

impl From<End> for Node {
    fn from(value: End) -> Self {
        Node::End(value)
    }
}