const std = @import("std");

const count = 7;

const names = [count][]const u8{
    "token",
    "slt",
    "llt",
    "span",
    "lexer",
    "ast",
    "parser",
};
const files = [count][]const u8{
    "src/token.zig",
    "src/slt.zig",
    "src/llt.zig",
    "src/span.zig",
    "src/lexer.zig",
    "src/ast.zig",
    "src/parser.zig",
};

const cranelift_count = 1;
const cranelift_names = [cranelift_count][]const u8{
    "cranelift",
};
const cranelift_files = [cranelift_count][]const u8{
    "src/cranelift.zig",
};

pub fn build(b: *std.Build) void {
    //    const cl_debug = b.option(bool, "cl_debug", "use the debug path for cranelift") orelse false;
    //    _ = cl_debug;
    //    const cl_target = b.option(?[]const u8, "cl_target", "use the provided target for cranelift") orelse undefined;
    //    _ = cl_target;
    const target = b.standardTargetOptions(.{});

    const optimize = b.standardOptimizeOption(.{});

    const test_step = b.step("test", "Run library tests step");

    for (0..count) |i| {
        const s_lib = b.addStaticLibrary(.{
            .name = names[i],
            .root_source_file = .{ .path = files[i] },
            .target = target,
            .optimize = optimize,
        });

        b.installArtifact(s_lib);

        const s_tests = b.addTest(.{
            .root_source_file = .{ .path = files[i] },
            .target = target,
            .optimize = optimize,
        });

        const run_tests = b.addRunArtifact(s_tests);

        test_step.dependOn(&run_tests.step);
    }

    for (0..cranelift_count) |i| {
        const s_lib = b.addStaticLibrary(.{
            .name = cranelift_names[i],
            .root_source_file = .{ .path = cranelift_files[i] },
            .target = target,
            .optimize = optimize,
        });
        s_lib.linkSystemLibrary("craneliftc");
        if (target.isWindows()) {
            // TODO:: fix windows when zig fixes windows.
            s_lib.linkSystemLibrary("msvcrt");
        }
        s_lib.linkSystemLibrary("unwind");
        // TODO:: use debug/release with an option, and supported targets
        const cl_path = "../craneliftc/target/release";
        //var cl_path = "../craneliftc/target";
        //if (cl_target) |ctarget| {
        //    cl_path = _path ++ "/" ++ ctarget;
        //}
        //if (cl_debug) {
        //    cl_path = cl_path ++ "/debug";
        //} else {
        //    cl_path = cl_path ++ "/release";
        //}

        s_lib.addLibraryPath(cl_path);
        s_lib.addIncludePath("../craneliftc/headers");
        b.installArtifact(s_lib);

        const s_tests = b.addTest(.{
            .root_source_file = .{ .path = cranelift_files[i] },
            .target = target,
            .optimize = optimize,
        });
        s_tests.addLibraryPath(cl_path);
        s_tests.addIncludePath("../craneliftc/headers");
        s_tests.linkSystemLibrary("craneliftc");
        if (target.isWindows()) {
            s_tests.linkSystemLibrary("msvcrt");
        }
        s_tests.linkSystemLibrary("unwind");

        const run_tests = b.addRunArtifact(s_tests);

        test_step.dependOn(&run_tests.step);
    }
}
