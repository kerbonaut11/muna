const std = @import("std");
const Var = @import("var.zig").Var;
const Vm = @import("vm.zig").Vm;
const Err = @import("err.zig").Err;
const Str = @import("str.zig").Str;
const Table = @import("table.zig").Table;

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

pub fn idiv(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(@divFloor(lhs.as(i32), rhs.as(i32))),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .idiv,
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

pub fn concat(lhs:Var,rhs:Var) !Var {
    const str = Str.initConcat((try toStr(lhs)).asSlice(), (try toStr(rhs)).asSlice());
    return Var.from(str);
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


pub fn binAnd(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(lhs.as(i32) & rhs.as(i32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .bin_and,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn binOr(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(lhs.as(i32) | rhs.as(i32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .bin_or,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn binXor(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(lhs.as(i32) ^ rhs.as(i32)),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .bin_xor,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn shl(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(lhs.as(i32) << @as(u5,@intCast(rhs.as(i32)))),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .shl,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}


pub fn shr(lhs:Var,rhs:Var) !Var {
    return switch (tycomb(lhs.tag(), rhs.tag())) {
        tycomb(.int, .int) => Var.from(lhs.as(i32) >> @as(u5,@intCast(rhs.as(i32)))),
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .shr,
                .lhs = lhs.tag(),
                .rhs = rhs.tag()
            }};
            return error.panic;
        },
    };
}

pub fn boolAnd(lhs:Var,rhs:Var) !bool {
    return try truthy(lhs) and try truthy(rhs);
}

pub fn boolOr(lhs:Var,rhs:Var) !bool {
    return try truthy(lhs) or try truthy(rhs);
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

pub fn lessEq(lhs:Var,rhs:Var) !bool {
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


pub fn get(lhs:Var,rhs:Var) !Var {
    return switch (lhs.tag()) {
        .table => {
            if (try lhs.as(*Table).get(rhs)) |x| {
                return x;
            } else {
                return Var.nil_val;
            }
        },
        else => {
            Err.global = Err{.opTypeErr = .{
                .op = .idx,
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


pub fn neg(x:Var) !Var {
    return switch (x.tag()) {
        .int => Var.from(-x.as(i32)),
        .float => Var.from(-x.as(f32)),
        else => {
            Err.global = Err{.unaryTypeErr = .{
                .op = .neg,
                .ty = x.tag()
            }};
            return error.panic;
        }
    };
}

pub fn binNot(x:Var) !Var {
    return switch (x.tag()) {
        .int => Var.from(~x.as(i32)),
        else => {
            Err.global = Err{.unaryTypeErr = .{
                .op = .bin_not,
                .ty = x.tag()
            }};
            return error.panic;
        }
    };
}

pub fn boolNot(x:Var) !bool {
    return !try truthy(x);
}


pub fn len(x:Var) !Var {
    return switch (x.tag()) {
        .str   => Var.from(x.as(Str).getLen()),
        .table => Var.from(@as(i32,@intCast(x.as(*Table).arrSlice().len))),
        else => {
            Err.global = Err{.unaryTypeErr = .{
                .op = .len,
                .ty = x.tag()
            }};
            return error.panic;
        }
    };
}