const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const ByteCode = @import("bytecode.zig").ByteCode;
const Str = @import("str.zig").Str;
const ops = @import("ops.zig");

pub fn exec(instr:ByteCode,vm: *Vm) !void {
    switch (instr) {
        .load_nil   => vm.push(Var.NIL),
        .load_true  => vm.push(Var.TRUE),
        .load_false => vm.push(Var.FALSE),
        .load_int   => vm.push(Var.from(vm.program.next(i32))),
        .load_float => vm.push(Var.from(vm.program.next(f32))),
        .load_str => |i| vm.push(vm.program.name_table[i]),

        .load  => |i| vm.push(vm.bp[i]),
        .write => |i| vm.bp[i] = vm.pop(),

        .add => try vm.binaryOp(ops.add),
        .sub => try vm.binaryOp(ops.sub),
        .mul => try vm.binaryOp(ops.mul),
        .div => try vm.binaryOp(ops.div),
        .pow => try vm.binaryOp(ops.pow),
        .mod => try vm.binaryOp(ops.mod),

        .halt => return error.halt,

        else => return error.todo,
    }
}

