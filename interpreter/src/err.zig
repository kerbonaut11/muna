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
    unaryTypeErr,
    funcTypeErr
};

pub const Err = union(ErrType) {
    pub var global:Err = undefined;

    invalidIdx:Var,
    opTypeErr:struct {
        op:enum {
            add,sub,div,idiv,mul,pow,mod,concat,
            eq,compare
        },
        rhs:Var.Type,
        lhs:Var.Type,
    },

    unaryTypeErr:struct {
        op:enum {
            call,not,
        },
        ty:Var.Type
    },

    funcTypeErr:struct {
        param:u8,
        rhs:Var.Type,
        lhs:Var.Type,
    },
};

