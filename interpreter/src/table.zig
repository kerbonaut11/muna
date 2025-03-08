const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const Str = @import("str.zig").Str;
const Err = @import("err.zig").RunErr;

pub const Table = struct {
    const Self = @This();
    const KV = struct{v:Var,k:Var};
    const map_start_size = 4; 

    arr:[*]Var,
    arr_cap:u32,
    arr_len:u32,

    map:[*]std.SinglyLinkedList(KV),
    map_len:u16,
    map_mask:u8,
    map_inserts_till_realloc:u8,

    marked:bool,

    pub fn init(arr_cap:u32) Self {
        const arr_slice = Vm.gpa.alloc(Var, arr_cap) catch unreachable;
        const map_slice = Vm.gpa.alloc(std.SinglyLinkedList(KV), map_start_size)  catch unreachable;
        @memset(map_slice, std.SinglyLinkedList(KV){.first = null});

        return Self{
            .arr = arr_slice.ptr,
            .arr_cap = arr_cap,
            .arr_len = 0,

            .map = map_slice.ptr,
            .map_len = map_start_size,
            .map_mask = 0b11,
            .map_inserts_till_realloc = map_start_size*3,

            .marked = false,
        };
    }

    pub fn get(self:*Self,k:Var) !Var {
        if (k.tag() == .int) {
            const int = k.as(i32);
            if (int > 0 and int < self.arr_len) {
                return self.arr[@intCast(int)];
            }
        }

        const idx = try hash(k) & self.map_mask;
        var node = self.map[idx].first;
        while (node != null) {
            const data = node.?.data;
            if (data.k.hash_eq(k)) {
                return data.v;
            }
            node = node.?.next;
        }

        return Var.NIL;
    }

    pub fn set(self:*Self,k:Var,v:Var) !void {
        if (k.tag() == .int) {
            const int = k.as(i32);
            if (int > 0 and int < self.arr_len) {
                self.arr[@intCast(int)] = v;
                return;
            }
        }

        const idx = try hash(k) & self.map_mask;
        var prev = self.map[idx].first;
        var node = self.map[idx].first;
        while (node != null) {
            const data = node.?.data;
            if (data.k.hash_eq(k)) {
                node.?.data.v = v;
                return;
            }
            prev = node;
            node = node.?.next;
        }

        var new = Vm.gpa.create(std.SinglyLinkedList(KV).Node) catch unreachable;
        new.data = .{.k = k,.v = v};
        new.next = null;
        if (prev != null) {
            prev.?.next = new;
        } else {
            self.map[idx].first = new;
        }
    }

    pub fn hash(k:Var) !u32 {
        const bool_hash:u32 = 0xaaaaaaaa; 
        switch (k.tag()) {
            .nil  => {
                Err.global = Err{.invalidIdx = k};
                return error.run;
            },
            .bool => return if (k.as(bool)) bool_hash else ~bool_hash,
            .char => unreachable,
            .int => return @bitCast(k.as(i32)),
            .float => {
                const float = k.as(f32);
                if (float == std.math.nan(f32)) {
                    Err.global = Err{.invalidIdx = k};
                    return error.run;
                }

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
};

test {
    var t = Table.init(3);
    try t.set(Var.from(1), Var.from(20));
    try t.set(Var.FALSE, Var.from(10));

    try std.testing.expectEqual(Var.from(20),try t.get(Var.from(1)));
    try std.testing.expectEqual(Var.from(10),try t.get(Var.FALSE));
}