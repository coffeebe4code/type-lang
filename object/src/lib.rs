use cranelift_codegen::ir::{types, AbiParam, Function, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::*;
use cranelift_codegen::Context;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub fn new_obj_handler(obj_name: &str) -> ObjectModule {
    let settings = builder();
    let flags = Flags::new(settings);
    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(flags).unwrap();

    let obj_builder =
        ObjectBuilder::new(isa, obj_name, cranelift_module::default_libcall_names()).unwrap();
    ObjectModule::new(obj_builder)
}

pub fn build_std_fn(om: &mut ObjectModule, func: Function, obj_name: &str) -> () {
    let mut signature = Signature::new(CallConv::SystemV);
    signature.returns.push(AbiParam::new(types::I16));

    let func_id = om
        .declare_function(obj_name, Linkage::Export, &func.signature)
        .unwrap();

    let mut ctx = Context::for_function(func);
    om.define_function(func_id, &mut ctx).unwrap();
}

pub fn flush_obj(om: ObjectModule) -> Vec<u8> {
    let object_product = om.finish();
    let bytes = object_product.emit().unwrap();
    return bytes;
}
