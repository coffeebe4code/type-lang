use cranelift::frontend::FunctionBuilderContext;

#[no_mangle]
pub extern "C" fn cl_function_builder_context_new() -> *mut FunctionBuilderContext {
    return &mut FunctionBuilderContext::new();
}

#[no_mangle]
pub extern "C" fn cl_function_builder_context_dispose(val: *mut FunctionBuilderContext) -> () {
    core::mem::drop(val);
}
