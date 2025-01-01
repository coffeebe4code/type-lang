use cranelift_codegen::ir::Function;
use cranelift_frontend::FunctionBuilderContext;
use datatable::DataTable;
use fir::Fir;
use oir::Oir;
use scopetable::ScopeTable;
use symtable::SymTable;
use types::{FunctionInitialize, TypeTree};

pub struct Scir {
    pub oir: Oir,
    pub fir: Fir,
    pub dtable: DataTable,
    pub scopes: Vec<ScopeTable>,
    pub types: Vec<TypeTree>,
    pub namespace: u32,
    pub index: u32,
    pub fbc: FunctionBuilderContext,
}

// Source Compiled Intermediate Representation
// Context of a Source file, these are the scopes, name, and type tables containing the type tree
// output from the linter.
// name = the name of the source file.
// scopes = the scopes output from linter.
// types = the type trees
impl Scir {
    pub fn new(name: &str, scopes: Vec<ScopeTableNew>, types: Vec<TypeTree>) -> Scir {
        Scir {
            oir: Oir::new(name),
            fir: Fir::new(0, SymTable::new()),
            dtable: DataTable::new(),
            scopes,
            types,
            namespace: 0,
            index: 0,
            fbc: FunctionBuilderContext::new(),
        }
    }
    // top_res is the output top decls of the linter
    pub fn loopf(&mut self, top_res: Vec<TypeTree>) -> () {
        for item in &top_res {
            match item {
                TypeTree::TopConstInit(ci) => {
                    self.oir.const_init(&ci, &mut self.dtable);
                }
                TypeTree::FuncInit(fi) => {
                    let _fn = self.make_fir(fi);
                    self.oir.add_fn(&fi.name, _fn);
                }
                _ => panic!("developer error, unhandled loopfval, {:?}", item.clone()),
            }
        }
    }
    fn make_fir(&mut self, fi: &FunctionInitialize) -> Function {
        self.fir.refresh();
        let _fn = self.fir.run(
            fi,
            &mut self.fbc,
            self.namespace,
            self.index,
            &self.dtable,
            &self.scopes,
            &self.types,
            &mut self.oir,
        );
        self.index += 1;
        return _fn;
    }
    pub fn flush_self(self) -> Vec<u8> {
        return self.oir.flush_self();
    }
}
