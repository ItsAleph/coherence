#[derive(Debug)]
pub struct CHRFile {
    pub header: CHRHeader,
    pub patches: Vec<CHRPatch>,
}

#[derive(Debug)]
pub struct CHRHeader {
    pub author: CHRAuthor,
    pub date: u64,
    pub description: String,
}

#[derive(Debug)]
pub struct CHRAuthor {
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct CHRPatch {
    pub type_: CHRPatchType,
    pub path: String,
    pub actions: Vec<CHRAction>,
}

#[derive(Debug)]
pub enum CHRPatchType {
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum CHRAction {
    Move {
        path: String,
    },
    Cut {
        number: u64,
    },
    Replace {
        line: u64,
        content: String,
    },
    Push {
        line: u64,
        offset: u64,
        content: String,
    },
}
