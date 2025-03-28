const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const Err = @import("err.zig").Err;
const Str = @import("str.zig").Str;

fn tycomb(a:Var.Type,b:Var.Type) u8 {
    return (@as(u8,@intFromEnum(a)) << 4) | @as(u8,@intFromEnum(b));
}

pub fn add(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(lhs.as(i32)      + rhs.as(i32)),
        tycomb(.int, .float)   => Var.from(lhs.intToFloat() + rhs.as(f32)),
        tycomb(.float, .int)   => Var.from(lhs.as(f32)      + rhs.intToFloat()),
        tycomb(.float, .float) => Var.from(lhs.as(f32)      + rhs.as(f32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .add,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn sub(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(lhs.as(i32)      - rhs.as(i32)),
        tycomb(.int, .float)   => Var.from(lhs.intToFloat() - rhs.as(f32)),
        tycomb(.float, .int)   => Var.from(lhs.as(f32)      - rhs.intToFloat()),
        tycomb(.float, .float) => Var.from(lhs.as(f32)      - rhs.as(f32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .sub,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn mul(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(lhs.as(i32)      * rhs.as(i32)),
        tycomb(.int, .float)   => Var.from(lhs.intToFloat() * rhs.as(f32)),
        tycomb(.float, .int)   => Var.from(lhs.as(f32)      * rhs.intToFloat()),
        tycomb(.float, .float) => Var.from(lhs.as(f32)      * rhs.as(f32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .mul,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn div(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(lhs.intToFloat() / rhs.intToFloat()),
        tycomb(.int, .float)   => Var.from(lhs.intToFloat() / rhs.as(f32)),
        tycomb(.float, .int)   => Var.from(lhs.as(f32)      / rhs.intToFloat()),
        tycomb(.float, .float) => Var.from(lhs.as(f32)      / rhs.as(f32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .div,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn pow(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(std.math.pow(i32,lhs.as(i32),     rhs.as(i32),)),
        tycomb(.int, .float)   => Var.from(std.math.pow(f32,lhs.intToFloat(),rhs.as(f32))),
        tycomb(.float, .int)   => Var.from(std.math.pow(f32,lhs.as(f32),     rhs.intToFloat())),
        tycomb(.float, .float) => Var.from(std.math.pow(f32,lhs.as(f32),     rhs.as(f32))),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .pow,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn mod(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => Var.from(@mod(lhs.as(i32),      rhs.as(i32))),
        tycomb(.int, .float)   => Var.from(@mod(lhs.intToFloat(), rhs.as(f32))),
        tycomb(.float, .int)   => Var.from(@mod(lhs.as(f32),      rhs.intToFloat())),
        tycomb(.float, .float) => Var.from(@mod(lhs.as(f32),      rhs.as(f32))),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .mod,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn concat(lhs:Var,rhs:Var) !Str {
    return Str.initConcat((try toStr(lhs)).asSlice(), (try toStr(rhs)).asSlice());
}

pub fn toStr(x:Var) !Str {
    return switch (x.tag()) {
        .nil => Vm.nil_str,
        .bool => if (x.as(bool)) Vm.true_str else Vm.false_str,
        .int => {
            var buf = [1]u8{0} ** 32;
            return Str.init(std.fmt.bufPrint(&buf, "{d}", .{x.as(i32)}) catch unreachable);
        },
        .float => {
            var buf = [1]u8{0} ** 32;
            return Str.init(std.fmt.bufPrint(&buf, "{d}", .{x.as(f32)}) catch unreachable);
        },
        .str => x.as(Str),

        else => unreachable
    };
}

pub fn eq(lhs:Var,rhs:Var) !bool {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.nil, .nil)     => true,
        tycomb(.bool, .bool)   => lhs.as(bool) == rhs.as(bool),
        tycomb(.str, .str)     => std.mem.eql(u8,lhs.as(Str).asSlice(),rhs.as(Str).asSlice()),

        tycomb(.int, .int)     => lhs.as(i32)      < rhs.as(i32),
        tycomb(.int, .float)   => lhs.intToFloat() < rhs.as(f32),
        tycomb(.float, .int)   => lhs.as(f32)      < rhs.intToFloat(),
        tycomb(.float, .float) => lhs.as(f32)      < rhs.as(f32),
        else => false
    };
}


pub fn less(lhs:Var,rhs:Var) !bool {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => lhs.as(i32)      < rhs.as(i32),
        tycomb(.int, .float)   => lhs.intToFloat() < rhs.as(f32),
        tycomb(.float, .int)   => lhs.as(f32)      < rhs.intToFloat(),
        tycomb(.float, .float) => lhs.as(f32)      < rhs.as(f32),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .compare,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn less_eq(lhs:Var,rhs:Var) !bool {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int)     => lhs.as(i32)      <= rhs.as(i32),
        tycomb(.int, .float)   => lhs.intToFloat() <= rhs.as(f32),
        tycomb(.float, .int)   => lhs.as(f32)      <= rhs.intToFloat(),
        tycomb(.float, .float) => lhs.as(f32)      <= rhs.as(f32),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .compare,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}


pub fn truthy(x:Var) !bool {
    return switch (x.tag()) {
        .nil  => false,
        .bool => x.as(bool),
        else => true
    };
}