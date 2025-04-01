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

        .add    => try vm.binaryOp(ops.add),
        .sub    => try vm.binaryOp(ops.sub),
        .mul    => try vm.binaryOp(ops.mul),
        .div    => try vm.binaryOp(ops.div),
        .idiv   => try vm.binaryOp(ops.idiv),
        .pow    => try vm.binaryOp(ops.pow),
        .mod    => try vm.binaryOp(ops.mod),
        .concat => try vm.binaryOp(ops.concat),

        .bin_and => try vm.binaryOp(ops.bin_and),
        .bin_or  => try vm.binaryOp(ops.bin_or),
        .bin_xor => try vm.binaryOp(ops.bin_xor),
        .shl     => try vm.binaryOp(ops.shl),
        .shr     => try vm.binaryOp(ops.shr),

        .bool_and => try vm.compOp(ops.bool_and, true),
        .bool_or  => try vm.compOp(ops.bool_and, false),

        .eq      => |expected| try vm.compOp(ops.eq, expected),
        .less    => |expected| try vm.compOp(ops.less, expected),
        .less_eq => |expected| try vm.compOp(ops.less_eq, expected),

        .closure => |arg| {
            const offset = vm.program.next(u32);
            std.debug.print("{}\n", .{offset});
            const ptr = vm.program.ip + offset;
            const func = Func.init(ptr,arg.arg_count,arg.upval_cap);
            vm.push(Var.from(func));
        },

        .call => {
            const x = vm.pop();
            switch (x.tag()) {
                .func => {
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

                else => {
                    Err.global = .{.unaryTypeErr = .{
                        .op = .call,
                        .ty = x.tag(),
                    }};

                    return error.panic;
                }
            } 
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

        .jump => |offset| vm.program.ip += @bitCast(@as(i64,offset)),
        .jump_true  => |offset| if ( try ops.truthy(vm.pop())) {vm.program.ip += @bitCast(@as(i64,offset));},
        .jump_false => |offset| if (!try ops.truthy(vm.pop())) {vm.program.ip += @bitCast(@as(i64,offset));},

        .halt => return error.halt,

        else => return error.todo,
    }
}

