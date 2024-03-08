use std::collections::BTreeMap;
use types::*;

pub struct SymTable<'i> {
    pub parent: Option<Box<SymTable<'i>>>,
    pub table: BTreeMap<String, &'i TypeTree>,
}

impl<'i> SymTable<'i> {
    pub fn new(parent: Option<Box<SymTable<'i>>>) -> Self {
        SymTable {
            parent,
            table: BTreeMap::new(),
        }
    }
}
