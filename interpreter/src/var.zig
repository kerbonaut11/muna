const std = @import("std");
const Str = @import("str.zig").Str;

pub const VarType = enum(u3) {
    nil,
    bool,
    int,
    float,

    str,
    table,
    func,
    zig,
};

pub const Var = struct {
    const Self = @This();
    pub const Type = VarType;

    bits: u64,

    pub const NIL = fromTypeAndHigh(.nil, 0);
    pub const TRUE = from(true);
    pub const FALSE = from(false);

    fn fromTypeAndHigh(ty: Type, high: u32) Self {
        return Self{ .bits = (@as(u64, high) << 32) | @intFromEnum(ty) };
    }

    fn fromTypeAndPtr(ty: Type, ptr: *void) Self {
        if (@import("builtin").mode == .Debug) {
            std.debug.assert(@intFromPtr(ptr) & 0x7 == 0);
        }
        return Self{ .bits = @intFromPtr(ptr) | @intFromEnum(ty) };
    }

    fn highBits(self: Self) u32 {
        return @intCast(self.bits >> 32);
    }

    fn asPtr(self: Self) *void {
        return @ptrFromInt(self.bits & ~@as(u64, 0x7));
    }

    pub fn from(x: anytype) Self {
        switch (@TypeOf(x)) {
            bool => return fromTypeAndHigh(.bool, @intFromBool(x)),
            i32, comptime_int   => return fromTypeAndHigh(.int, @bitCast(@as(i32, x))),
            f32, comptime_float => return fromTypeAndHigh(.float, @bitCast(@as(f32, x))),
            Str  => return fromTypeAndPtr(.str, @ptrCast(x.ptr)),
            else => unreachable,
        }
    }

    pub fn as(self: Self, comptime T: type) T {
        switch (T) {
            bool => return self.highBits() == 1,
            i32 => return @bitCast(self.highBits()),
            f32 => return @bitCast(self.highBits()),
            Str => return Str.fromPtr(self.asPtr()),
            else => unreachable,
        }
    }

    pub fn intToFloat(self:Self) f32 {
        return @floatFromInt(self.as(i32));
    }

    pub fn floatToInt(self:Self) i32 {
        return @intFromFloat(self.as(f32));
    }

    pub fn tag(self: Self) Type {
        return @enumFromInt(self.bits & 0x7);
    }

    pub fn hash_eq(a:Self,b:Self) bool {
        return a.bits == b.bits;
    }
};

test "Val" {
    const val1 = Var.from(0xffaa);
    try std.testing.expectEqual(0xffaa00000000|@as(u64,@intFromEnum(VarType.int)), val1.bits);
    try std.testing.expectEqual(0xffaa, val1.as(i32));
    try std.testing.expectEqual(.int, val1.tag());

    const ptr: *void = @ptrFromInt(0xffff0);
    const val2 = Var.fromTypeAndPtr(.table, ptr);
    try std.testing.expectEqual(0xffff0|@as(u64,@intFromEnum(VarType.table)), val2.bits);
}
