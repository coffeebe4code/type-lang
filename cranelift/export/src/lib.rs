use cranelift::prelude::*;
use cranelift::codegen::ir::*;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::isa::*;
use cranelift_object::ObjectBuilder;
use cranelift_module::*;
use target_lexicon::*;
use libc::c_char;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;

pub struct ModuleData {
    builder_ctx: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: Option<Module<ObjectBackend>>,
    userdata: usize
}
pub struct FunctionData<'a> {
    variable_counter: u32,
    builder: FunctionBuilder<'a>,
    module: &'a mut Module<ObjectBackend>
}

#[no_mangle]
pub extern "C" fn cranelift_module_new(
    target_triple: *const c_char,
    flags: *const c_char,
    name: *const c_char,
    err: *const c_char,
    userdata: usize,
) -> *mut ModuleData {
    let mut flag_builder = settings::builder();
    let trip: &str = unsafe { CStr::from_ptr(target_triple) }.to_str().unwrap();
    let flag: &str = unsafe { CStr::from_ptr(flags) }.to_str().unwrap();
    let name_str: &str = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let triple = triple!(trip);
    let isa_builder = isa::lookup(triple).unwrap();

    for s in flag.split(",") {
        if s.len() > 0 {
            let n = s.find(",");
            if n.is_none() {
                let res = flag_builder.enable(s);
                if res.is_err() {
                    let dd: &dyn Display = &res.err().unwrap();
                    let error = dd.to_string();
                    err = CString::new(format! ("userdata {}: {}", userdata, error)).unwrap().as_ptr() as *const c_char;
                }
                res.unwrap();
            } else {
                let args = s.split_at(n.unwrap());
                let res = flag_builder.set(args.0, args.1);
                if res.is_err() {
                    let dd: &dyn Display = &res.err().unwrap();
                    let error = dd.to_string();
                    err = CString::new(format! ("userdata {}: {}", userdata, error)).unwrap().as_ptr() as *const c_char;
                }
                res.unwrap();
            }
        }
    }

    let isa = isa_builder.finish(settings::Flags::new(flag_builder));

    let builder = ObjectBuilder::new(
        isa.unwrap(),
        name_str.to_owned(),
        cranelift_module::default_libcall_names(),
    );

    let module = Module::new(builder);
    return Box::into_raw(Box::new(ModuleData {
        builder_ctx: FunctionBuilderContext::new(),
        ctx: module.make_context(),
        data_ctx: DataContext::new(),
        module: Some(module),
        userdata
    }));
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftLinkage {
    Import = 0,
    Local = 1,
    Preemptible = 2,
    Hidden = 3,
    Export = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftDataFlags {
    None = 0,
    TLS = 1,
    Writable = 2,
}

fn convert_linkage(l: CraneliftLinkage) -> Linkage {
    match l {
        CraneliftLinkage::Export => Linkage::Export,
        CraneliftLinkage::Import => Linkage::Import,
        CraneliftLinkage::Local => Linkage::Local,
        CraneliftLinkage::Preemptible => Linkage::Preemptible,
        CraneliftLinkage::Hidden => Linkage::Hidden,
    }
}

#[no_mangle]
pub extern "C" fn cranelift_define_data(
    ptr: *mut ModuleData,
    name: *const c_char,
    linkage: CraneliftLinkage,
    data_flags: CraneliftDataFlags,
    err: *const c_char,
    align: u8,
    id: *mut u32,
) -> bool {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let real_name: &str = unsafe { CStr::from_ptr(name) }.to_str().unwrap();

    let intid = inst.module.as_mut().unwrap().declare_data(
        real_name,
        convert_linkage(linkage),
        CraneliftDataFlags::Writable as i32 & data_flags as i32 != 0,
        CraneliftDataFlags::TLS as i32 & data_flags as i32 != 0,
        if align == 0 {
            Option::None
        } else {
            Option::from(align)
        },
    );

    if intid.is_err() {
        let dd: &dyn Display = &intid.err().unwrap();
        let error = dd.to_string();
        err = CString::new(format! ("cranelift_define_data: {}", error)).unwrap().as_ptr() as *const c_char; 
    }
    unsafe {
        *id = intid.unwrap().as_u32();
    }
    true
}

#[no_mangle]
pub extern "C" fn cranelift_declare_function(
    ptr: *mut ModuleData,
    name: *const c_char,
    err: *const c_char,
    linkage: CraneliftLinkage,
    id: *mut u32,
) -> bool {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let real_name: &str = unsafe { CStr::from_ptr(name) }.to_str().unwrap();

    let intid = inst.module.as_mut().unwrap().declare_function(
        real_name,
        convert_linkage(linkage),
        &mut inst.ctx.func.signature,
    );

    if intid.is_err() {
        let dd: &dyn Display = &intid.err().unwrap();
        let error = dd.to_string();
        err = CString::new(format! ("cranelift_declare_function: {}", error)).unwrap().as_ptr() as *const c_char; 
    }
    unsafe {
        *id = intid.unwrap().as_u32();
    }
    true
}

#[no_mangle]
pub extern "C" fn cranelift_define_function(ptr: *mut ModuleData, func: u32, err: *const c_char) -> i32 {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let res = inst.module.as_mut().unwrap().define_function(
        FuncId::from_u32(func),
        &mut inst.ctx,
        &mut NullTrapSink {},
    );
    if res.is_err() {
        let dd: &dyn Display = &res.err().unwrap();
        let error = dd.to_string();
        err = CString::new(format! ("cranelift_define_function: {}", error)).unwrap().as_ptr() as *const c_char; 
    }
    return res.unwrap().size as i32;
}

#[no_mangle]
pub extern "C" fn cranelift_set_data_value(
    ptr: *mut ModuleData,
    content: *const u8,
    length: i32,
) -> bool {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    if content.is_null() {
        inst.data_ctx.define_zeroinit(length as usize);
    } else {
        let data = unsafe { core::slice::from_raw_parts(content, length as usize).to_vec() };
        inst.data_ctx.define(data.into_boxed_slice());
    }
    true
}

#[no_mangle]
pub extern "C" fn cranelift_set_data_section(
    ptr: *mut ModuleData,
    seg: *const c_char,
    sec: *const c_char,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let real_seg: &str = unsafe { CStr::from_ptr(seg) }.to_str().unwrap();
    let real_sec: &str = unsafe { CStr::from_ptr(sec) }.to_str().unwrap();

    inst.data_ctx.set_section(real_seg, real_sec);
}

#[no_mangle]
pub extern "C" fn cranelift_clear_data(ptr: *mut ModuleData) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    inst.data_ctx.clear();
}

#[no_mangle]
pub extern "C" fn cranelift_write_data_in_data(
    ptr: *mut ModuleData,
    target_id: u32,
    offset: u32,
    source_id: u32,
    addend: i64,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gv = inst
        .module
        .as_mut()
        .unwrap()
        .declare_data_in_data(DataId::from_u32(source_id), &mut inst.data_ctx);
    inst.module.as_mut().unwrap().write_data_dataaddr(
        DataId::from_u32(target_id),
        offset as usize,
        gv,
        addend,
    );
}

#[no_mangle]
pub extern "C" fn cranelift_write_function_in_data(
    ptr: *mut ModuleData,
    target_id: u32,
    offset: u32,
    source_id: u32,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gv = inst
        .module
        .as_mut()
        .unwrap()
        .declare_func_in_data(FuncId::from_u32(source_id), &mut inst.data_ctx);
    inst.module.as_mut().unwrap().write_data_funcaddr(
        DataId::from_u32(target_id),
        offset as usize,
        gv,
    )
}

#[no_mangle]
pub extern "C" fn cranelift_assign_data_to_global(ptr: *mut ModuleData, id: u32, err: *const c_char) -> bool {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let res = inst
        .module
        .as_mut()
        .unwrap()
        .define_data(DataId::from_u32(id), &inst.data_ctx);
    if res.is_err() {
        let dd: &dyn Display = &res.err().unwrap();
        let error = dd.to_string();
        err = CString::new(format! ("cranelift_declare_function: {}", error)).unwrap().as_ptr() as *const c_char; 
        res.unwrap()
    }
    inst.data_ctx.clear();
    true
}

#[no_mangle]
pub extern "C" fn cranelift_module_emit_object(
    ptr: *mut ModuleData,
    filename: *const c_char,
) -> bool {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let product = inst.module.take().unwrap().finish();
    let filenamestr: &str = unsafe { CStr::from_ptr(filename) }.to_str().unwrap();

    if cfg!(debug_assertions) {
        inst.emit_message_string(&format!("Emitting object file {}", filenamestr)[..], None)
    }

    let file = File::create(filenamestr);
    if file.is_err() {
        inst.emit_error(&file.err().unwrap(), Some(filenamestr));
        return false;
    }

    let data = product.emit();
    if data.is_err() {
        inst.emit_error(&data.err().unwrap(), Some(filenamestr));
        return false;
    }

    let write_err = file.unwrap().write(&data.unwrap());
    if write_err.is_err() {
        {
            inst.emit_error(&write_err.err().unwrap(), Some(filenamestr));
            return false;
        }
    }

    if cfg!(debug_assertions) {
        inst.emit_message_string(
            &format!("Done emitting object file {}", filenamestr)[..],
            None,
        )
    }

    true
}

#[no_mangle]
pub extern "C" fn cranelift_module_delete(ptr: *mut ModuleData) {
    if !ptr.is_null() {
        unsafe {
            Box::from_raw(ptr);
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftCallConv {
    CraneliftCallConvDefault = 0xffffffff,
    CraneliftCallConvFast = 0,
    CraneliftCallConvCold = 1,
    CraneliftCallConvSystemV = 2,
    CraneliftCallConvWindowsFastcall = 3,
    CraneliftCallConvProbestack = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftIntCC {
    CraneliftIntCCEqual = 0,
    CraneliftIntCCNotEqual = 1,
    CraneliftIntCCSignedLessThan = 2,
    CraneliftIntCCSignedGreaterThanOrEqual = 3,
    CraneliftIntCCSignedGreaterThan = 4,
    CraneliftIntCCSignedLessThanOrEqual = 5,
    CraneliftIntCCUnsignedLessThan = 6,
    CraneliftIntCCUnsignedGreaterThanOrEqual = 7,
    CraneliftIntCCUnsignedGreaterThan = 8,
    CraneliftIntCCUnsignedLessThanOrEqual = 9,
    CraneliftIntCCOverflow = 10,
    CraneliftIntCCNotOverflow = 11,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CraneliftFloatCC {
    CraneliftFloatCCOrdered = 0,
    CraneliftFloatCCUnordered = 1,
    CraneliftFloatCCEqual = 2,
    CraneliftFloatCCNotEqual = 3,
    CraneliftFloatCCOrderedNotEqual = 4,
    CraneliftFloatCCUnorderedOrEqual = 5,
    CraneliftFloatCCLessThan = 6,
    CraneliftFloatCCLessThanOrEqual = 7,
    CraneliftFloatCCGreaterThan = 8,
    CraneliftFloatCCGreaterThanOrEqual = 9,
    CraneliftFloatCCUnorderedOrLessThan = 10,
    CraneliftFloatCCUnorderedOrLessThanOrEqual = 11,
    CraneliftFloatCCUnorderedOrGreaterThan = 12,
    CraneliftFloatCCUnorderedOrGreaterThanOrEqual = 13,
}
/// CPU flags representing the result of an integer comparison. These flags
/// can be tested with an :u8:`intcc` condition code.
#[allow(non_upper_case_globals)]
pub const TypeIFLAGS: u8 = 0x1;

/// CPU flags representing the result of a floating point comparison. These
/// flags can be tested with a :u8:`floatcc` condition code.
#[allow(non_upper_case_globals)]
pub const TypeFFLAGS: u8 = 0x2;

/// A boolean u8 with 1 bits.
#[allow(non_upper_case_globals)]
pub const TypeB1: u8 = 0x70;

/// A boolean u8 with 8 bits.
#[allow(non_upper_case_globals)]
pub const TypeB8: u8 = 0x71;

/// A boolean u8 with 16 bits.
#[allow(non_upper_case_globals)]
pub const TypeB16: u8 = 0x72;

/// A boolean u8 with 32 bits.
#[allow(non_upper_case_globals)]
pub const TypeB32: u8 = 0x73;

/// A boolean u8 with 64 bits.
#[allow(non_upper_case_globals)]
pub const TypeB64: u8 = 0x74;

/// A boolean u8 with 128 bits.
#[allow(non_upper_case_globals)]
pub const TypeB128: u8 = 0x75;

/// An integer u8 with 8 bits.
/// WARNING: arithmetic on 8bit integers is incomplete
#[allow(non_upper_case_globals)]
pub const TypeI8: u8 = 0x76;

/// An integer u8 with 16 bits.
/// WARNING: arithmetic on 16bit integers is incomplete
#[allow(non_upper_case_globals)]
pub const TypeI16: u8 = 0x77;

/// An integer u8 with 32 bits.
#[allow(non_upper_case_globals)]
pub const TypeI32: u8 = 0x78;

/// An integer u8 with 64 bits.
#[allow(non_upper_case_globals)]
pub const TypeI64: u8 = 0x79;

/// An integer u8 with 128 bits.
#[allow(non_upper_case_globals)]
pub const TypeI128: u8 = 0x7a;

/// A 32-bit floating point u8 represented in the IEEE 754-2008
/// *binary32* interchange format. This corresponds to the :c:u8:`float`
/// u8 in most C implementations.
#[allow(non_upper_case_globals)]
pub const TypeF32: u8 = 0x7b;

/// A 64-bit floating point u8 represented in the IEEE 754-2008
/// *binary64* interchange format. This corresponds to the :c:u8:`double`
/// u8 in most C implementations.
#[allow(non_upper_case_globals)]
pub const TypeF64: u8 = 0x7c;

/// An opaque reference u8 with 32 bits.
#[allow(non_upper_case_globals)]
pub const TypeR32: u8 = 0x7e;

/// An opaque reference u8 with 64 bits.
#[allow(non_upper_case_globals)]
pub const TypeR64: u8 = 0x7f;

/// A SIMD vector with 8 lanes containing a `b8` each.
#[allow(non_upper_case_globals)]
pub const TypeB8X8: u8 = 0xa1;

/// A SIMD vector with 4 lanes containing a `b16` each.
#[allow(non_upper_case_globals)]
pub const TypeB16X4: u8 = 0x92;

/// A SIMD vector with 2 lanes containing a `b32` each.
#[allow(non_upper_case_globals)]
pub const TypeB32X2: u8 = 0x83;

/// A SIMD vector with 8 lanes containing a `i8` each.
#[allow(non_upper_case_globals)]
pub const TypeI8X8: u8 = 0xa6;

/// A SIMD vector with 4 lanes containing a `i16` each.
#[allow(non_upper_case_globals)]
pub const TypeI16X4: u8 = 0x97;

/// A SIMD vector with 2 lanes containing a `i32` each.
#[allow(non_upper_case_globals)]
pub const TypeI32X2: u8 = 0x88;

/// A SIMD vector with 2 lanes containing a `f32` each.
#[allow(non_upper_case_globals)]
pub const TypeF32X2: u8 = 0x8b;

/// A SIMD vector with 16 lanes containing a `b8` each.
#[allow(non_upper_case_globals)]
pub const TypeB8X16: u8 = 0xb1;

/// A SIMD vector with 8 lanes containing a `b16` each.
#[allow(non_upper_case_globals)]
pub const TypeB16X8: u8 = 0xa2;

/// A SIMD vector with 4 lanes containing a `b32` each.
#[allow(non_upper_case_globals)]
pub const TypeB32X4: u8 = 0x93;

/// A SIMD vector with 2 lanes containing a `b64` each.
#[allow(non_upper_case_globals)]
pub const TypeB64X2: u8 = 0x84;

/// A SIMD vector with 16 lanes containing a `i8` each.
#[allow(non_upper_case_globals)]
pub const TypeI8X16: u8 = 0xb6;

/// A SIMD vector with 8 lanes containing a `i16` each.
#[allow(non_upper_case_globals)]
pub const TypeI16X8: u8 = 0xa7;

/// A SIMD vector with 4 lanes containing a `i32` each.
#[allow(non_upper_case_globals)]
pub const TypeI32X4: u8 = 0x98;

/// A SIMD vector with 2 lanes containing a `i64` each.
#[allow(non_upper_case_globals)]
pub const TypeI64X2: u8 = 0x89;

/// A SIMD vector with 4 lanes containing a `f32` each.
#[allow(non_upper_case_globals)]
pub const TypeF32X4: u8 = 0x9b;

/// A SIMD vector with 2 lanes containing a `f64` each.
#[allow(non_upper_case_globals)]
pub const TypeF64X2: u8 = 0x8c;

/// A SIMD vector with 32 lanes containing a `b8` each.
#[allow(non_upper_case_globals)]
pub const TypeB8X32: u8 = 0xc1;

/// A SIMD vector with 16 lanes containing a `b16` each.
#[allow(non_upper_case_globals)]
pub const TypeB16X16: u8 = 0xb2;

/// A SIMD vector with 8 lanes containing a `b32` each.
#[allow(non_upper_case_globals)]
pub const TypeB32X8: u8 = 0xa3;

/// A SIMD vector with 4 lanes containing a `b64` each.
#[allow(non_upper_case_globals)]
pub const TypeB64X4: u8 = 0x94;

/// A SIMD vector with 2 lanes containing a `b128` each.
#[allow(non_upper_case_globals)]
pub const TypeB128X2: u8 = 0x85;

/// A SIMD vector with 32 lanes containing a `i8` each.
#[allow(non_upper_case_globals)]
pub const TypeI8X32: u8 = 0xc6;

/// A SIMD vector with 16 lanes containing a `i16` each.
#[allow(non_upper_case_globals)]
pub const TypeI16X16: u8 = 0xb7;

/// A SIMD vector with 8 lanes containing a `i32` each.
#[allow(non_upper_case_globals)]
pub const TypeI32X8: u8 = 0xa8;

/// A SIMD vector with 4 lanes containing a `i64` each.
#[allow(non_upper_case_globals)]
pub const TypeI64X4: u8 = 0x99;

/// A SIMD vector with 2 lanes containing a `i128` each.
#[allow(non_upper_case_globals)]
pub const TypeI128X2: u8 = 0x8a;

/// A SIMD vector with 8 lanes containing a `f32` each.
#[allow(non_upper_case_globals)]
pub const TypeF32X8: u8 = 0xab;

/// A SIMD vector with 4 lanes containing a `f64` each.
#[allow(non_upper_case_globals)]
pub const TypeF64X4: u8 = 0x9c;

/// A SIMD vector with 64 lanes containing a `b8` each.
#[allow(non_upper_case_globals)]
pub const TypeB8X64: u8 = 0xd1;

/// A SIMD vector with 32 lanes containing a `b16` each.
#[allow(non_upper_case_globals)]
pub const TypeB16X32: u8 = 0xc2;

/// A SIMD vector with 16 lanes containing a `b32` each.
#[allow(non_upper_case_globals)]
pub const TypeB32X16: u8 = 0xb3;

/// A SIMD vector with 8 lanes containing a `b64` each.
#[allow(non_upper_case_globals)]
pub const TypeB64X8: u8 = 0xa4;

/// A SIMD vector with 4 lanes containing a `b128` each.
#[allow(non_upper_case_globals)]
pub const TypeB128X4: u8 = 0x95;

/// A SIMD vector with 64 lanes containing a `i8` each.
#[allow(non_upper_case_globals)]
pub const TypeI8X64: u8 = 0xd6;

/// A SIMD vector with 32 lanes containing a `i16` each.
#[allow(non_upper_case_globals)]
pub const TypeI16X32: u8 = 0xc7;

/// A SIMD vector with 16 lanes containing a `i32` each.
#[allow(non_upper_case_globals)]
pub const TypeI32X16: u8 = 0xb8;

/// A SIMD vector with 8 lanes containing a `i64` each.
#[allow(non_upper_case_globals)]
pub const TypeI64X8: u8 = 0xa9;

/// A SIMD vector with 4 lanes containing a `i128` each.
#[allow(non_upper_case_globals)]
pub const TypeI128X4: u8 = 0x9a;

/// A SIMD vector with 16 lanes containing a `f32` each.
#[allow(non_upper_case_globals)]
pub const TypeF32X16: u8 = 0xbb;

/// A SIMD vector with 8 lanes containing a `f64` each.
#[allow(non_upper_case_globals)]
pub const TypeF64X8: u8 = 0xac;
type Type = u8;

type TrapCode = u32;
/// The current stack space was exhausted.
///
/// On some platforms, a stack overflow may also be indicated by a segmentation fault from the
/// stack guard page.
#[allow(non_upper_case_globals)]
pub const TrapCodeStackOverflow: u32 = 1 << 16;

/// A `heap_addr` instruction detected an out-of-bounds error.
///
/// Note that not all out-of-bounds heap accesses are reported this way;
/// some are detected by a segmentation fault on the heap unmapped or
/// offset-guard pages.
#[allow(non_upper_case_globals)]
pub const TrapCodeHeapOutOfBounds: u32 = 2 << 16;

/// A `table_addr` instruction detected an out-of-bounds error.
#[allow(non_upper_case_globals)]
pub const TrapCodeTableOutOfBounds: u32 = 3 << 16;

/// Indirect call to a null table entry.
#[allow(non_upper_case_globals)]
pub const TrapCodeIndirectCallToNull: u32 = 5 << 16;

/// Signature mismatch on indirect call.
#[allow(non_upper_case_globals)]
pub const TrapCodeBadSignature: u32 = 6 << 16;

/// An integer arithmetic operation caused an overflow.
#[allow(non_upper_case_globals)]
pub const TrapCodeIntegerOverflow: u32 = 7 << 16;

/// An integer division by zero.
#[allow(non_upper_case_globals)]
pub const TrapCodeIntegerDivisionByZero: u32 = 8 << 16;

/// Failed float-to-int conversion.
#[allow(non_upper_case_globals)]
pub const TrapCodeBadConversionToInteger: u32 = 9 << 16;

/// Code that was supposed to have been unreachable was reached.
#[allow(non_upper_case_globals)]
pub const TrapCodeUnreachableCodeReached: u32 = 10 << 16;

/// Execution has potentially run too long and may be interrupted.
/// This trap is resumable.
#[allow(non_upper_case_globals)]
pub const TrapCodeInterrupt: u32 = 11 << 16;

#[no_mangle]
pub extern "C" fn cranelift_get_pointer_type(ptr: *mut ModuleData) -> Type {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst
        .module
        .as_ref()
        .unwrap()
        .target_config()
        .pointer_type()
        .index() as u8;
}

#[no_mangle]
pub extern "C" fn cranelift_get_pointer_size_bytes(ptr: *mut ModuleData) -> u8 {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst
        .module
        .as_ref()
        .unwrap()
        .target_config()
        .pointer_bytes();
}

#[no_mangle]
pub extern "C" fn cranelift_clear_context(ptr: *mut ModuleData) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.ctx.clear();
}

fn convert_cc(cc: CraneliftCallConv, default: CallConv) -> CallConv {
    return match cc {
        CraneliftCallConv::CraneliftCallConvDefault => default,
        CraneliftCallConv::CraneliftCallConvSystemV => CallConv::SystemV,
        CraneliftCallConv::CraneliftCallConvWindowsFastcall => CallConv::WindowsFastcall,
        CraneliftCallConv::CraneliftCallConvProbestack => CallConv::Probestack,
        CraneliftCallConv::CraneliftCallConvFast => CallConv::Fast,
        CraneliftCallConv::CraneliftCallConvCold => CallConv::Cold,
    };
}

#[no_mangle]
pub extern "C" fn cranelift_signature_builder_reset(ptr: *mut ModuleData, cc: CraneliftCallConv) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    inst.ctx.func.signature.clear(convert_cc(
        cc,
        inst.module
            .as_ref()
            .unwrap()
            .target_config()
            .default_call_conv,
    ));
}

fn convert_type(typ: Type) -> codegen::ir::types::Type {
    #[allow(non_upper_case_globals)]
    return match typ {
        TypeI8 => I8,
        TypeI16 => I16,
        TypeI32 => I32,
        TypeI64 => I64,
        TypeI128 => I128,
        TypeF32 => F32,
        TypeF64 => F64,
        TypeR32 => R32,
        TypeR64 => R64,
        TypeI8X8 => I8X8,
        TypeI16X4 => I16X4,
        TypeI32X2 => I32X2,
        TypeF32X2 => F32X2,
        TypeI8X16 => I8X16,
        TypeI16X8 => I16X8,
        TypeI32X4 => I32X4,
        TypeI64X2 => I64X2,
        TypeF32X4 => F32X4,
        TypeF64X2 => F64X2,
        TypeF32X8 => F32X8,
        TypeF64X4 => F64X4,
        TypeF32X16 => F32X16,
        TypeF64X8 => F64X8,

        _ => I32,
    };
}

#[no_mangle]
pub extern "C" fn cranelift_signature_builder_add_param(ptr: *mut ModuleData, typ: Type) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    inst.ctx
        .func
        .signature
        .params
        .push(AbiParam::new(convert_type(typ)));
}

#[no_mangle]
pub extern "C" fn cranelift_signature_builder_add_result(ptr: *mut ModuleData, typ: Type) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    inst.ctx
        .func
        .signature
        .returns
        .push(AbiParam::new(convert_type(typ)));
}

#[no_mangle]
pub extern "C" fn cranelift_build_function(
    ptr: *mut ModuleData,
    userdata: usize,
    cb: fn(userdata: usize, builder: &mut FunctionData),
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let fnbuilder = FunctionBuilder::new(&mut inst.ctx.func, &mut inst.builder_ctx);

    let fd = &mut FunctionData {
        module: inst.module.as_mut().unwrap(),
        builder: fnbuilder,
        variable_counter: 0,
    };
    cb(userdata, fd);

    fd.builder.finalize()
}

#[no_mangle]
pub extern "C" fn cranelift_function_to_string(
    ptr: *mut ModuleData,
    userdata: usize,
    cb: fn(userdata: usize, str: *const c_char),
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let str = inst.ctx.func.to_string();
    cb(userdata, str.as_ptr() as *const c_char);
}

#[no_mangle]
pub extern "C" fn cranelift_set_source_loc(ptr: *mut FunctionData, loc: u32) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    inst.builder.set_srcloc(SourceLoc::new(loc))
}

type BlockCode = u32;
type VariableCode = u32;
type ValueCode = u32;
type ValueLabelCode = u32;
type JumpTableCode = u32;
type TableCode = u32;
type InstCode = u32;
type FuncRefCode = u32;
type SigRefCode = u32;
type HeapCode = u32;

#[no_mangle]
pub extern "C" fn cranelift_create_block(ptr: *mut FunctionData) -> BlockCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.builder.create_block().as_u32();
}

