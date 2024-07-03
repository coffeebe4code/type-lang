use std::collections::BTreeMap;

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
