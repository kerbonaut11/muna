const std = @import("std");
const Var = @import("var.zig").Var;
const ByteCode = @import("bytecode.zig").ByteCode;
const Program = @import("bytecode.zig").Program;

pub const Vm = struct {
    const Self = @This();

    var _gpa = std.heap.GeneralPurposeAllocator(.{}){};
    pub const gpa = _gpa.allocator();
    pub const page_a = std.heap.page_allocator;

    const STACKSIZE:usize = 40_000;

    full_stack_slice:[]Var,
    program:Program,
    sp:[*]Var,
    bp:[*]Var,

    pub fn init(program:Program) Self {
        const stack = page_a.alloc(Var, STACKSIZE) catch unreachable;
        return .{
            .full_stack_slice = stack,
            .program = program,
            .bp = stack.ptr,
            .sp = stack.ptr,
        };
    }

    pub fn deinit(self:*Self) void {
        page_a.free(self.full_stack_slice);
        self.program.deinit();
    }

    pub fn exec(self:*Self) !void {
        const instr = self.program.next(ByteCode);
        try @import("exec.zig").exec(instr, self);
    }


    pub fn push(self:*Self,x:Var) void {
        self.sp[0] = x;
        self.sp += 1;
    }

    pub fn pop(self:*Self) Var {
        self.sp -= 1;
        return self.sp[0];
    }

    pub fn top(self:*Self) *Var {
        return @ptrCast(self.sp - 1);
    }

    pub fn binaryOp(self:*Self,comptime op:fn(Var,Var) @import("err.zig").ErrorEnum!Var) !void {
        const lhs = self.pop();
        const rhs = self.top().*;
        self.top().* = try op(lhs,rhs);
    }
};


test "Vm" {
    var program = Program.init();
    program.encode(ByteCode.load_int);
    program.encode(20);
    program.encode(ByteCode.load_int);
    program.encode(10);
    program.encode(ByteCode.load_int);
    program.encode(10);
    program.encode(ByteCode{.load = 0});
    program.encode(ByteCode.add);
    program.encode(ByteCode{.write = 1});

    var vm = Vm.init(program);
    defer vm.deinit();

    vm.program.start();
    vm.push(Var.from(10));
    try std.testing.expectEqual(vm.pop(), Var.from(10));
    try std.testing.expectEqual(vm.sp, vm.bp);
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try vm.exec();
    try std.testing.expectEqual(vm.top().*, Var.from(30));
    try vm.exec();
    try std.testing.expectEqual(vm.bp[1], Var.from(30));
}