#[no_mangle]
pub extern "C" fn cranelift_switch_to_block(ptr: *mut FunctionData, block: BlockCode) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.builder.switch_to_block(Block::from_u32(block));
}

#[no_mangle]
pub extern "C" fn cranelift_seal_block(ptr: *mut FunctionData, block: BlockCode) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.builder.seal_block(Block::from_u32(block));
}

#[no_mangle]
pub extern "C" fn cranelift_seal_all_blocks(ptr: *mut FunctionData) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.builder.seal_all_blocks();
}

#[no_mangle]
pub extern "C" fn cranelift_append_block_params_for_function_params(
    ptr: *mut FunctionData,
    block: BlockCode,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    inst.builder
        .append_block_params_for_function_params(Block::from_u32(block))
}

#[no_mangle]
pub extern "C" fn cranelift_append_block_params_for_function_returns(
    ptr: *mut FunctionData,
    block: BlockCode,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    inst.builder
        .append_block_params_for_function_returns(Block::from_u32(block))
}

#[no_mangle]
pub extern "C" fn cranelift_block_params_count(ptr: *mut FunctionData, block: BlockCode) -> i32 {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let pars = inst.builder.block_params(Block::from_u32(block));
    return (*pars).len() as i32;
}

#[no_mangle]
pub extern "C" fn cranelift_block_params(
    ptr: *mut FunctionData,
    block: BlockCode,
    dest: *mut ValueCode,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let pars = inst.builder.block_params(Block::from_u32(block));
    for i in 0..(*pars).len() {
        unsafe { *dest.offset(i as isize) = (*pars)[i].as_u32() };
    }
}

