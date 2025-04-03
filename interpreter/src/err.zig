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
            bin_and,bin_or,bin_xor,shl,shr,
            compare,idx
        },
        rhs:Var.Type,
        lhs:Var.Type,
    },

    unaryTypeErr:struct {
        op:enum {
            call,neg,len,bin_not,bool_not
        },
        ty:Var.Type
    },

    funcTypeErr:struct {
        param:u8,
        rhs:Var.Type,
        lhs:Var.Type,
    },
};

