const std = @import("std");
const Var = @import("var.zig").Var;
const ByteCode = @import("bytecode.zig").ByteCode;
const Program = @import("bytecode.zig").Program;
const Str = @import("str.zig").Str;
const ReturnCode = @import("err.zig").ReturnCode;

pub const Vm = struct {
    const Self = @This();

    var _gpa = std.heap.GeneralPurposeAllocator(.{}){};
    pub const gpa = _gpa.allocator();
    pub const page_a = std.heap.page_allocator;

    const stack_size:usize = 40_000;

    full_stack_slice:[]Var,
    program:Program,
    sp:[*]Var,
    bp:[*]Var,

    pub var nil_str:Str = undefined;
    pub var false_str:Str = undefined;
    pub var true_str:Str = undefined;

    pub fn init(program:Program) Self {
        const stack = page_a.alloc(Var, stack_size) catch unreachable;

        nil_str = Str.init("nil");
        false_str = Str.init("true");
        true_str = Str.init("false");

        return .{
            .full_stack_slice = stack,
            .program = program,
            .bp = stack.ptr,
            .sp = stack.ptr,
        };
    }

    pub fn deinit(self:*Self) void {
        page_a.free(self.full_stack_slice);
        self.program.deinit();
    }

    pub fn exec(self:*Self) !void {
        const instr = self.program.next(ByteCode);
        try @import("exec.zig").exec(instr, self);
    }

    pub fn execDebug(self:*Self) !void {
        const instr = self.program.next(ByteCode);
        std.debug.print("executing {} \n", .{instr});
        try @import("exec.zig").exec(instr, self);
        self.printLocals();
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