#[no_mangle]
pub extern "C" fn cranelift_declare_var(ptr: *mut FunctionData, typ: Type) -> VariableCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let next = Variable::new(inst.variable_counter as usize);
    inst.variable_counter += 1;
    inst.builder.declare_var(next, convert_type(typ));
    return next.index() as u32;
}

#[no_mangle]
pub extern "C" fn cranelift_def_var(ptr: *mut FunctionData, var: VariableCode, val: ValueCode) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst
        .builder
        .def_var(Variable::with_u32(var), Value::from_u32(val));
}

#[no_mangle]
pub extern "C" fn cranelift_use_var(ptr: *mut FunctionData, var: VariableCode) -> ValueCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst.builder.use_var(Variable::with_u32(var)).as_u32();
}

#[no_mangle]
pub extern "C" fn cranelift_set_val_label(
    ptr: *mut FunctionData,
    val: ValueCode,
    label: ValueLabelCode,
) {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst
        .builder
        .set_val_label(Value::from_u32(val), ValueLabel::from_u32(label));
}

#[no_mangle]
pub extern "C" fn cranelift_create_jump_table(
    ptr: *mut FunctionData,
    count: u32,
    targets: *mut BlockCode,
) -> JumpTableCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let mut jt = JumpTableData::default_block();
    for i in 0..count {
        jt.push_entry(Block::from_u32(unsafe { *targets.offset(i as isize) }));
    }
    return inst.builder.create_jump_table(jt).as_u32();
}

