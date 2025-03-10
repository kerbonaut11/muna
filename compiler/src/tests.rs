use crate::{asm::Assembler, ast_gen, bytecode::ByteCode, compiler::FuncCtx, expr::Expr, tokenizer};

#[test]
pub fn compatibility_test_file() {
    let mut asm = Assembler::new();
    asm.encode_instr(ByteCode::LoadTrue);
    asm.encode_instr(ByteCode::LoadInt);
    asm.encode_int(20);
    asm.encode_instr(ByteCode::LoadFalse);
    asm.encode_instr(ByteCode::LoadNil);
    asm.encode_instr(ByteCode::LoadFloat);
    asm.encode_float(0.1);
    asm.encode_instr(ByteCode::Load(1));
    asm.encode_instr(ByteCode::Add);
    asm.encode_instr(ByteCode::Halt);
    asm.write_to_file("../tests/compat.out");
}

#[test]
pub fn expr_test_file() {
    let mut asm = Assembler::new();
    let expr = Expr::parse(&tokenizer::parse("(1+32.0)/3 ").unwrap()).unwrap();
    expr.compile(&mut FuncCtx::new(), &mut asm);
    asm.print();
    asm.encode_instr(ByteCode::Halt);
    asm.write_to_file("../tests/expr.out");
}

#[test]
pub fn assing_declaration_test_file() {
    let mut asm = Assembler::new();
    let tokens = &tokenizer::parse("local x,y,z = 10,12.1,\"hello\" x,y,z = (x+1)*y,x,z..\"world\"..10.1 ").unwrap();
    let block = ast_gen::parse_block(&tokens).unwrap();
    FuncCtx::new().compile(&mut asm, &block);
    asm.print();
    asm.encode_instr(ByteCode::Halt);
    asm.write_to_file("../tests/assing_dec.out");
}
