const std = @import("std");
const Var = @import("var.zig").Var;

pub const ReturnCode = error {
    halt,
    panic,
    todo,
};

const ErrType = enum {
    invalidIdx,
    opTypeErr,
    funcTypeErr
};

pub const Err = union(ErrType) {
    pub var global:Err = undefined;

    invalidIdx:Var,
    opTypeErr:struct {
        op:enum {
            add,sub,div,idiv,mul,pow,mod,concat
        },
        rhs:Var.Type,
        lhs:Var.Type,
    },

    funcTypeErr:struct {
        param:u8,
        rhs:Var.Type,
        lhs:Var.Type,
    },
};

