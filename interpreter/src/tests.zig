const std = @import("std");
const Vm = @import("vm.zig").Vm;
const Program = @import("bytecode.zig").Program;
const Str = @import("str.zig").Str;
const ByteCode = @import("bytecode.zig").ByteCode;
const Table = @import("table.zig").Table;
const Var = @import("var.zig").Var;

test "compat" {
    const p = try Program.init("tests/compat.lout");
    var vm = Vm.init(p);
    try vm.execUntilHaltDebug();
    try std.testing.expectApproxEqRel(20.1, vm.pop().as(f32), 0.01);
}

test "assing/declaration" {
    const p = try Program.init("tests/assing_dec.lout");
    var vm = Vm.init(p);
    defer vm.deinit();
    try vm.execUntilHaltDebug();
    try std.testing.expectEqualDeep("helloworld10.1", vm.pop().as(Str).asSlice());
    try std.testing.expectEqual(10, vm.pop().as(i32));
    try std.testing.expectEqual(11.0*12.1, vm.pop().as(f32));
}


test "func comp" {
    const p = try Program.init("tests/func_comp.lout");
    var vm = Vm.init(p);
    defer vm.deinit();
    try vm.execUntilHaltDebug();
    try std.testing.expectEqual(22,vm.pop().as(i32));
}

test "inline func" {
    const p = try Program.init("tests/inline_func.lout");
    var vm = Vm.init(p);
    defer vm.deinit();
    try vm.execUntilHaltDebug();
    try std.testing.expectEqual(23,vm.pop().as(i32));
    try std.testing.expectEqual(13,vm.pop().as(i32));
}

test "while" {
    const p = try Program.init("tests/while.lout");
    var vm = Vm.init(p);
    defer vm.deinit();
    try vm.execUntilHaltDebug();
    try std.testing.expectEqual(10,vm.pop().as(i32));
}

test "tables get set" {
    const p = try Program.init("tests/table.lout");
    var vm = Vm.init(p);
    defer vm.deinit();
    try vm.execUntilHaltDebug();

    try std.testing.expectEqual(.table, vm.top().tag());
    const y = vm.pop().as(*Table);

    try std.testing.expectEqual(.table, vm.top().tag());
    const x =  vm.pop().as(*Table);

    try std.testing.expectEqual(x, (try y.get(Var.from(2))).?.as(*Table));
    try std.testing.expectApproxEqRel(3.14, (try y.get(Var.from(1))).?.as(f32), 0.01);
    try std.testing.expectEqual(3, (try y.get(Var.from(0))).?.as(i32));

    try std.testing.expectEqual(10, (try y.get(Var.from(Str.init("x")))).?.as(i32));
}

//test "layout" {
//    var int:u32 = 0;
//    const x:*ByteCode = @ptrCast(&int);
//    x.* = ByteCode{.closure = .{.arg_count = 1,.upval_cap = 2}};
//    std.debug.print("{x}\n", .{int});
//    x.* = ByteCode{.write = 0x00cc};
//    std.debug.print("{x}\n", .{int});
//    x.* = ByteCode{.load = 0xffaa};
//    std.debug.print("{x}\n", .{int});
//}
