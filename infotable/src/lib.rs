use layout::*;
use std::collections::HashMap;

// Info table contains the layout info for each item at the scope.
#[derive(Debug)]
pub struct InfoTable {
    pub layout: HashMap<u32, Vec<(String, Container)>>,
}

impl InfoTable {
    pub fn new() -> Self {
        InfoTable {
            layout: HashMap::new(),
        }
    }
    pub fn insert(&mut self, scope: u32, key: String, val: Container) {
        let sc = self.layout.get_mut(&scope);
        if let Some(x) = sc {
            x.push((key, val));
        } else {
            self.layout.insert(scope, vec![(key, val)]);
        }
    }
}
