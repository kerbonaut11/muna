const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const ByteCode = @import("bytecode.zig").ByteCode;
const Str = @import("str.zig").Str;
const ops = @import("ops.zig");
const Err = @import("err.zig").Err;
const Func = @import("func.zig").Func;


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
        .concat =>  {
            const rhs = vm.pop();
            const lhs = vm.top().*;
            vm.top().* = Var.from(try ops.concat(lhs,rhs));
        },

        .closure => |arg| {
            const offset = vm.program.next(u32);
            const ptr = vm.program.ip + offset;
            const func = Func.init(ptr,arg.arg_count,arg.upval_cap);
            vm.push(Var.from(func));
        },

        .call => {
            const x = vm.pop(); 
            if (x.tag() != .func) {
                Err.global = .{.unaryTypeErr = .{
                    .op = .call,
                    .ty = x.tag(),
                }};

                return error.panic;
            }

            const func = x.as(*Func);

            vm.call_stack.append(.{
                .ip = vm.program.ip,
                .bp = vm.bp,
                .upval_ctx = vm.upval_ctx
            }) catch unreachable;

            vm.program.ip = func.ptr;
            vm.bp = vm.sp-func.arg_count-1;
            vm.upval_ctx = func.upvals;
        },

        .ret => {
            if (vm.call_stack.pop()) |e| {
                vm.sp = vm.bp+1;
                vm.bp = e.bp;
                vm.program.ip = e.ip;
                vm.upval_ctx = e.upval_ctx;
            } else {
                return error.halt;
            }
        },

        .bind_upval => |i| {
            const x = vm.pop();
            vm.top().as(*Func).upvals[i] = x;
        },

        .get_upval => |i| vm.push(vm.upval_ctx[i]),
        .set_upval => |i| vm.upval_ctx[i] = vm.pop(),

        .halt => return error.halt,

        else => return error.todo,
    }
}

