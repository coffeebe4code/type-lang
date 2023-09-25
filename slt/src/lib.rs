use std::collections::BTreeMap;
pub struct SLT {
    table: BTreeMap<String, u32>,
}

impl SLT {
    pub fn new() -> Self {
        SLT {
            table: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, value: String, variable: u32) -> () {
        self.table.insert(value, variable);
    }
    pub fn lookup(&self, value: &str) -> Option<u32> {
        match self.table.get(value) {
            Some(v) => Some(*v),
            _ => None,
        }
    }
}
