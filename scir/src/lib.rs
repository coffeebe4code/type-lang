use fir::Fir;
use oir::Oir;
use scopetable::ScopeTable;
use symtable::SymTable;
use typetable::TypeTable;

pub struct Scir {
    pub oir: Oir,
    pub fir: Fir,
    pub scopes: Vec<ScopeTable>,
    pub type_tables: Vec<TypeTable>,
}

impl Scir {
    pub fn new(name: &str, scopes: Vec<ScopeTable>, type_tables: Vec<TypeTable>) -> Scir {
        Scir {
            oir: Oir::new(name),
            fir: Fir::new(0, SymTable::new()),
            scopes,
            type_tables,
        }
    }
}
