use cranelift_codegen::ir::{types, AbiParam, Function, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::*;
use cranelift_codegen::Context;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub fn build_main(func: Function) -> Vec<u8> {
    let settings = builder();
    let flags = Flags::new(settings);
    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(flags).unwrap();

    let obj_builder =
        ObjectBuilder::new(isa, "main", cranelift_module::default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(obj_builder);

    let mut signature = Signature::new(CallConv::SystemV);
    signature.returns.push(AbiParam::new(types::I16));

    let func_id = module
        .declare_function("main", Linkage::Export, &func.signature)
        .unwrap();

    let mut ctx = Context::for_function(func);
    module.define_function(func_id, &mut ctx).unwrap();

    let object_product = module.finish();
    let bytes = object_product.emit().unwrap();
    return bytes;
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
