const std = @import("std");
const Vm = @import("vm.zig").Vm;
const Program = @import("bytecode.zig").Program;

test "compat" {
    const p = try Program.init("tests/compat.out");
    var vm = Vm.init(p);
    try vm.execUntilHaltDebug();
    try std.testing.expectApproxEqRel(20.1, vm.pop().as(f32), 0.01);
}


test "expr" {
    const p = try Program.init("tests/expr.out");
    var vm = Vm.init(p);
    try vm.execUntilHaltDebug();
    try std.testing.expectEqual((1.0+32.0)/3.0, vm.pop().as(f32));
}

test "assing/declaration" {
    const p = try Program.init("tests/assing_dec.out");
    var vm = Vm.init(p);
    try vm.execUntilHaltDebug();
    try std.testing.expectEqual(10, vm.pop().as(i32));
    try std.testing.expectEqual(11.0*12.1, vm.pop().as(f32));
}

