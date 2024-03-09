use std::collections::BTreeMap;
use types::*;

pub struct SymTable<'tt> {
    pub table: BTreeMap<String, &'tt TypeTree<'tt>>,
}

impl<'tt> SymTable<'tt> {
    pub fn new() -> Self {
        SymTable {
            table: BTreeMap::new(),
        }
    }
}
