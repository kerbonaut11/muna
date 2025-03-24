const std = @import("std");
const Var = @import("var.zig").Var;
const ByteCode = @import("bytecode.zig").ByteCode;
const Program = @import("bytecode.zig").Program;
const Str = @import("str.zig").Str;
const Func = @import("func.zig").Func;
const ReturnCode = @import("err.zig").ReturnCode;

pub const Vm = struct {
    const Self = @This();

    var _gpa = std.heap.GeneralPurposeAllocator(.{}){};
    pub const gpa = _gpa.allocator();
    pub const page_a = std.heap.page_allocator;

    const stack_size:usize = 40_000;

    full_stack_slice:[]Var,
    sp:[*]Var,
    bp:[*]Var,
    upval_ctx:[*]Var,

    program:Program,

    call_stack:std.ArrayList(CallStackEntry),

    pub const CallStackEntry = struct {
        bp:[*]Var,
        ip:[*]u32,
        upval_ctx:[*]Var,
    };

    pub var nil_str:Str = undefined;
    pub var false_str:Str = undefined;
    pub var true_str:Str = undefined;

    pub fn init(program:Program) Self {
        const stack = page_a.alloc(Var, stack_size) catch unreachable;

        nil_str = Str.init("nil");
        false_str = Str.init("true");
        true_str = Str.init("false");

        var self = Vm{
            .full_stack_slice = stack,
            .program = program,
            .bp = stack.ptr,
            .sp = stack.ptr,
            .upval_ctx = undefined,
            .call_stack = std.ArrayList(CallStackEntry).init(Vm.gpa)
        };

        self.push(Var.NIL);
        return self;
    }

    pub fn deinit(self:*Self) void {
        page_a.free(self.full_stack_slice);
        self.call_stack.deinit();
        self.program.deinit();
    }

    pub fn exec(self:*Self) !void {
        const instr = self.program.next(ByteCode);
        try @import("exec.zig").exec(instr, self);
    }

    pub fn execDebug(self:*Self) !void {
        self.printLocals();
        const instr = self.program.next(ByteCode);
        std.debug.print("executing:{} {} \n", .{self.program.ip-self.program.list.items.ptr,instr});
        try @import("exec.zig").exec(instr, self);
        std.debug.print("\n", .{});
    }

    pub fn execUntilHalt(self:*Self) ReturnCode!void {
        while (true) {
            self.exec() catch |err| {
                switch (err) {
                    ReturnCode.halt => return,
                    else => return err,
                }
            };
        }
    }

    pub fn execUntilHaltDebug(self:*Self) ReturnCode!void {
        while (true) {
            self.execDebug() catch |err| {
                switch (err) {
                    ReturnCode.halt => return,
                    else => return err,
                }
            };
        }
    }

    pub fn push(self:*Self,x:Var) void {
        self.sp[0] = x;
        self.sp += 1;
    }

    pub fn pop(self:*Self) Var {
        self.sp -= 1;
        return self.sp[0];
    }

    pub fn top(self:*Self) *Var {
        return @ptrCast(self.sp - 1);
    }

    pub fn localSlice(self:*Self) []Var {
        const size = self.sp-self.bp;
        return self.bp[0..size];
    }

    pub fn printLocals(self:*Self) void {
        for (self.localSlice()) |local| {
            switch (local.tag()) {
                .nil => std.debug.print("nil\n", .{}),
                .bool => std.debug.print("bool:{}\n", .{local.as(bool)}),
                .int => std.debug.print("int:{d}\n", .{local.as(i32)}),
                .float => std.debug.print("float:{d}\n", .{local.as(f32)}),
                .str => std.debug.print("str:{s}\n", .{local.as(Str).asSlice()}),
                .func => std.debug.print("func@{x}\n", .{@intFromPtr(local.as(*Func).ptr)}),
                else => {}
            }
        }
    }

    pub fn binaryOp(self:*Self,comptime op:fn(Var,Var) ReturnCode!Var) !void {
        const rhs = self.pop();
        const lhs = self.top().*;
        self.top().* = try op(lhs,rhs);
    }
};

