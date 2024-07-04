use std::rc::Rc;
use types::*;
use typetable::TypeTable;

pub struct ScopeTable {
    pub parent_scope: usize,
    pub self_scope: usize,
}

impl ScopeTable {
    pub fn new(parent_scope_id: usize, self_scope: usize) -> Self {
        ScopeTable {
            parent_scope: parent_scope_id,
            self_scope,
        }
    }
    pub fn get_tt_same_up<'sco, 'ttb: 'sco>(
        &'sco self,
        symbol: String,
        ttbls: &'ttb Vec<TypeTable>,
    ) -> Option<&Rc<Box<TypeTree>>> {
        let tbl = ttbls.get(self.self_scope).unwrap();
        let sibling = tbl.table.get(&symbol);
        if sibling.is_some() {
            return sibling;
        }
        if self.parent_scope != self.self_scope {
            let ptbl = ttbls.get(self.parent_scope).unwrap();
            let parent = ptbl.table.get(&symbol);
            if parent.is_some() {
                return parent;
            }
        }
        None
    }
}
