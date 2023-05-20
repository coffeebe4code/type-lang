const llvm_b = @import("./llvm-bindings.zig");
const build_options = @import("build_options");
const std = @import("std");
const Allocator = @import("std").mem.Allocator;

pub const CodeModel = enum {
    default,
    tiny,
    small,
    kernel,
    medium,
    large,
};

pub const Relocation = enum {
    Static,
    PIC,
};

pub const Optimize = enum {
    None,
    Full,
};

pub const GlobalOptions = struct {
    target_arch: std.Target.Cpu.Arch,
    target_os: std.Target.Os.Tag,
    target_abi: std.Target.Abi,
    target_model: std.Target.Cpu.Model,
    code_model: CodeModel,
};

pub const ModuleOptions = struct {
    file_name: []const u8,
    optimize: Optimize,
    pic: Relocation,
};

pub const GlobalContext = struct {
    context: *llvm_b.Context,
    allocator: Allocator,
    options: GlobalOptions,
    code_model: llvm_b.CodeModel,
    target: *llvm_b.Target,

    pub fn init(allocator: std.mem.Allocator, opts: GlobalOptions) !Module {
        const context = llvm_b.Context.create();
        errdefer context.dispose();

        initializeLLVMTarget(opts.target_arch);

        const llvm_target_triple = try targetTriple(allocator, opts);
        defer allocator.free(llvm_target_triple);

        var error_message: [*:0]const u8 = undefined;
        var target: *llvm_b.Target = undefined;
        if (llvm_b.Target.getFromTriple(llvm_target_triple.ptr, &target, &error_message).toBool()) {
            defer llvm_b.disposeMessage(error_message);

            std.log.err("LLVM failed to parse '{s}': {s}", .{ llvm_target_triple, error_message });
            return error.InvalidLlvmTriple;
        }

        const opt_level: llvm_b.CodeGenOptLevel = if (opts.optimize_mode == .Debug)
            .None
        else
            .Aggressive;

        const reloc_mode: llvm_b.RelocMode = if (opts.reloc_mode)
            .PIC
        else
            .Static;

        const code_model: llvm_b.CodeModel = switch (opts.code_model) {
            .default => .Default,
            .tiny => .Tiny,
            .small => .Small,
            .kernel => .Kernel,
            .medium => .Medium,
            .large => .Large,
        };
        _ = code_model;

        const float_abi: llvm_b.ABIType = .Default;
        _ = float_abi;

        const target_machine = llvm_b.TargetMachine.create(
            target,
            llvm_target_triple.ptr,
            if (opts.target_model.llvm_name) |s| s.ptr else null,
            opts.llvm_cpu_features,
            opt_level,
            reloc_mode,
            code_model,
            opts.function_sections,
            float_abi,
            if (target_util.llvmMachineAbi(opts.target)) |s| s.ptr else null,
        );
        return GlobalContext{
            .context = context,
            .options = opts,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *GlobalContext) void {
        self.context.dispose();
    }
};

pub const Module = struct {
    context: GlobalContext = undefined,
    allocator: Allocator = undefined,
    module: *llvm_b.Module = undefined,
    options: ModuleOptions,
    pub fn init(allocator: Allocator, context: GlobalContext, opts: ModuleOptions) !Module {
        _ = opts;
        _ = context;
        _ = allocator;
    }

    pub fn deinit(self: Module) void {
        self.module.dispose();
    }
};

fn initializeLLVMTarget(arch: std.Target.Cpu.Arch) void {
    switch (arch) {
        .aarch64, .aarch64_be, .aarch64_32 => {
            llvm_b.LLVMInitializeAArch64Target();
            llvm_b.LLVMInitializeAArch64TargetInfo();
            llvm_b.LLVMInitializeAArch64TargetMC();
            llvm_b.LLVMInitializeAArch64AsmPrinter();
            llvm_b.LLVMInitializeAArch64AsmParser();
        },
        .amdgcn => {
            llvm_b.LLVMInitializeAMDGPUTarget();
            llvm_b.LLVMInitializeAMDGPUTargetInfo();
            llvm_b.LLVMInitializeAMDGPUTargetMC();
            llvm_b.LLVMInitializeAMDGPUAsmPrinter();
            llvm_b.LLVMInitializeAMDGPUAsmParser();
        },
        .thumb, .thumbeb, .arm, .armeb => {
            llvm_b.LLVMInitializeARMTarget();
            llvm_b.LLVMInitializeARMTargetInfo();
            llvm_b.LLVMInitializeARMTargetMC();
            llvm_b.LLVMInitializeARMAsmPrinter();
            llvm_b.LLVMInitializeARMAsmParser();
        },
        .avr => {
            llvm_b.LLVMInitializeAVRTarget();
            llvm_b.LLVMInitializeAVRTargetInfo();
            llvm_b.LLVMInitializeAVRTargetMC();
            llvm_b.LLVMInitializeAVRAsmPrinter();
            llvm_b.LLVMInitializeAVRAsmParser();
        },
        .bpfel, .bpfeb => {
            llvm_b.LLVMInitializeBPFTarget();
            llvm_b.LLVMInitializeBPFTargetInfo();
            llvm_b.LLVMInitializeBPFTargetMC();
            llvm_b.LLVMInitializeBPFAsmPrinter();
            llvm_b.LLVMInitializeBPFAsmParser();
        },
        .hexagon => {
            llvm_b.LLVMInitializeHexagonTarget();
            llvm_b.LLVMInitializeHexagonTargetInfo();
            llvm_b.LLVMInitializeHexagonTargetMC();
            llvm_b.LLVMInitializeHexagonAsmPrinter();
            llvm_b.LLVMInitializeHexagonAsmParser();
        },
        .lanai => {
            llvm_b.LLVMInitializeLanaiTarget();
            llvm_b.LLVMInitializeLanaiTargetInfo();
            llvm_b.LLVMInitializeLanaiTargetMC();
            llvm_b.LLVMInitializeLanaiAsmPrinter();
            llvm_b.LLVMInitializeLanaiAsmParser();
        },
        .mips, .mipsel, .mips64, .mips64el => {
            llvm_b.LLVMInitializeMipsTarget();
            llvm_b.LLVMInitializeMipsTargetInfo();
            llvm_b.LLVMInitializeMipsTargetMC();
            llvm_b.LLVMInitializeMipsAsmPrinter();
            llvm_b.LLVMInitializeMipsAsmParser();
        },
        .msp430 => {
            llvm_b.LLVMInitializeMSP430Target();
            llvm_b.LLVMInitializeMSP430TargetInfo();
            llvm_b.LLVMInitializeMSP430TargetMC();
            llvm_b.LLVMInitializeMSP430AsmPrinter();
            llvm_b.LLVMInitializeMSP430AsmParser();
        },
        .nvptx, .nvptx64 => {
            llvm_b.LLVMInitializeNVPTXTarget();
            llvm_b.LLVMInitializeNVPTXTargetInfo();
            llvm_b.LLVMInitializeNVPTXTargetMC();
            llvm_b.LLVMInitializeNVPTXAsmPrinter();
            // There is no LLVMInitializeNVPTXAsmParser function available.
        },
        .powerpc, .powerpcle, .powerpc64, .powerpc64le => {
            llvm_b.LLVMInitializePowerPCTarget();
            llvm_b.LLVMInitializePowerPCTargetInfo();
            llvm_b.LLVMInitializePowerPCTargetMC();
            llvm_b.LLVMInitializePowerPCAsmPrinter();
            llvm_b.LLVMInitializePowerPCAsmParser();
        },
        .riscv32, .riscv64 => {
            llvm_b.LLVMInitializeRISCVTarget();
            llvm_b.LLVMInitializeRISCVTargetInfo();
            llvm_b.LLVMInitializeRISCVTargetMC();
            llvm_b.LLVMInitializeRISCVAsmPrinter();
            llvm_b.LLVMInitializeRISCVAsmParser();
        },
        .sparc, .sparc64, .sparcel => {
            llvm_b.LLVMInitializeSparcTarget();
            llvm_b.LLVMInitializeSparcTargetInfo();
            llvm_b.LLVMInitializeSparcTargetMC();
            llvm_b.LLVMInitializeSparcAsmPrinter();
            llvm_b.LLVMInitializeSparcAsmParser();
        },
        .s390x => {
            llvm_b.LLVMInitializeSystemZTarget();
            llvm_b.LLVMInitializeSystemZTargetInfo();
            llvm_b.LLVMInitializeSystemZTargetMC();
            llvm_b.LLVMInitializeSystemZAsmPrinter();
            llvm_b.LLVMInitializeSystemZAsmParser();
        },
        .wasm32, .wasm64 => {
            llvm_b.LLVMInitializeWebAssemblyTarget();
            llvm_b.LLVMInitializeWebAssemblyTargetInfo();
            llvm_b.LLVMInitializeWebAssemblyTargetMC();
            llvm_b.LLVMInitializeWebAssemblyAsmPrinter();
            llvm_b.LLVMInitializeWebAssemblyAsmParser();
        },
        .x86, .x86_64 => {
            llvm_b.LLVMInitializeX86Target();
            llvm_b.LLVMInitializeX86TargetInfo();
            llvm_b.LLVMInitializeX86TargetMC();
            llvm_b.LLVMInitializeX86AsmPrinter();
            llvm_b.LLVMInitializeX86AsmParser();
        },
        .xtensa => {
            if (build_options.llvm_has_xtensa) {
                llvm_b.LLVMInitializeXtensaTarget();
                llvm_b.LLVMInitializeXtensaTargetInfo();
                llvm_b.LLVMInitializeXtensaTargetMC();
                llvm_b.LLVMInitializeXtensaAsmPrinter();
                llvm_b.LLVMInitializeXtensaAsmParser();
            }
        },
        .xcore => {
            llvm_b.LLVMInitializeXCoreTarget();
            llvm_b.LLVMInitializeXCoreTargetInfo();
            llvm_b.LLVMInitializeXCoreTargetMC();
            llvm_b.LLVMInitializeXCoreAsmPrinter();
            // There is no LLVMInitializeXCoreAsmParser function.
        },
        .m68k => {
            if (build_options.llvm_has_m68k) {
                llvm_b.LLVMInitializeM68kTarget();
                llvm_b.LLVMInitializeM68kTargetInfo();
                llvm_b.LLVMInitializeM68kTargetMC();
                llvm_b.LLVMInitializeM68kAsmPrinter();
                llvm_b.LLVMInitializeM68kAsmParser();
            }
        },
        .csky => {
            if (build_options.llvm_has_csky) {
                llvm_b.LLVMInitializeCSKYTarget();
                llvm_b.LLVMInitializeCSKYTargetInfo();
                llvm_b.LLVMInitializeCSKYTargetMC();
                // There is no LLVMInitializeCSKYAsmPrinter function.
                llvm_b.LLVMInitializeCSKYAsmParser();
            }
        },
        .ve => {
            llvm_b.LLVMInitializeVETarget();
            llvm_b.LLVMInitializeVETargetInfo();
            llvm_b.LLVMInitializeVETargetMC();
            llvm_b.LLVMInitializeVEAsmPrinter();
            llvm_b.LLVMInitializeVEAsmParser();
        },
        .arc => {
            if (build_options.llvm_has_arc) {
                llvm_b.LLVMInitializeARCTarget();
                llvm_b.LLVMInitializeARCTargetInfo();
                llvm_b.LLVMInitializeARCTargetMC();
                llvm_b.LLVMInitializeARCAsmPrinter();
                // There is no LLVMInitializeARCAsmParser function.
            }
        },

        // LLVM backends that have no initialization functions.
        .tce,
        .tcele,
        .r600,
        .le32,
        .le64,
        .amdil,
        .amdil64,
        .hsail,
        .hsail64,
        .shave,
        .spir,
        .spir64,
        .kalimba,
        .renderscript32,
        .renderscript64,
        .dxil,
        .loongarch32,
        .loongarch64,
        => {},

        .spu_2 => unreachable, // LLVM does not support this backend
        .spirv32 => unreachable, // LLVM does not support this backend
        .spirv64 => unreachable, // LLVM does not support this backend
    }
}

pub fn targetTriple(allocator: Allocator, opts: GlobalOptions) ![:0]u8 {
    var llvm_triple = std.ArrayList(u8).init(allocator);
    defer llvm_triple.deinit();

    const llvm_arch = switch (opts.target_arch) {
        .arm => "arm",
        .armeb => "armeb",
        .aarch64 => "aarch64",
        .aarch64_be => "aarch64_be",
        .aarch64_32 => "aarch64_32",
        .arc => "arc",
        .avr => "avr",
        .bpfel => "bpfel",
        .bpfeb => "bpfeb",
        .csky => "csky",
        .dxil => "dxil",
        .hexagon => "hexagon",
        .loongarch32 => "loongarch32",
        .loongarch64 => "loongarch64",
        .m68k => "m68k",
        .mips => "mips",
        .mipsel => "mipsel",
        .mips64 => "mips64",
        .mips64el => "mips64el",
        .msp430 => "msp430",
        .powerpc => "powerpc",
        .powerpcle => "powerpcle",
        .powerpc64 => "powerpc64",
        .powerpc64le => "powerpc64le",
        .r600 => "r600",
        .amdgcn => "amdgcn",
        .riscv32 => "riscv32",
        .riscv64 => "riscv64",
        .sparc => "sparc",
        .sparc64 => "sparc64",
        .sparcel => "sparcel",
        .s390x => "s390x",
        .tce => "tce",
        .tcele => "tcele",
        .thumb => "thumb",
        .thumbeb => "thumbeb",
        .x86 => "i386",
        .x86_64 => "x86_64",
        .xcore => "xcore",
        .xtensa => "xtensa",
        .nvptx => "nvptx",
        .nvptx64 => "nvptx64",
        .le32 => "le32",
        .le64 => "le64",
        .amdil => "amdil",
        .amdil64 => "amdil64",
        .hsail => "hsail",
        .hsail64 => "hsail64",
        .spir => "spir",
        .spir64 => "spir64",
        .spirv32 => "spirv32",
        .spirv64 => "spirv64",
        .kalimba => "kalimba",
        .shave => "shave",
        .lanai => "lanai",
        .wasm32 => "wasm32",
        .wasm64 => "wasm64",
        .renderscript32 => "renderscript32",
        .renderscript64 => "renderscript64",
        .ve => "ve",
        .spu_2 => return error.@"LLVM backend does not support SPU Mark II",
    };
    try llvm_triple.appendSlice(llvm_arch);
    try llvm_triple.appendSlice("-unknown-");

    const llvm_os = switch (opts.target_os) {
        .freestanding => "unknown",
        .ananas => "ananas",
        .cloudabi => "cloudabi",
        .dragonfly => "dragonfly",
        .freebsd => "freebsd",
        .fuchsia => "fuchsia",
        .kfreebsd => "kfreebsd",
        .linux => "linux",
        .lv2 => "lv2",
        .netbsd => "netbsd",
        .openbsd => "openbsd",
        .solaris => "solaris",
        .windows => "windows",
        .zos => "zos",
        .haiku => "haiku",
        .minix => "minix",
        .rtems => "rtems",
        .nacl => "nacl",
        .aix => "aix",
        .cuda => "cuda",
        .nvcl => "nvcl",
        .amdhsa => "amdhsa",
        .ps4 => "ps4",
        .ps5 => "ps5",
        .elfiamcu => "elfiamcu",
        .mesa3d => "mesa3d",
        .contiki => "contiki",
        .amdpal => "amdpal",
        .hermit => "hermit",
        .hurd => "hurd",
        .wasi => "wasi",
        .emscripten => "emscripten",
        .uefi => "windows",
        .macos => "macosx",
        .ios => "ios",
        .tvos => "tvos",
        .watchos => "watchos",
        .driverkit => "driverkit",
        .shadermodel => "shadermodel",
        .opencl,
        .glsl450,
        .vulkan,
        .plan9,
        .other,
        => "unknown",
    };
    try llvm_triple.appendSlice(llvm_os);

    if (opts.target_os.isDarwin()) {
        const min_version = opts.target_os.version_range.semver.min;
        try llvm_triple.writer().print("{d}.{d}.{d}", .{
            min_version.major,
            min_version.minor,
            min_version.patch,
        });
    }
    try llvm_triple.append('-');

    const llvm_abi = switch (opts.target_abi) {
        .none => "unknown",
        .gnu => "gnu",
        .gnuabin32 => "gnuabin32",
        .gnuabi64 => "gnuabi64",
        .gnueabi => "gnueabi",
        .gnueabihf => "gnueabihf",
        .gnuf32 => "gnuf32",
        .gnuf64 => "gnuf64",
        .gnusf => "gnusf",
        .gnux32 => "gnux32",
        .gnuilp32 => "gnuilp32",
        .code16 => "code16",
        .eabi => "eabi",
        .eabihf => "eabihf",
        .android => "android",
        .musl => "musl",
        .musleabi => "musleabi",
        .musleabihf => "musleabihf",
        .muslx32 => "muslx32",
        .msvc => "msvc",
        .itanium => "itanium",
        .cygnus => "cygnus",
        .coreclr => "coreclr",
        .simulator => "simulator",
        .macabi => "macabi",
        .pixel => "pixel",
        .vertex => "vertex",
        .geometry => "geometry",
        .hull => "hull",
        .domain => "domain",
        .compute => "compute",
        .library => "library",
        .raygeneration => "raygeneration",
        .intersection => "intersection",
        .anyhit => "anyhit",
        .closesthit => "closesthit",
        .miss => "miss",
        .callable => "callable",
        .mesh => "mesh",
        .amplification => "amplification",
    };
    try llvm_triple.appendSlice(llvm_abi);

    return llvm_triple.toOwnedSliceSentinel(0);
}
test "should create context" {
    const module = try Module.init(.{
        .file_name = "first_lib",
    });
    defer module.deinit();
}
