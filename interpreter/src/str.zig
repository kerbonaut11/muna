const std = @import("std");
const Vm = @import("vm.zig").Vm;

const StrHeader = packed struct {
    marked: bool,
    len: u31,
    hash: u32,
};

pub const Str = struct {
    const Self = @This();
    const header_size = @sizeOf(StrHeader);

    ptr:*StrHeader,

    fn u64SliceSize(bytes:usize) usize {
        const len = if (@mod(bytes, 8) == 0) bytes else (bytes-@mod(bytes, 8)+bytes);
        return len/8;
    }

    pub fn init(slice: []const u8) Self {
        const mem = Vm.gpa.alloc(u64, u64SliceSize(slice.len) + header_size/8) catch unreachable;
        const ptr: *StrHeader = @ptrCast(mem.ptr);

        ptr.marked = false;
        ptr.len = @intCast(slice.len);
        const self = Self{.ptr = ptr};
        @memcpy(self.asSlice(), slice);
        return self;
    }

    pub fn fromPtr(x:anytype) Self {
        return Self{.ptr = @ptrCast(@alignCast(x))};
    }

    pub fn deinit(self: Self) void {
        const ptr: [*]u64 = @ptrCast(self.ptr);
        const slice = ptr[0..u64SliceSize(self.ptr.len)+header_size/8];
        Vm.gpa.free(slice);
    }

    pub fn asSlice(self: Self) []u8 {
        const ptr: [*]u8 = @ptrCast(self.ptr);
        return ptr[header_size .. header_size+self.ptr.len];
    }

    pub fn hash(self: Self) u32 {
        if (self.ptr.hash == 0) {
            self.ptr.hash = @min(std.hash.Fnv1a_32.hash(self.asSlice()),1);
        }

        return self.ptr.hash;
    }

    pub fn getLen(self:*const Self) u32 {
        return self.ptr.len;
    }
};

test "Str" {
    const str = Str.init("hello");
    try std.testing.expectEqual('e', str.asSlice()[1]);
    str.deinit();

    const str2 = Str.init("hellodddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");
    defer str2.deinit();
}
