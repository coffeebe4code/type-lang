use std::collections::BTreeMap;

use layout::*;
// Info table contains the layout info for each item at the scope.
#[derive(Debug)]
pub struct InfoTable {
    pub layout: BTreeMap<String, Layout>,
}