#[no_mangle]
pub extern "C" fn cranelift_import_signature(
    ptr: *mut FunctionData,
    cc: CraneliftCallConv,
    argscount: u32,
    args: *mut Type,
    retcount: u32,
    rets: *mut Type,
) -> SigRefCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let mut sig = Signature::new(convert_cc(
        cc,
        inst.module.target_config().default_call_conv,
    ));
    for i in 0..argscount {
        sig.params.push(AbiParam::new(convert_type(unsafe {
            *args.offset(i as isize)
        })));
    }
    for i in 0..retcount {
        sig.returns.push(AbiParam::new(convert_type(unsafe {
            *rets.offset(i as isize)
        })));
    }

    return inst.builder.import_signature(sig).as_u32();
}

#[no_mangle]
pub extern "C" fn cranelift_declare_func_in_current_func(
    ptr: *mut FunctionData,
    source_id: u32,
) -> FuncRefCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    return inst
        .module
        .declare_func_in_func(FuncId::from_u32(source_id), inst.builder.func)
        .as_u32();
}

#[no_mangle]
pub extern "C" fn cranelift_ins_jump(
    ptr: *mut FunctionData,
    block: BlockCode,
    count: u32,
    args: *mut ValueCode,
) -> InstCode {
    let inst = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let mut argsdata = vec![];
    for i in 0..count {
        argsdata.push(Value::from_u32(unsafe { *args.offset(i as isize) }));
    }
    return inst
        .builder
        .ins()
        .jump(Block::from_u32(block), argsdata.as_mut_slice())
        .as_u32();
}
