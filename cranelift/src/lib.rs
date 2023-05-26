use core::mem::drop;
use cranelift::codegen::ir::*;
use cranelift::codegen::isa::*;
use cranelift::frontend::Variable;
use cranelift::frontend::*;
use cranelift::prelude::types::*;

macro_rules! dispose {
    ($namespace:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _dispose >](val: *mut *mut [< $namespace >]) -> () {
                unsafe {
                    drop(Box::from_raw(val));
                }
            }
        }
    };
}

macro_rules! type_invoke {
    ($namespace:ident, $func_name: ident, $invoke_name: ident, $($variant:tt)*) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name >](val: *mut *mut $namespace) -> () {
                unsafe {
                    *val = Box::into_raw(Box::new([< $namespace >]::[< $invoke_name >]($($variant)*)));
                }
            }
        }
    }
}

/*
 *
 *
 *
*/

macro_rules! two_invoke_return {
    ($namespace:ident, $func_name:ident, $one:ident, $two:ident, $ret: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name >](one: $one, two: $two) -> *mut [< $ret >] {
                let mut val = [< $namespace >]::[<$func_name>](one, two);
                return &mut val;
            }
        }
    };
}

macro_rules! one_invoke_return {
    ($namespace:ident, $func_name:ident, $one:ident, $ret: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name >](one: $one) -> *mut [< $ret >] {
                let mut val = [< $namespace >]::[<$func_name>](one);
                return &mut val;
            }
        }
    };
}

macro_rules! self_one_invoke_return {
    ($namespace:ident, $func_name:ident, $one:ident, $ret: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name >](one: *mut $one) -> *mut [< $ret >] {
                let uone =  unsafe { &mut core::ptr::read(one) };
                let mut val = [< $namespace >]::[<$func_name>](uone);
                return &mut val;
            }
        }
    };
}

macro_rules! empty_invoke_return {
    ($namespace:ident, $func_name:ident, $ret: ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name >]() -> *mut [< $ret >] {
                let mut val = [< $namespace >]::[<$func_name>]();
                return &mut val;
            }
        }
    };
}

macro_rules! self_one_deref_invoke_void {
    ($namespace:ident, $func_name:ident, $one:ident) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<CL_ $namespace _ $func_name>](val: *mut $namespace, val2: *mut $one) -> () {
                let uval2 = unsafe { *val2 };
                let uval = unsafe { &mut *val };
                [< $namespace >]::[< $func_name >](uval, uval2);
            }
        }
    };
}

//
//
//
// END MACRO DECLS
//
//
//

//USERFUNCNAME
dispose!(UserFuncName);
two_invoke_return!(UserFuncName, user, u32, u32, UserFuncName);

// VARIABLE
dispose!(Variable);
one_invoke_return!(Variable, from_u32, u32, Variable);

// ABIPARAM
dispose!(AbiParam);
type_invoke!(AbiParam, new_i32, new, I32);

// SIGNATURE
dispose!(Signature);
type_invoke!(Signature, new_systemv, new, CallConv::SystemV);

// FUNCTION
dispose!(Function);
empty_invoke_return!(Function, new, Function);

#[no_mangle]
pub extern "C" fn CL_Function_with_name_signature(
    name: *const UserFuncName,
    sig: *const Signature,
) -> *mut Function {
    let uname = unsafe { core::ptr::read(name) };
    let usig = unsafe { core::ptr::read(sig) };
    return &mut Function::with_name_signature(uname, usig);
}

// FUNCTION BUILDER
dispose!(FunctionBuilder);
self_one_invoke_return!(FunctionBuilder, create_block, FunctionBuilder, Block);

self_one_deref_invoke_void!(
    FunctionBuilder,
    append_block_params_for_function_params,
    Block
);

#[no_mangle]
pub extern "C" fn CL_FunctionBuilder_declare_var(
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
pub extern "C" fn CL_FunctionBuilder_new<'a>(
    function: *mut Function,
    value: *mut FunctionBuilderContext,
) -> *mut FunctionBuilder<'a> {
    let ufunction = unsafe { &mut *function };
    let uvalue = unsafe { &mut *value };
    return &mut FunctionBuilder::new(ufunction, uvalue);
}

// FUNCTION BUILDER CONTEXT
dispose!(FunctionBuilderContext);
empty_invoke_return!(FunctionBuilderContext, new, FunctionBuilderContext);
