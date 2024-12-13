use std::collections::BTreeMap;
use std::rc::Rc;
use types::*;

// the purpose of this table is to serve as a lookup for the types of identifiers
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
