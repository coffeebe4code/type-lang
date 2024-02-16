use std::collections::BTreeMap;
//use types::*;

//todo:: this is for when we are using the TypeTree
//pub struct SymTable {
//    pub parent: Box<SymTable>,
//    pub table: BTreeMap<String, Box<TypeTree>>,
//}

pub struct SymTable {
    pub table: BTreeMap<String, u32>,
}

impl SymTable {
    pub fn new() -> Self {
        SymTable {
            table: BTreeMap::new(),
        }
    }
}
