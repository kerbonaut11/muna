const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const Str = @import("str.zig").Str;
const Err = @import("err.zig").Err;

pub const Table = struct {
    const Self = @This();
    const Map = std.hash_map.HashMapUnmanaged(Var, Var, struct {
        pub fn eql(_:@This(),lhs:Var,rhs:Var) bool {
            return Table.hashEq(lhs, rhs);
        }

        pub fn hash(_:@This(),k:Var) u32 {
            return Table.hash(k);
        }
    }, 80);

    const mt_mask = ~@as(usize, 1); 

    arr:[*]Var,
    arr_cap:u32,
    arr_len:u32,

    map:Map,
    meta_table_bits:usize,

    pub fn init(arr_cap:u32) *Self {
        var self = Vm.gpa.create(Self) catch unreachable;
        const arr_slice = Vm.gpa.alloc(Var, arr_cap) catch unreachable;

        self.arr = arr_slice.ptr;
        self.arr_cap = arr_cap;
        self.arr_len = 0;

        self.map = Map.empty;
        self.meta_table_bits = 0;

        return self;
    }

    pub fn deinit(self: *Self) void {
        Vm.gpa.free(self.arrSlice());
        self.map.deinit(Vm.gpa);
        Vm.deinit(self);
    }

    pub fn getMetaTable(self: *const Self) ?*Self {
        return @ptrFromInt(self.meta_table_bits & mt_mask);
    }

    pub fn setMetaTable(self: *Self,mt: ?*Self) void {
        self.meta_table_bits &= 1;
        self.meta_table_bits |= @intFromPtr(mt);
    }

    fn reallocArr(self: *Self,new_cap:u32) void {
        Vm.gpa.free(self.arrSlice());
        self.arr = (Vm.gpa.alloc(Var, new_cap) catch unreachable).ptr;
        self.arr_cap = new_cap;
    }

    pub fn arrSlice(self: *Self) []Var {
        return self.arr[0..self.arr_cap];
    }

    fn validateKey(k:Var) !Var {
        return switch (k.tag()) {
            .nil => {
                Err.global = Err{.invalidIdx = k};
                return error.panic;
            },

            .float => {
                const float = k.as(f32);
                if (float == std.math.nan(f32)) {
                    Err.global = Err{.invalidIdx = k};
                    return error.panic;
                }

                if (@rem(float, 1.0) == 0.0) {
                    return Var.from(@as(i32,@intFromFloat(float)));
                }

                return k;
            },
            else => k,
        };
    }

    pub fn get(self:*const Self,_k:Var) !?Var {
        const k = try validateKey(_k);
        return self.getNoValidate(k);
    }

    pub fn getNoValidate(self:*const Self,k:Var) ?Var {
        if (k.tag() == .int) {
            const int = k.as(i32);
            if (int > 0 and int < self.arr_len) {
                return self.arr[@intCast(int)];
            }
        }

        if (self.map.get(k)) |x| {
            return x;
        }

        if (self.getMetaTable()) |mt| {
            return mt.getNoValidate(k);
        }

        return null;
    }

    pub fn set(self:*Self,_k:Var,v:Var) !void {
        const k = try validateKey(_k);
        self.setNoValidate(k, v);
    }

    pub fn setNoValidate(self:*Self,k:Var,v:Var) void {
        if (k.tag() == .int) {
            const int = k.as(i32);
            if (int > 0 and int < self.arr_len) {
                self.arr[@intCast(int)] = v;
                return;
            }
        }

        self.map.put(Vm.gpa, k, v) catch unreachable;
    }

    pub fn push(self:*Self, x:Var) void {
        if (self.arr_len == self.arr_cap) {
            self.reallocArr(self.arr_cap*2);
        }
        self.pushUnsafe(x);
    }

    pub fn pushUnsafe(self:*Self, x:Var) void {
        self.arr[self.arr_len] = x;
        self.arr_len += 1;
    }

    pub fn hash(k:Var) u32 {
        const bool_hash:u32 = 0xaaaaaaaa; 
        switch (k.tag()) {
            .bool => return if (k.as(bool)) bool_hash else ~bool_hash,
            .int => return @bitCast(k.as(i32)),
            .float => {
                const float = k.as(f32);
                if (@mod(float, 1.0) == 0.0) {
                    return @bitCast(k.floatToInt());
                } else {
                    return @bitCast(float);
                }
            },
            .str => return k.as(Str).hash(),

            else => unreachable,
        }
    }

    pub fn hashEq(lhs:Var,rhs:Var) bool {
        if (lhs.bits == rhs.bits) {
            return true;
        }

        if (lhs.tag() == .str and rhs.tag() == .str) {
            return std.mem.eql(u8,lhs.as(Str).asSlice(), rhs.as(Str).asSlice());
        }

        return false;
    }

    pub fn printDebug(self: *Self) void {
        std.debug.print("{s}", .{"{"});

        for (self.arrSlice()) |x| {
            x.printDebug();
            std.debug.print(", ", .{});
        }
        
        var iter = self.map.iterator();
        while (iter.next()) |e|{
            e.key_ptr.printDebug();
            std.debug.print("=", .{});
            e.value_ptr.printDebug();
            std.debug.print(", ", .{});
        }

        std.debug.print("{s}", .{"}"});
    }
};

test {
    std.debug.print("{}\n", .{@sizeOf(Table)});
    var t = Table.init(3);
    try t.set(Var.from(1), Var.from(20));
    try t.set(Var.false_val, Var.from(10));

    try std.testing.expectEqual(Var.from(20),t.get(Var.from(1)));
    try std.testing.expectEqual(Var.from(10),t.get(Var.false_val));
}