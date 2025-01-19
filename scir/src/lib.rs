use cranelift_frontend::FunctionBuilderContext;
use datatable::DataTable;
use fir::Fir;
use infotable::InfoTable;
use layout::{Container, LayoutBuilder};
use oir::Oir;
use scopetable::ScopeTable;
use symtable::SymTable;
use types::TypeTree;

pub struct Scir {
    pub oir: Oir,
    pub fir: Fir,
    pub dtable: DataTable,
    pub scopes: Vec<ScopeTable>,
    pub types: Vec<TypeTree>,
    pub info: InfoTable,
    pub namespace: u32,
    pub index: u32,
    pub fbc: FunctionBuilderContext,
    pub layout: LayoutBuilder,
}

// Source Compiled Intermediate Representation
// Context of a Source file, these are the scopes, name, and type tables containing the type tree
// output from the linter.
// name = the name of the source file.
// scopes = the scopes output from linter.
// types = the type trees
impl Scir {
    pub fn new(name: &str, scopes: Vec<ScopeTable>, types: Vec<TypeTree>) -> Scir {
        Scir {
            oir: Oir::new(name),
            fir: Fir::new(0, SymTable::new()),
            layout: LayoutBuilder::new(),
            dtable: DataTable::new(),
            scopes,
            types,
            namespace: 0,
            index: 0,
            info: InfoTable::new(),
            fbc: FunctionBuilderContext::new(),
        }
    }
    pub fn resolve_layouts(&mut self, item: usize, scope: u32) {
        let tt = self.types.get(item as usize).unwrap();
        match tt {
            TypeTree::StructInfo(s) => {
                let curr = s.curried.into_struct();

                let layout = self.layout.struct_layout(&curr.1);
                self.info
                    .insert(scope, curr.0.clone(), Container::Struct(layout));
                for x in s.props {
                    self.resolve_layouts(x, s.child_scope);
                }
            }
            _ => panic!("developer error, unhandled layout resolution, {:?}", tt),
        }
    }
    // top_res is the output top decls of the linter
    pub fn loopf(&mut self, top_res: Vec<u32>) -> () {
        for item in top_res {
            let tt = self.types.get(item as usize).unwrap();
            match tt {
                TypeTree::TopMutInit(m) => {
                    self.oir
                        .mut_init(&m, &mut self.dtable, &self.types, &mut self.layout);
                }
                TypeTree::TopConstInit(ci) => {
                    self.oir
                        .const_init(&ci, &mut self.dtable, &self.types, &mut self.layout);
                }
                TypeTree::FuncInit(fi) => {
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
                    self.oir.add_fn(&fi.name, _fn);
                }
                TypeTree::StructInfo(s) => {
                    let curr = s.curried.into_struct();
                    let layout = self.layout.struct_layout(&curr.1);
                    self.info
                        .insert(0, s.name.clone(), Container::Struct(layout));
                    for x in s.props {
                        self.resolve_layouts(x, s.child_scope);
                    }
                }
                _ => panic!("developer error, unhandled loopfval, {:?}", tt),
            }
        }
    }
    pub fn flush_self(self) -> Vec<u8> {
        return self.oir.flush_self();
    }
}
