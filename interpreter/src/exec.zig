const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const ByteCode = @import("bytecode.zig").ByteCode;
const Str = @import("str.zig").Str;
const ops = @import("ops.zig");
const Err = @import("err.zig").Err;
const Func = @import("func.zig").Func;
const Table = @import("table.zig").Table;


pub fn exec(instr:ByteCode,vm: *Vm) !void {
    switch (instr) {
        .load_nil   => vm.push(Var.nil_val),
        .load_true  => vm.push(Var.true_val),
        .load_false => vm.push(Var.false_val),
        .load_int   => vm.push(Var.from(vm.program.next(i32))),
        .load_float => vm.push(Var.from(vm.program.next(f32))),
        .load_str => |i| vm.push(vm.program.name_table[i]),

        .load  => |i| vm.push(vm.bp[i]),
        .write => |i| vm.bp[i] = vm.pop(),
        .pop => _ = vm.pop(),

        .add    => try vm.binaryOp(ops.add),
        .sub    => try vm.binaryOp(ops.sub),
        .mul    => try vm.binaryOp(ops.mul),
        .div    => try vm.binaryOp(ops.div),
        .idiv   => try vm.binaryOp(ops.idiv),
        .pow    => try vm.binaryOp(ops.pow),
        .mod    => try vm.binaryOp(ops.mod),
        .concat => try vm.binaryOp(ops.concat),

        .bin_and => try vm.binaryOp(ops.binAnd),
        .bin_or  => try vm.binaryOp(ops.binOr),
        .bin_xor => try vm.binaryOp(ops.binXor),
        .shl     => try vm.binaryOp(ops.shl),
        .shr     => try vm.binaryOp(ops.shr),

        .bool_and => try vm.compOp(ops.boolAnd, true),
        .bool_or  => try vm.compOp(ops.boolOr, true),

        .eq      => |expected| try vm.compOp(ops.eq, expected),
        .less    => |expected| try vm.compOp(ops.less, expected),
        .less_eq => |expected| try vm.compOp(ops.lessEq, expected),

        .neg      => try vm.unaryOp(ops.neg),
        .bin_not  => try vm.unaryOp(ops.binNot),
        .bool_not => vm.top().* = Var.from(try ops.boolNot(vm.top().*)),
        .len      => try vm.unaryOp(ops.len),

        .new_table => |cap| vm.push(Var.from(Table.init(cap))),

        .get => try vm.binaryOp(ops.get),
        .get_method => |i| {
            const k = vm.program.name_table[i];
            if (vm.top().tag() != .table) {
                Err.global = Err{.unaryTypeErr = .{
                    .op = .method,
                    .ty = vm.top().tag(),
                }};
                return error.panic;
            }

            if (vm.top().as(*Table).getMetaTable()) |mt| {
                if (mt.getNoValidate(k)) |m| {
                    vm.push(m);
                    return;
                }
            }

            Err.global = Err{.methodNotFound = k.as(Str).asSlice()};
            return error.panic;
        },

        .set => {
            const v = vm.pop();
            const k = vm.pop();
            const t = vm.top().*;
            switch (t.tag()) {
                .table => try t.as(*Table).set(k,v),
                else => {
                    Err.global = Err{.opTypeErr = .{
                        .op = .idx,
                        .lhs = t.tag(),
                        .rhs = .nil
                    }};
                    return error.panic;
                },
            }
        },
        .set_pop => {
            const v = vm.pop();
            const k = vm.pop();
            const t = vm.pop();
            switch (t.tag()) {
                .table => try t.as(*Table).set(k,v),
                else => {
                    Err.global = Err{.opTypeErr = .{
                        .op = .idx,
                        .lhs = t.tag(),
                        .rhs = .nil
                    }};
                    return error.panic;
                },
            }
        },

        .push => {
            const x = vm.pop();
            vm.top().as(*Table).pushUnsafe(x);
        },

        .closure => |arg| {
            const offset = vm.program.next(u32);
            std.debug.print("{}\n", .{offset});
            const ptr = vm.program.ip + offset;
            const func = Func.init(ptr,arg.arg_count,arg.upval_cap);
            vm.push(Var.from(func));
        },

        .call => |arg_count| {
            const x = vm.pop();
            switch (x.tag()) {
                .func => try x.as(*Func).call(@intCast(arg_count),vm),
                else => {
                    Err.global = .{.unaryTypeErr = .{
                        .op = .call,
                        .ty = x.tag(),
                    }};

                    return error.panic;
                }
            } 
        },

        .ret => try Func.ret(vm),

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

        //else => return error.todo,
    }
}

