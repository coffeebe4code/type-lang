use std::{collections::BTreeMap, rc::Rc};
use types::*;

pub struct SymTable {
    pub table: BTreeMap<String, (Rc<Box<TypeTree>>, u32)>,
}

impl SymTable {
    pub fn new() -> Self {
        SymTable {
            table: BTreeMap::new(),
        }
    }
}
