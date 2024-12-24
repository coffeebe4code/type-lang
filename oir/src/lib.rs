use cranelift_codegen::ir::Function;
use cranelift_codegen::settings::*;
use cranelift_codegen::Context;
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use datatable::DataTable;
use types::TopInitialization;
use types::TypeTree;

// Object intermediate representation
pub struct Oir {
    pub obj_mod: ObjectModule,
    pub data: DataDescription,
}

impl Oir {
    pub fn new(obj_name: &str) -> Oir {
        let mut settings = builder();
        let _ = settings.set("is_pic", "true");
        let flags = Flags::new(settings);
        let isa_builder = cranelift_native::builder().unwrap();
        let isa = isa_builder.finish(flags).unwrap();

        let obj_builder =
            ObjectBuilder::new(isa, obj_name, cranelift_module::default_libcall_names()).unwrap();
        Oir {
            obj_mod: ObjectModule::new(obj_builder),
            data: DataDescription::new(),
        }
    }
    pub fn recurse(&mut self, expr: &TypeTree) -> () {
        match expr {
            TypeTree::I64(x) => self.data.define(Box::from(x.clone().to_ne_bytes())),
            TypeTree::U64(x) => self.data.define(Box::from(x.clone().to_ne_bytes())),
            _ => panic!("unexpected type tree in oir"),
        }
    }

    pub fn const_init(&mut self, init: &TopInitialization, dt: &mut DataTable) -> () {
        let slice = &init.left.into_symbol_init().ident;
        self.recurse(init.right.as_ref());
        let id = self
            .obj_mod
            .declare_data(slice, Linkage::Export, false, false)
            .unwrap();
        self.obj_mod.define_data(id, &self.data).unwrap();
        dt.table.insert(slice.to_string(), id);
    }
    pub fn add_fn(&mut self, name: &str, func: Function) -> () {
        let func_id = self
            .obj_mod
            .declare_function(name, Linkage::Export, &func.signature)
            .unwrap();

        let mut ctx = Context::for_function(func);
        self.obj_mod.define_function(func_id, &mut ctx).unwrap();
    }
    pub fn flush_self(self) -> Vec<u8> {
        let object_product = self.obj_mod.finish();
        let bytes = object_product.emit().unwrap();
        return bytes;
    }
}
