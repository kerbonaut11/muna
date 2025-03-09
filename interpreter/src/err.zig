const std = @import("std");
const Var = @import("var.zig").Var;

pub const ReturnCode = error {
    halt,
    panic,
    todo,
};

const RunErrType = enum {
    invalidIdx,
    opTypeErr,
    funcTypeErr
};

pub const RunErr = union(RunErrType) {
    pub var global:RunErr = undefined;

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

