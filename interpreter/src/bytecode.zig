const std = @import("std");
const Vm = @import("vm.zig").Vm;
const Var = @import("var.zig").Var;
const Str = @import("str.zig").Str;

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

    closure = 17,
    call    = 18,
    ret     = 19,

    bind_upval = 20,
    get_upval  = 21,
    set_upval  = 22,

    halt = 30,
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

    closure: packed struct {
        upval_cap:u8,
        arg_count:u8
    },

    call:void,
    ret:void,

    bind_upval:u16,
    get_upval:u16,
    set_upval:u16,

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

    list:std.ArrayList(u32),
    ip:[*]u32,
    name_table:[]Var,

    pub fn init(path:[]const u8) !Self {
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();
        const reader = file.reader();

        const name_table = loadNameTable(reader);
        const list = loadByteCode(reader);

        return .{
           .name_table = name_table,
           .list = list,
           .ip = @ptrCast(list.items.ptr),
        };
    }

    pub fn loadNameTable(reader:anytype) []Var {
        const name_count = reader.readInt(u16, .little)  catch unreachable;
        var name_table = Vm.gpa.alloc(Var, max_names) catch unreachable;
        var buffer:[std.math.maxInt(u16)]u8 = undefined;

        for (0..name_count) |i| {
            const slice = reader.readUntilDelimiter(buffer[0..], 0) catch unreachable;
            const str = Str.init(slice);
            std.debug.print("name {}:{c} \n", .{i,str.asSlice()});
            name_table[i] = Var.from(str);
        }

        return name_table;
    }

    pub fn loadByteCode(reader:anytype) std.ArrayList(u32) {
        var bytes = std.ArrayListAligned(u8, 4).init(Vm.gpa);
        reader.readAllArrayListAligned(4, &bytes, std.math.maxInt(usize)) catch unreachable;

        return .{
            .items = @ptrCast(bytes.items),
            .capacity = bytes.capacity/4,
            .allocator = Vm.gpa,
        };
    }

    pub fn deinit(self:*Self) void {
        self.list.deinit();
        Vm.page_a.free(self.name_table);
    }

    pub fn next(self: *Self,comptime T:type) T {
        const ptr:*const T = @ptrCast(@alignCast(self.ip));
        self.ip += 1;
        return ptr.*;
    }  
};
