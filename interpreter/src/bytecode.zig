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
    load_str           = 6,

    load  = 7,
    write = 8,
    pop   = 43,

    add    =  9,
    sub    = 10,
    mul    = 11,
    div    = 12,
    idiv   = 13,
    pow    = 14,
    mod    = 15,
    concat = 16,

    bin_and  = 31,
    bin_or   = 32,
    bin_xor  = 33,
    shl  = 40,
    shr  = 41,

    bool_and = 34,
    bool_or  = 35,

    less    = 26,
    less_eq = 27,
    eq      = 28,

    neg      = 36,
    bin_not  = 37,
    bool_not = 38,
    len      = 39,

    new_table  = 42,
    get        = 44,
    set        = 46,
    set_pop    = 47,
    get_method = 48,


    closure = 17,
    call    = 18,
    ret     = 19,

    bind_upval = 20,
    get_upval  = 21,
    set_upval  = 22,

    jump       = 23,
    jump_true  = 24,
    jump_false = 25,

    halt = 30,
};

pub const ByteCode = union(ByteCodeType) {
    const Type = ByteCodeType;

    load_nil:void,
    load_true:void,
    load_false:void,
    load_int:void,
    load_float:void,
    load_str:u16,


    load:u16,
    write:u16,
    pop:void,

    add:void,
    sub:void,
    mul:void,
    div:void,
    idiv:void,
    pow:void,
    mod:void,
    concat:void,

    bin_and:void,
    bin_or:void,
    bin_xor:void,
    shl:void,
    shr:void,

    bool_and:void,
    bool_or:void,  

    less:bool,
    less_eq:bool,
    eq:bool,

    neg:void,      
    bin_not:void,      
    bool_not:void, 
    len:void,      

    new_table: u16,
    get:void,
    set:void,
    set_pop:void,
    get_method:u16,

    closure: packed struct {
        upval_cap:u8,
        arg_count:u8
    },

    call:void,
    ret:void,

    bind_upval:u16,
    get_upval:u16,
    set_upval:u16,

    jump:i16,
    jump_true:i16,
    jump_false:i16,

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
        var name_table = Vm.page_a.alloc(Var, name_count) catch unreachable;
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
        for (self.name_table) |str| {
            str.as(Str).deinit();
        }
        Vm.page_a.free(self.name_table);
    }

    pub fn next(self: *Self,comptime T:type) T {
        const ptr:*const T = @ptrCast(@alignCast(self.ip));
        self.ip += 1;
        return ptr.*;
    }  
};
