const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;

pub const Func = struct {
    const Self = @This();

    ptr:[*]u32,
    arg_count:u8,
    marked:bool,

    upvals:[*]Var,
    upval_count:u16,

    pub fn init(ptr:[*]u32,arg_count:u8,upval_cap:u8) *Self {
        const upvals = Vm.gpa.alloc(Var, upval_cap) catch unreachable;
        var self = Vm.gpa.create(Func) catch unreachable;

        self.ptr = ptr;
        self.arg_count = arg_count;
        self.marked = false;
        self.upvals = upvals.ptr;
        self.upval_count = upval_cap;
        return self;
    }

    pub fn deinit(self:*Self) void {
        Vm.gpa.destroy(self);
    }
};