use cranelift::codegen::ir::*;
use cranelift::codegen::isa::*;
use cranelift::frontend::*;
use cranelift::frontend::Variable;
use cranelift::prelude::types::*;

macro_rules! empty_namespace_invoke_return {
    ($attr:ident, $func_name:ident, $invoke_name: ident, $ret: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _ $func_name >]() -> *mut [< $ret >] {
                let mut val = [< $attr >]::[<$invoke_name>]();
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

macro_rules! one_invoke_void {
    ($attr:ident, $name:ident, $one:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $attr _ $name>](val: *mut $attr, val2: *mut $one) -> () {
                let mut uval = unsafe { core::ptr::read(val) };
                let uval2 = unsafe { core::ptr::read(val2) };
                uval.[< $name >](uval2);
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
    } }
// Variable
empty_dispose!(Variable);

#[no_mangle]
pub extern "C" fn CL_Variable_From_U32(val: u32) -> *mut Variable {
    let mut val = Variable::from_u32(val);
    return &mut val;
}
// ABIPARAM
type_new!(AbiParam, I32, I32);
empty_dispose!(AbiParam);

// SIGNATURE
type_new!(Signature, SystemV, CallConv::SystemV);
empty_dispose!(Signature);

// FUNCTION
empty_namespace_invoke_return!(Function, New, new, Function);
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
one_invoke_void!(FunctionBuilder, append_block_params_for_function_params, Block);

#[no_mangle]
pub extern "C" fn CL_FunctionBuilder_Declare_Var(
    function: *mut FunctionBuilder,
    var: *mut Variable,
    ty: *mut Type,
) -> () {
    let ufunction = unsafe { &mut *function };
    let uvar = unsafe { *var };
    let uty = unsafe { *ty };
    ufunction.declare_var(uvar, uty);
}

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
empty_namespace_invoke_return!(FunctionBuilderContext, New, new, FunctionBuilderContext);
empty_dispose!(FunctionBuilderContext);
