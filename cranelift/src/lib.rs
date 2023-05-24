use cranelift::codegen::ir::*;
use cranelift::codegen::isa::*;
use cranelift::frontend::*;
use cranelift::prelude::types::*;

macro_rules! empty_new {
    ($attr:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _New >]() -> *mut [< $attr >] {
                let mut val = [< $attr >]::new();
                return &mut val;
            }
        }
    }
}

macro_rules! empty_invoke_return {
    ($attr:ident, $name:ident, $ret:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _ $name>](val: *mut $attr) -> *mut [< $ret >] {
                let mut uval = unsafe { core::ptr::read(val) };
                return &mut uval.[< $name >]();
            }
        }
    }
}

macro_rules! empty_dispose {
    ($attr:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _Dispose >](val: *mut [< $attr >]) -> () {
                core::mem::drop(val);
            }
        }
    }
}

macro_rules! type_new {
    ($attr:ident, $name: ident, $($variant:tt)*) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _New_ $name >]() -> *mut [< $attr >] {
                let mut val = [< $attr >]::new($($variant)*);
                return &mut val;
            }
        }
    }
}

// ABIPARAM
type_new!(AbiParam, I32, I32);

// SIGNATURE
type_new!(Signature, SystemV, CallConv::SystemV);
empty_dispose!(Signature);

// FUNCTION
empty_new!(Function);
empty_dispose!(Function);
#[no_mangle]
pub extern "C" fn CL_Function_With_Name_Signature(
    name: *const UserFuncName,
    sig: *const Signature,
) -> *mut Function {
    let uname = unsafe { core::ptr::read(name) };
    let usig= unsafe { core::ptr::read(sig) };
    return &mut Function::with_name_signature(uname, usig);

}

// FUNCTION BUILDER
empty_dispose!(FunctionBuilder);
empty_invoke_return!(FunctionBuilder, create_block, Block);

#[no_mangle]
pub extern "C" fn CL_FunctionBuilder_New<'a>(
    function: *mut Function,
    value: *mut FunctionBuilderContext,
) -> *mut FunctionBuilder<'a> {
    let ufunction = unsafe { &mut *function };
    let uvalue = unsafe { &mut *value };
    return &mut FunctionBuilder::new(ufunction, uvalue);
}

// FUNCTION BUILDER CONTEXT
empty_new!(FunctionBuilderContext);
empty_dispose!(FunctionBuilderContext);
