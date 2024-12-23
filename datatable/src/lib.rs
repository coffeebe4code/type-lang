use cranelift_module::DataId;
use std::collections::BTreeMap;

pub struct DataTable {
    pub table: BTreeMap<String, DataId>,
}

impl DataTable {
    pub fn new() -> Self {
        DataTable {
            table: BTreeMap::new(),
        }
    }
}
