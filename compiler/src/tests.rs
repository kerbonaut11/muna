use crate::{asm::{ByteCodeVec, CompileCtx}, ast_gen, bytecode::ByteCode, compiler::FuncCtx, expr::Expr, tokenizer};

#[test]
pub fn compatibility_test_file() {
    let comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();
    bytecode.encode_instr(ByteCode::LoadTrue);
    bytecode.encode_instr(ByteCode::LoadInt);
    bytecode.encode_int(20);
    bytecode.encode_instr(ByteCode::LoadFalse);
    bytecode.encode_instr(ByteCode::LoadNil);
    bytecode.encode_instr(ByteCode::LoadFloat);
    bytecode.encode_float(0.1);
    bytecode.encode_instr(ByteCode::Load(2));
    bytecode.encode_instr(ByteCode::Add);
    bytecode.encode_instr(ByteCode::Halt);
    comp_ctx.write_to_file(&bytecode,"../tests/compat.out");
}

#[test]
pub fn expr_test_file() {
    let mut comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();

    let expr = Expr::parse(&tokenizer::parse("(1+32.0)/3 ").unwrap()).unwrap();
    expr.compile(&mut FuncCtx::new(&[]), &mut comp_ctx, &mut bytecode);
    bytecode.encode_instr(ByteCode::Halt);
    comp_ctx.write_to_file(&bytecode, "../tests/expr.out");
}

#[test]
pub fn assing_declaration_test_file() {
    let mut comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();


    let tokens = &tokenizer::parse("local x,y,z = 10,12.1,\"hello\" x,y,z = (x+1)*y,x,z..\"world\"..10.1 ").unwrap();
    let block = ast_gen::parse_block(&tokens).unwrap();
     FuncCtx::new(&[]).compile(&block,&mut comp_ctx, &mut bytecode);
    bytecode.encode_instr(ByteCode::Halt);
    comp_ctx.write_to_file(&bytecode,"../tests/assing_dec.out");
}

#[test]
pub fn call_ret_test_file() {
    let comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();

    bytecode.encode_instr(ByteCode::LoadNil);
    bytecode.encode_instr(ByteCode::LoadInt);
    bytecode.encode_int(32);
    bytecode.encode_instr(ByteCode::Closure{upval_cap:0,arg_count:1});
    bytecode.encode_int(2);
    bytecode.encode_instr(ByteCode::Call);
    bytecode.encode_instr(ByteCode::Halt);
    bytecode.encode_instr(ByteCode::Load(1));
    bytecode.encode_instr(ByteCode::Mul);
    bytecode.encode_instr(ByteCode::Write(0));
    bytecode.encode_instr(ByteCode::Ret);

    comp_ctx.write_to_file(&bytecode,"../tests/call_ret.out");
}


#[test]
pub fn func_comp_test_file() {
    let mut comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();


    let tokens = &tokenizer::parse("
        function a(x) {
            return b(x+1)
        }

        function b(x) {
            return x*2
        }

        local x = a(10) 
    ").unwrap();
    let block = ast_gen::parse_block(&tokens).unwrap();
    FuncCtx::new(&[]).compile(&block,&mut comp_ctx, &mut bytecode);
    comp_ctx.write_to_file(&bytecode,"../tests/func_comp.out");
}


#[test]
pub fn inline_func_test_file() {
    let mut comp_ctx = CompileCtx::new();
    let mut bytecode = ByteCodeVec::new();


    let tokens = &tokenizer::parse("
        function new_counter(x) {
            local step = 10
            return function() {
                x = x+step
                return x
            }
        }

        local counter = new_counter(3)
        local x = counter()
        local y = counter()
    ").unwrap();
    let block = ast_gen::parse_block(&tokens).unwrap();
    FuncCtx::new(&[]).compile(&block,&mut comp_ctx, &mut bytecode);
    comp_ctx.write_to_file(&bytecode,"../tests/inline_func.out");
}