const std = @import("std");
const Vm = @import("vm.zig").Vm;
const Var = @import("var.zig").Var;

pub const ByteCodeType = enum(u8) {
    load_nil,
    load_true,
    load_false,
    load_int,
    load_float,

    load,
    write,

    add,
    sub,
    mul,
    div,
    idiv,
    pow,
    mod,
    concat,
    halt,
};

pub const ByteCode = union(ByteCodeType) {
    const Type = ByteCodeType;

    load_nil:void,
    load_true:void,
    load_false:void,
    load_int:void,
    load_float:void,

    load:u16,
    write:u16,

    add:void,
    sub:void,
    mul:void,
    div:void,
    idiv:void,
    pow:void,
    mod:void,
    concat:void,

    halt:void,

    pub fn asInt(self:*const ByteCode) u32 {
        const ptr:*const u32 = @ptrCast(@alignCast(self));
        return ptr.*;
    }

    pub fn fromInt(x:u32) ByteCode {
        const ptr:*const ByteCode = @ptrCast(@alignCast(&x));
        return ptr.*;
    }
};

pub const Program = struct {
    const Self = @This();

    list:std.ArrayList(u32),
    ip:[*]u32,

    pub fn init() Program {
        const list = std.ArrayList(u32).init(Vm.gpa);
        return .{
            .ip = undefined,
            .list = list,
        };
    }

    pub fn deinit(self:*Self) void {
        self.list.deinit();
    }

    pub fn encode(self:*Self,x:anytype) void {
        switch (@TypeOf(x)) {
            ByteCodeType => {
                const bc:ByteCode = switch (@as(ByteCodeType,x)) {
                    .load_nil   => .load_nil,
                    .load_true  => .load_true,
                    .load_false => .load_false,
                    .load_int   => .load_int,
                    .load_float => .load_float,

                    .add => .add,
                    .sub => .sub,
                    .mul => .mul,
                    .div => .div,
                    .pow => .pow,
                    .mod => .mod,

                    else => unreachable,
                };
                const ptr:*const u32 = @ptrCast(@alignCast(&bc));
                self.list.append(ptr.*) catch unreachable;
            },

            ByteCode => {
                const ptr:*const u32 = @ptrCast(@alignCast(&x));
                self.list.append(ptr.*) catch unreachable;
            },

            i32,comptime_int => {
                const int:i32 = x; 
                self.list.append(@bitCast(int)) catch unreachable;
            },

            f32,comptime_float => {
                const float:f32 = x; 
                self.list.append(@bitCast(float)) catch unreachable;
            },

            else => unreachable,
        }
    }

    pub fn loadFromFile(path:[]const u8) !Self {
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();

        var bytes = std.ArrayListAligned(u8,4).init(Vm.gpa);
        try file.reader().readAllArrayListAligned(4,&bytes, std.math.maxInt(usize));

        return .{
           .list = .{
                .items = @ptrCast(@alignCast(bytes.items)),
                .capacity = bytes.capacity/4,
                .allocator = Vm.gpa
           },
           .ip = undefined
        };
    }

    pub fn start(self: *Self) void {
        self.ip = self.list.items.ptr;
    }

    pub fn next(self: *Self,comptime T:type) T {
        const ptr:*const T = @ptrCast(@alignCast(self.ip));
        self.ip += 1;
        return ptr.*;
    }  
};


test "compat" {
    var p = try Program.loadFromFile("test.out");
    p.start();
    var vm = Vm.init(p);
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try std.testing.expectEqual(20.1, vm.pop().as(f32));
}
