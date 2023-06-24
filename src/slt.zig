const Allocator = @import("std").mem.Allocator;
const std = @import("std");
const ArrayList = @import("std").ArrayList;
const StringHashMap = @import("std").StringHashMap;
const testing = @import("std").testing;

pub const Namespace = u32;
pub const Index = u32;

pub const Slt = struct {
    ncounter: u32,
    vcounters: ArrayList(u32),
    namespaces: StringHashMap(Namespace),
    values: ArrayList(StringHashMap(Index)),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Slt {
        const namespaces = StringHashMap(Namespace).init(
            allocator,
        );
        const values = ArrayList(StringHashMap(Index)).init(allocator);
        const vcounters = ArrayList(u32).init(allocator);
        return Slt{
            .ncounter = 0,
            .namespaces = namespaces,
            .vcounters = vcounters,
            .values = values,
            .allocator = allocator,
        };
    }

    pub fn new_namespace(self: *Slt, name: []const u8) !Namespace {
        const temp = self.ncounter;
        try self.vcounters.append(0);
        try self.values.append(StringHashMap(Index).init(
            self.allocator,
        ));
        try self.namespaces.put(name, @as(Namespace, self.ncounter));
        self.ncounter += 1;
        return @as(Namespace, temp);
    }

    pub fn new_value(self: *Slt, namespace: Namespace, val: []const u8) !Index {
        const n = @as(u32, namespace);
        const temp = self.vcounters.items[n];
        try self.values.items[n].put(val, @as(Index, temp));
        self.vcounters.items[n] += 1;
        return @as(Index, temp);
    }

    pub fn get_namespace(self: Slt, name: []const u8) ?Namespace {
        return self.namespaces.get(name);
    }

    pub fn check_namespace(self: Slt, name: []const u8) bool {
        return self.namespaces.contains(name);
    }
    pub fn check_value(self: Slt, namespace: Namespace, val: []const u8) ?Index {
        const n = @as(u32, namespace);
        return self.values.items[n].contains(val);
    }

    pub fn get_value(self: Slt, namespace: Namespace, val: []const u8) ?Index {
        const n = @as(u32, namespace);
        return self.values.items[n].get(val);
    }

    pub fn deinit(self: *Slt) void {
        self.vcounters.deinit();
        self.namespaces.deinit();
        for (self.values.items) |*v| {
            v.deinit();
        }
        self.values.deinit();
    }
};

test "adds values and can return all" {
    const check = "not";
    const ns1 = "main";
    const ns2 = "add_32";
    const sym1 = "global";
    const sym2 = "global";
    var slt = Slt.init(testing.allocator);
    defer slt.deinit();
    const namespace1 = try slt.new_namespace(ns1);
    const namespace2 = try slt.new_namespace(ns2);
    const symbol1 = try slt.new_value(namespace1, sym1);
    const symbol2 = try slt.new_value(namespace2, sym2);

    try testing.expect(slt.check_namespace(check) == false);
    try testing.expect(slt.check_namespace(ns1) == true);
    try testing.expect(namespace1 == 0);
    try testing.expect(namespace2 == 1);
    try testing.expect(symbol1 == slt.get_value(namespace1, sym1).?);
    try testing.expect(symbol2 == slt.get_value(namespace2, sym2).?);
}
