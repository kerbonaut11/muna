const std = @import("std");
const Vm = @import("vm.zig").Vm;
const Var = @import("var.zig").Var;

pub const ByteCodeType = enum(u8) {
    load_nil           = 0,
    load_true          = 1,
    load_false         = 2,
    load_int           = 3,
    load_float         = 4,
    load_to_name_table = 5,
    load_str           = 6,

    load  = 7,
    write = 8,

    add    =  9,
    sub    = 10,
    mul    = 11,
    div    = 12,
    idiv   = 13,
    pow    = 14,
    mod    = 15,
    concat = 16,

    halt = 17,
};

pub const ByteCode = union(ByteCodeType) {
    const Type = ByteCodeType;

    load_nil:void,
    load_true:void,
    load_false:void,
    load_int:void,
    load_float:void,
    load_to_name_table:u16,
    load_str:u16,


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
    const max_names = std.math.maxInt(u16);

    bytes:std.ArrayListAligned(u8,4),
    ip:[*]u32,
    name_table:[]Var,

    pub fn init(path:[]const u8) !Self {
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();

        var bytes = std.ArrayListAligned(u8,4).init(Vm.gpa);
        try file.reader().readAllArrayListAligned(4,&bytes, std.math.maxInt(usize));
        
        

        return .{
           .bytes= bytes,
           .ip = @ptrCast(bytes.items.ptr),
           .name_table = Vm.page_a.alloc(Var, max_names) catch unreachable
        };
    }

    pub fn deinit(self:*Self) void {
        self.bytes.deinit();
        Vm.page_a.free(self.name_table);
    }

    pub fn next(self: *Self,comptime T:type) T {
        const ptr:*const T = @ptrCast(@alignCast(self.ip));
        self.ip += 1;
        return ptr.*;
    }  
};
