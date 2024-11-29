use cranelift_codegen::ir::Function;
use cranelift_codegen::settings::*;
use cranelift_codegen::Context;
use cranelift_module::DataId;
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub struct ObjectSource {
    pub obj_mod: ObjectModule,
    pub data: DataDescription,
    pub name: String,
}

impl ObjectSource {
    pub fn new(obj_name: &str) -> ObjectSource {
        let settings = builder();
        let flags = Flags::new(settings);
        let isa_builder = cranelift_native::builder().unwrap();
        let isa = isa_builder.finish(flags).unwrap();

        let obj_builder =
            ObjectBuilder::new(isa, obj_name, cranelift_module::default_libcall_names()).unwrap();
        ObjectSource {
            obj_mod: ObjectModule::new(obj_builder),
            data: DataDescription::new(),
            name: obj_name.to_string(),
        }
    }
    pub fn add_const_data(&mut self, name: &str, contents: Vec<u8>) -> DataId {
        self.data.define(contents.into_boxed_slice());
        let id = self
            .obj_mod
            .declare_data(name, Linkage::Export, false, false)
            .unwrap();
        self.obj_mod.define_data(id, &self.data).unwrap();
        return id;
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
