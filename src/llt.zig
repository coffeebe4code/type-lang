const Allocator = @import("std").mem.Allocator;
const std = @import("std");
const StringHashMap = @import("std").StringHashMap;
const testing = @import("std").testing;

pub const Local = u32;

pub const Llt = struct {
    counter: u32,
    locals: StringHashMap(Local),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Llt {
        const locals = StringHashMap(Local).init(
            allocator,
        );
        return Llt{
            .counter = 0,
            .locals = locals,
            .allocator = allocator,
        };
    }

    pub fn new_local(self: *Llt, name: []const u8) !Local {
        const temp = self.counter;
        try self.locals.put(name, @as(Local, self.counter));
        self.counter += 1;
        return @as(Local, temp);
    }

    pub fn get_local(self: Llt, name: []const u8) ?Local {
        return self.locals.get(name);
    }

    pub fn check_local(self: Llt, name: []const u8) bool {
        return self.locals.contains(name);
    }

    pub fn deinit(self: *Llt) void {
        self.locals.deinit();
    }
};

test "adds values and can return all" {
    const check = "not";
    const val1 = "x";
    const val2 = "y";
    var llt = Llt.init(testing.allocator);
    defer llt.deinit();
    const local1 = try llt.new_local(val1);
    const local2 = try llt.new_local(val2);

    try testing.expect(llt.check_local(check) == false);
    try testing.expect(llt.check_local(val1) == true);
    try testing.expect(local1 == 0);
    try testing.expect(local2 == 1);
    try testing.expect(local1 == llt.get_local(val1).?);
}
