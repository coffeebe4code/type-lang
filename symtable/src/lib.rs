use std::collections::BTreeMap;
use std::rc::Rc;
use types::*;

//todo:: this is for when we are using the TypeTree
pub struct TypeTable {
    pub table: BTreeMap<String, (Rc<Box<TypeTree>>, u32)>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable {
            table: BTreeMap::new(),
        }
    }
}

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
