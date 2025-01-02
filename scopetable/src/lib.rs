use std::collections::BTreeMap;
use types::TypeTreeIndex;

#[derive(Debug)]
pub struct ScopeTable {
    pub parent_scope: u32,
    pub this_scope: u32,
    pub this_tree: BTreeMap<String, TypeTreeIndex>,
}

impl ScopeTable {
    pub fn new(parent_scope: u32, this_scope: u32) -> Self {
        ScopeTable {
            parent_scope,
            this_tree: BTreeMap::new(),
            this_scope,
        }
    }
    pub fn get_tt_idx_same_up(&self, symbol: &str, scopes: &Vec<ScopeTable>) -> Option<u32> {
        let sibling = self.this_tree.get(symbol);
        if let Some(sib) = sibling {
            return Some(*sib);
        }
        if self.parent_scope != 0 && self.this_scope != 0 {
            let ptbl = scopes.get(self.parent_scope as usize).unwrap();
            let parent = ptbl.this_tree.get(symbol);
            if let Some(par) = parent {
                return Some(*par);
            }
        }
        None
    }
}
