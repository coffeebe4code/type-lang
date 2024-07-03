use std::collections::BTreeMap;
use std::rc::Rc;
use types::*;

pub struct TypeTable {
    pub table: BTreeMap<String, Rc<Box<TypeTree>>>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable {
            table: BTreeMap::new(),
        }
    }
}
