use std::rc::Rc;
use types::*;
use typetable::TypeTable;

pub struct ScopeTable {
    pub parent_scope: u32,
    pub self_scope: u32,
}

impl ScopeTable {
    pub fn new(parent_scope_id: u32, self_scope: u32) -> Self {
        ScopeTable {
            parent_scope: parent_scope_id,
            self_scope,
        }
    }
    pub fn get_tt_same_up<'sco, 'ttb: 'sco>(
        &'sco self,
        symbol: String,
        ttbls: &'ttb Vec<TypeTable>,
        scopes: &'sco Vec<ScopeTable>,
    ) -> Option<&'sco Rc<Box<TypeTree>>> {
        let tbl = ttbls.get(self.self_scope as usize).unwrap();
        let sibling = tbl.table.get(&symbol);
        if sibling.is_some() {
            return sibling;
        }
        if self.parent_scope != self.self_scope {
            let ptbl = ttbls.get(self.parent_scope as usize).unwrap();
            let parent = ptbl.table.get(&symbol);
            if parent.is_some() {
                return parent;
            }
            if self.parent_scope != 0 && self.self_scope != 0 {
                return scopes
                    .get(self.parent_scope as usize)
                    .unwrap()
                    .get_tt_same_up(symbol, ttbls, scopes);
            }
        }
        None
    }
}
