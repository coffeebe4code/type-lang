const Allocator = @import("std").mem.Allocator;
const std = @import("std");
const ArrayList = @import("std").ArrayList;
const AutoHashMap = @import("std").AutoHashMap;
const testing = @import("std").testing;

pub const Namespace = u32;

pub const SltResult = struct {
    namespace: Namespace,
    idx: u32,
};
pub const Slt = struct {
    namespaces: ArrayList(AutoHashMap),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Slt {
        return Slt{
            .allocator = allocator,
        };
    }

    pub fn new_parse(allocator: Allocator) Namespace {
        return Slt{
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Slt) void {
        _ = self;
    }
};
