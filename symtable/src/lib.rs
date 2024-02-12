use std::collections::BTreeMap;

pub struct SymTable {
    pub scope: String,
    pub table: BTreeMap<String, u64>,
}

impl SymTable {
    pub fn new(scope: String) -> Self {
        SymTable {
            scope,
            table: BTreeMap::new(),
        }
    }
}
