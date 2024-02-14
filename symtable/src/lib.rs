use std::collections::BTreeMap;
use types::*;

pub struct SymTable {
    pub parent: Box<SymTable>,
    pub table: BTreeMap<String, Box<TypeTree>>,
}

impl SymTable {
    pub fn new(parent: Box<SymTable>) -> Self {
        SymTable {
            parent,
            table: BTreeMap::new(),
        }
    }
}
