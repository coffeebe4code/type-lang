use cranelift_codegen::ir::Function;
use cranelift_module::DataId;
use ir::IRFunc;
use object::ObjectSource;
use std::rc::Rc;
use symtable::SymTable;
use types::{FunctionInitialize, TypeTree};
use typetable::TypeTable;

// right now is just looping on lint source to generate multiple ir and data
pub struct Compiler {
    pub lintres: Vec<Rc<Box<TypeTree>>>,
    pub fncnt: usize,
    pub firs: Vec<Function>,
    pub obj: ObjectSource,
    pub typtbl: Vec<TypeTable>,
    pub gbl: Vec<(DataId, String)>,
}

impl<'table> Compiler {
    pub fn new(lintres: Vec<Rc<Box<TypeTree>>>, obj: ObjectSource, typtbl: Vec<TypeTable>) -> Self {
        Compiler {
            lintres,
            firs: vec![],
            fncnt: 0,
            obj,
            typtbl,
            gbl: vec![],
        }
    }

    pub fn loopf(&mut self) -> () {
        for item in &self.lintres {
            match item.as_ref().as_ref() {
                TypeTree::ConstInit(y) => {
                    let id = self.obj.add_const_data("x", y.right.into_data());
                    self.gbl.push((id, "x".to_string()));
                }
                TypeTree::FuncInit(y) => {
                    let func = self.make_ir(&y);
                    self.firs.push(func);
                }
                _ => panic!("developer error, unhandled loopfval, {:?}", item.clone()),
            }
        }
    }
    fn make_ir(&self, fi: &FunctionInitialize) -> Function {
        let mut ir = IRFunc::new(0, SymTable::new());
        return ir.begin(fi, &self.gbl);
    }
}
