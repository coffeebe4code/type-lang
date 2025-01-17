use std::collections::HashMap;

use layout::*;
// Info table contains the layout info for each item at the scope.
#[derive(Debug)]
pub struct InfoTable {
    pub layout: HashMap<u32, Vec<(String, Layout)>>,
}

impl InfoTable {
    pub fn new() -> Self {
        InfoTable {
            layout: HashMap::new(),
        }
    }
}
