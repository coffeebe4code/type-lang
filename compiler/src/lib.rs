use cranelift_codegen::ir::Function;
use object::ObjectSource;
use std::rc::Rc;
use types::TypeTree;

// right now is just looping on lint source to generate multiple ir and data
pub struct Compiler {
    pub lintres: Vec<Rc<Box<TypeTree>>>,
    pub firs: Vec<Function>,
    pub obj: ObjectSource,
}

impl<'table> Compiler {
    pub fn new(lintres: Vec<Rc<Box<TypeTree>>>, obj: ObjectSource) -> Self {
        Compiler {
            lintres,
            firs: vec![],
            obj,
        }
    }

    pub fn loopf(self) -> () {
        self.lintres.into_iter().for_each(|x| match x {
            TypeTree::ConstInit(y) => { self.obj.add_const_data(y.right.into_data()) },
            TypeTree::FuncInit(y) => { self.firs.push(

        });
    }
}
