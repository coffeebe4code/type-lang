const std = @import("std");

const count = 5;

const names = [count][]const u8{
    "token", "span", "lexer", "ast", "parser",
};
const files = [count][]const u8{
    "src/token.zig",
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
        s_lib.linkSystemLibrary("unwind");
        // TODO:: use debug/release with an option, and supported targets
        s_lib.addLibraryPath("./cranelift/target/debug");
        s_lib.addIncludePath("./cranelift/headers");
        //const rustc_lib = b.exec(&.{ "rustc", "--print=sysroot" });
        //var tokenizer = std.mem.tokenize(u8, rustc_lib, "\r\n");
        //const path_unpadded = tokenizer.next().?;
        //if (std.mem.eql(u8, path_unpadded, rustc_lib)) {
        //    std.debug.print("Unable to determine path to {s}\n", .{rustc_lib});
        //}
        b.installArtifact(s_lib);

        const s_tests = b.addTest(.{
            .root_source_file = .{ .path = cranelift_files[i] },
            .target = target,
            .optimize = optimize,
        });
        s_tests.addLibraryPath("./cranelift/target/debug");
        s_tests.addIncludePath("./cranelift/headers");
        s_tests.linkSystemLibrary("craneliftc");
        s_tests.linkSystemLibrary("unwind");

        const run_tests = b.addRunArtifact(s_tests);

        test_step.dependOn(&run_tests.step);
    }
}
