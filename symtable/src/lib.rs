use std::collections::BTreeMap;
use types::*;

pub struct SymTable<'tt> {
    pub parent: Option<Box<SymTable<'tt>>>,
    pub table: BTreeMap<String, &'tt TypeTree<'tt>>,
}

impl<'tt> SymTable<'tt> {
    pub fn new(parent: Option<Box<SymTable<'tt>>>) -> Self {
        SymTable {
            parent,
            table: BTreeMap::new(),
        }
    }
}
