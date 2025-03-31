use crate::{asm::{ByteCodeVec, CompileCtx}, ast_gen, bytecode::ByteCode, compiler::FuncCtx, expr::Expr, tokenizer};

fn compile_to_file(src:&str,path:&str) {
    let mut comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();
    let tokens = &tokenizer::parse(src).unwrap();
    let block = ast_gen::parse_block(&tokens).unwrap();
    FuncCtx::new(&[]).compile(&block ,&mut comp_ctx, &mut bytecode, Some(ByteCode::Halt), None);
    bytecode.print();
    comp_ctx.write_to_file(bytecode,path);
}


#[test]
pub fn compatibility_test_file() {
    let comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();
    bytecode.add_instr(ByteCode::LoadTrue);
    bytecode.add_instr(ByteCode::LoadInt(20));
    bytecode.add_instr(ByteCode::LoadFalse);
    bytecode.add_instr(ByteCode::LoadNil);
    bytecode.add_instr(ByteCode::LoadFloat(0.1));
    bytecode.add_instr(ByteCode::Load(2));
    bytecode.add_instr(ByteCode::Add);
    bytecode.add_instr(ByteCode::Halt);
    comp_ctx.write_to_file(bytecode,"../tests/compat.lout");
}


#[test]
pub fn assing_declaration_test_file() {
    compile_to_file("
        local x,y,z = 10,12.1,\"hello\";
        x,y,z = (x+1)*y,x,z..\"world\"..10.1;
        ","../tests/assing_dec.lout"
    );
}


#[test]
pub fn func_comp_test_file() {
    compile_to_file("
        function a(x) {
            return b(x+1);
        }

        function b(x) {
            return x*2;
        }

        local x = a(10); 
    ","../tests/func_comp.lout");
}


#[test]
pub fn inline_func_test_file() {
    compile_to_file("
        function new_counter(x) {
            local step = 10;
            return function() {
                x = x+step;
                return x;
            };
        }

        local counter = new_counter(3);
        local x = counter();
        local y = counter();
    ","../tests/inline_func.lout");
}

#[test]
pub fn while_test_file() {
    compile_to_file("
        local i = 0;
        while 10 > i {
            i = i+1;
        }
    ","../tests/while.lout");
}

#[test]
pub fn if_else_test_file() {
    compile_to_file("
        local x = true;
        local y = 1;
        if x {
            y = 2;
        } else {
            y = 3; 
        }
    ","../tests/while.lout");
}


