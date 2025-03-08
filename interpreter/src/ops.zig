const std = @import("std");
const Var = @import("var.zig").Var;
const Err = @import("err.zig").RunErr;

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
                .op = .pow,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}