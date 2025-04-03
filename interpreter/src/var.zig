const std = @import("std");
const Str = @import("str.zig").Str;
const Func = @import("func.zig").Func;
const Table = @import("table.zig").Table;

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

    pub const nil_val = fromTypeAndHigh(.nil, 0);
    pub const true_val = from(true);
    pub const false_val = from(false);

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
        return switch (@TypeOf(x)) {
            bool => fromTypeAndHigh(.bool, @intFromBool(x)),
            i32, comptime_int   => fromTypeAndHigh(.int, @bitCast(@as(i32, x))),
            f32, comptime_float => fromTypeAndHigh(.float, @bitCast(@as(f32, x))),
            Str    => fromTypeAndPtr(.str,   @ptrCast(x.ptr)),
            *Func  => fromTypeAndPtr(.func,  @ptrCast(x)),
            *Table => fromTypeAndPtr(.table, @ptrCast(x)),
            else => unreachable,
        };
    }

    pub fn as(self: Self, comptime T: type) T {
        return switch (T) {
            bool   => self.highBits() == 1,
            i32    => @bitCast(self.highBits()),
            f32    => @bitCast(self.highBits()),
            Str    => Str.fromPtr(self.asPtr()),
            *Func  => @ptrCast(@alignCast(self.asPtr())),
            *Table => @ptrCast(@alignCast(self.asPtr())),
            else => unreachable,
        };
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

    pub fn printDebug(self: Self) void {
        switch (self.tag()) {
            .nil => std.debug.print("nil", .{}),
            .bool => std.debug.print("{}", .{self.as(bool)}),
            .int => std.debug.print("{d}", .{self.as(i32)}),
            .float => std.debug.print("{d}", .{self.as(f32)}),
            .str => std.debug.print("'{s}'", .{self.as(Str).asSlice()}),
            .func => std.debug.print("func@{x}", .{@intFromPtr(self.as(*Func).ptr)}),
            .table => self.as(*Table).printDebug(),
            else => {}
        }
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
