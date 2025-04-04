const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const Err = @import("err.zig").Err;
const ReturnCode = @import("err.zig").ReturnCode;

pub const Func = struct {
    const Self = @This();

    ptr:[*]const u32,
    arg_count:u8,
    is_callback:bool,
    marked:bool,

    upvals:[*]Var,
    upval_count:u16,

    pub const CallStackEntry = struct {
        bp:[*]Var,
        ip:[*]const u32,
        upval_ctx:[*]Var,
    };

    pub const CallStack = std.ArrayList(CallStackEntry);

    pub fn init(ptr:[*]const u32,arg_count:u8,upval_cap:u8) *Self {
        const upvals = Vm.gpa.alloc(Var, upval_cap) catch unreachable;
        var self = Vm.gpa.create(Func) catch unreachable;

        self.ptr = ptr;
        self.arg_count = arg_count;
        self.marked = false;
        self.is_callback = false;
        self.upvals = upvals.ptr;
        self.upval_count = upval_cap;
        return self;
    }

    pub fn initCallBack(func:*const fn(*Vm,[]Var) ReturnCode!Var,arg_count:u8) *Self {
        var self = Vm.gpa.create(Func) catch unreachable;

        self.ptr = @ptrCast(@alignCast(func));
        self.arg_count = arg_count;
        self.marked = false;
        self.is_callback = true;
        self.upvals = undefined;
        self.upval_count = 0;
        return self;
    }

    pub fn wrap(func:anytype) *Func {
        const F = @TypeOf(func);
        const info = switch (@typeInfo(F)) {
            .@"fn" => |x| x,
            else => @compileError("expected function"),
        };

        if (info.params[0].type.? != *Vm) {
            @compileError("first param must be *Vm");
        }

        const Wraper = struct {
            fn castParam(x:Var, comptime n:u8, comptime T:type) !T {
                if (T == Var) {
                    return x;
                } else {
                    if (x.tryAs(T)) |cast| {
                        return cast;
                    } else {
                        Err.global = Err{.paramTypeErr = .{
                            .param = n,
                            .expected = Var.Type.ofType(T),
                            .got = x.tag(),
                        }};
                        return error.panic;
                    }
                }
            }

            fn castReturn(x:anytype) Var {
                if (@TypeOf(x) == Var) {
                    return x;
                } else {
                    return Var.from(x);
                }
            }

            fn wraped(vm:*Vm,args:[]Var) !Var {
                return switch (info.params.len) {
                    1 => castReturn(try func(vm)),
                    2 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                    )),
                    3 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                        try castParam(args[1], 1, info.params[2].type.?),
                    )),
                    4 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                        try castParam(args[1], 1, info.params[2].type.?),
                        try castParam(args[2], 2, info.params[3].type.?),
                    )),
                    5 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                        try castParam(args[1], 1, info.params[2].type.?),
                        try castParam(args[2], 2, info.params[3].type.?),
                        try castParam(args[3], 3, info.params[4].type.?),
                    )),
                    6 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                        try castParam(args[1], 1, info.params[2].type.?),
                        try castParam(args[2], 2, info.params[3].type.?),
                        try castParam(args[3], 3, info.params[4].type.?),
                        try castParam(args[4], 4, info.params[5].type.?),
                    )),
                    7 => castReturn(try func(vm,
                        try castParam(args[0], 0, info.params[1].type.?),
                        try castParam(args[1], 1, info.params[2].type.?),
                        try castParam(args[2], 2, info.params[3].type.?),
                        try castParam(args[3], 3, info.params[4].type.?),
                        try castParam(args[4], 4, info.params[5].type.?),
                        try castParam(args[5], 5, info.params[6].type.?),
                    )),

                    else => @compileError("to many parameters"),
                };
            }
        };

        return Func.initCallBack(Wraper.wraped, info.params.len);
    }

    pub fn call(self: *Self, arg_count:u8, vm: *Vm) !void {
        if (arg_count > self.arg_count) {
            for (0..arg_count-self.arg_count) |_| {
                vm.push(Var.nil_val);
            }
        } else if (arg_count < self.arg_count) {
            vm.sp -= self.arg_count-arg_count;
        }

        if (!self.is_callback) {
            vm.call_stack.append(.{
                .ip = vm.program.ip,
                .bp = vm.bp,
                .upval_ctx = vm.upval_ctx
            }) catch unreachable;

            vm.program.ip = self.ptr;
            vm.bp = vm.sp-self.arg_count-1;
            vm.upval_ctx = self.upvals;
        } else {
            const func: *const fn(*Vm,[]Var) ReturnCode!Var = @ptrCast(self.ptr);
            vm.bp[0] = try func(vm, (vm.bp+1)[0..arg_count]);
            vm.program.ip -= arg_count;
        }
    }

    pub fn ret(vm: *Vm) !void {
        if (vm.call_stack.pop()) |e| {
            vm.sp = vm.bp+1;
            vm.bp = e.bp;
            vm.program.ip = e.ip;
            vm.upval_ctx = e.upval_ctx;
        } else {
            return error.halt;
        }
    }

    pub fn deinit(self:*Self) void {
        Vm.gpa.destroy(self);
    }

};

fn hello(_:*Vm, x:i32) !i32 {
    std.debug.print("hello\n", .{});
    return x+1;
}

test "Callback" {
    const f = Func.wrap(hello);
    var vm = Vm.init(undefined);
    defer f.deinit();
    defer vm.deinit();

    vm.push(Var.from(32));
    try f.call(1, &vm);
    try std.testing.expectEqual(33, vm.top().as(i32));
}