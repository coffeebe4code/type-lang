const llvm_b = @import("./llvm-bindings.zig");
const strings = @import("./strings.zig");
const std = @import("std");

pub const CodeModel = enum {
    Static,
    PIC,
};

pub const Relocation = enum {
    Static,
    PIC,
};

pub const Optimize = enum {
    None,
    Full,
};

pub const ModuleOptions = struct {
    file_name: []const u8,
    target_arch: std.Target.Cpu.Arch,
    target_os: std.Target.Os.Tag,
    target_abi: std.Target.Abi,
    optimize: Optimize,
    pic: Relocation,
};

pub const Module = struct {
    context: *llvm_b.Context = undefined,
    module: *llvm_b.Module = undefined,
    options: ModuleOptions,
    pub fn init(options: ModuleOptions) !Module {
        const context = llvm_b.Context.create();
        errdefer context.dispose();

        const file = strings.sentinel(options.file_name);
        defer file.?.allocator.free();
        //const module = llvm_b.Module.createWithName(file.?.val, context);
        //errdefer module.dispose();
    }

    pub fn deinit(self: Module) void {
        self.module.dispose();
        self.context.dispose();
    }
};

test "should create context" {
    const module = try Module.init(.{
        .file_name = "first_lib",
    });
    defer module.deinit();
}
