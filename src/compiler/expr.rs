use std::{u32, usize};

use crate::{compiler::ast_gen, function::Function, ops::{Op, UnaryOp}};

use super::{ast_gen::Block, tokenizer::Token, err::Result};

pub enum Expr {
    NilLiteral,
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(Box<str>),
    TableLiteral(TableLiteral),
    Function(InlineFunction),

    Ident(Box<str>),

    Binary{op:Op,lhs:Box<Self>,rhs:Box<Self>},
    Unary{op:UnaryOp,val:Box<Self>},

    Index{table:Box<Self>,idx:Box<Self>},
    Call{function:Box<Self>,args:Vec<Self>},
    MethodCall{table:Box<Self>,name:Box<str>,args:Vec<Self>},
}

pub enum TableLiteralIdx {
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(Box<str>), 
}

pub type TableLiteral = Vec<(TableLiteralIdx,Expr)>;

pub struct InlineFunction {
    args:Vec<Box<str>>,
    block:Block
}


impl Expr {
    pub fn parse(tokens:&[Token]) -> Result<Expr> {
        parse_rec(tokens)
    }

    pub fn display_tree(&self,depth:u32) {
        for _ in 0..depth {
            print!("  ");
        }
        match self {
            Expr::Ident(name) => print!("var({})\n",name),

            Expr::NilLiteral               => print!("nil\n"),
            Expr::BoolLiteral(x)    => print!("bool({:?})\n",x),
            Expr::IntLiteral(x)      => print!("int({:?})\n",x),
            Expr::FloatLiteral(x)    => print!("float({:?})\n",x),
            Expr::StrLiteral(x) => print!("string({:?})\n",x),
            Expr::TableLiteral(t) => {
                print!("table:\n");
                for (k,v) in t {
                    for _ in 0..depth+1 {
                        print!("  ");
                    }
                    match k {
                        TableLiteralIdx::BoolLiteral(x)    => print!("bool({:?}) =\n",x),
                        TableLiteralIdx::IntLiteral(x)      => print!("int({:?}) =\n",x),
                        TableLiteralIdx::FloatLiteral(x)    => print!("float({:?}) =\n",x),
                        TableLiteralIdx::StrLiteral(x) => print!("str({:?}) =\n",x),
                    }

                    v.display_tree(depth+2);
                }
            }

            Expr::Function(f) => {
                print!("function(");
                for (i,arg) in f.args.iter().enumerate() {
                    print!("{}",arg);
                    if i != arg.len() {
                        print!(",");
                    }
                }
                print!(")\n");
            },

            Expr::Binary { op, lhs, rhs } => {
                print!("{:?}:\n",op);
                lhs.display_tree(depth+1);
                rhs.display_tree(depth+1);
            }
            Expr::Unary { op, val } => {
                print!("{:?}:\n",op);
                val.display_tree(depth+1);
            }

            Expr::Index { table, idx } => {
                print!("Index:\n");
                table.display_tree(depth+1);
                idx.display_tree(depth+1);
            }

            Expr::Call { function, args } => {
                print!("Call:\n");
                function.display_tree(depth+1);
                for arg in args {
                    arg.display_tree(depth+1);
                }
            }

            Expr::MethodCall { table, name, args } => {
                print!("Metod({}):\n",name);
                table.display_tree(depth+1);
                for arg in args {
                    arg.display_tree(depth+1);
                }
            }
        }
    }
}


fn parse_rec(tokens:&[Token]) -> Result<Expr> {
    println!("{:?}",tokens);
    if tokens.len() == 1 {
        return Ok(match &tokens[0] {
            Token::Nil => Expr::NilLiteral,
            Token::BoolLiteral(x)    => Expr::BoolLiteral(*x),
            Token::IntLiteral(x)      => Expr::IntLiteral(*x),
            Token::FloatLiteral(x)    => Expr::FloatLiteral(*x),
            Token::StrLiteral(x) => Expr::StrLiteral(x.clone()),
            Token::Ident(x)      => Expr::Ident(x.clone()),
            _ => panic!("invalid token {:?}",tokens[0])
        });
    }

    if let Some(i) = find_highest_order_op(tokens) {
        let op = tokens[i].op().unwrap();
        return Ok(Expr::Binary{ 
            op,
            lhs: Box::new(parse_rec(&tokens[0..i])?),
            rhs: Box::new(parse_rec(&tokens[(i+1)..])?),
        });
    }

    if let Some(op) = tokens[0].unary_op() {
        return Ok(Expr::Unary{
            op,
            val: Box::new(parse_rec(&tokens[1..])?)
        });
    }

    match tokens[0] {
        Token::Function => {
            let args_close_bracket = Token::find(tokens, &Token::RoundC).unwrap();
            let block_close_bracket = Token::find_matching_bracket(tokens,args_close_bracket+1);

            if block_close_bracket == Some(tokens.len()-2-args_close_bracket) {
                let mut arg_iter = tokens[2..args_close_bracket].into_iter();
                let mut args:Vec<Box<str>> = vec![];
                loop {
                    let arg_name = match arg_iter.next() {
                        Some(token) => match token {
                            Token::Ident(name) => name,
                            _ => panic!("invalid arg {:?}",token)
                        },

                        None => break
                    };
                    args.push(arg_name.clone());

                    match arg_iter.next() {
                        Some(token) => match token {
                            Token::Comma => {},
                            _ => panic!("expected comma, got {:?}",token)
                        },

                        None => break
                    }
                }

                let block = vec![];
                //let block = ast_gen::parse_block(tokens).unwrap();

                return Ok(Expr::Function(InlineFunction{
                    args,
                    block,
                }));
            }
        }
        _ => {}
    }

    match tokens.last().unwrap() {
        Token::SquareC => {
            let open_idx = Token::find_matching_bracket_rev(tokens).unwrap();
            return Ok(Expr::Index{
                table: Box::new(parse_rec(&tokens[0..open_idx])?),
                idx: Box::new(parse_rec(&tokens[(open_idx+1)..tokens.len()-1])?), 
            });
        }

        Token::RoundC => {
            let open_idx = Token::find_matching_bracket_rev(tokens).unwrap();
            if open_idx == 0 {
                return parse_rec(&tokens[1..tokens.len()-1]);
            } else {
                let args = parse_args(&tokens[(open_idx+1)..tokens.len()-1])?;
                if open_idx != 1 {
                    if tokens[open_idx-2] == Token::Colon {
                        let name = match &tokens[open_idx-1] {
                            Token::Ident(x) => x.clone(),
                            _ => panic!("{:?}",tokens[open_idx-1])
                        };
                        return Ok(Expr::MethodCall{
                            table: Box::new(parse_rec(&tokens[0..open_idx-2])?),
                            name,
                            args, 
                        });
                    }
                }

                return Ok(Expr::Call{
                    function: Box::new(parse_rec(&tokens[0..open_idx])?),
                    args,
                });
            }
        }

        Token::CurlyC => {
            assert_eq!(Token::find_matching_bracket_rev(tokens),Some(0));
            return Ok(Expr::TableLiteral(parse_table_literal(&tokens[1..tokens.len()-1])?));
        }

        Token::Ident(name) => {
            assert_eq!(tokens[tokens.len()-2],Token::Dot);
            return Ok(Expr::Index{
                table: Box::new(parse_rec(&tokens[0..(tokens.len()-2)])?),
                idx: Box::new(Expr::StrLiteral(name.clone())),
            });
        }
        _ => {}
    }
    panic!("{:?}",tokens);
}


fn parse_args(tokens:&[Token]) -> Result<Vec<Expr>> {
    if let Some(comma_idx) = Token::find_outside_of_brackets(tokens, &Token::Comma) {
        let arg = Expr::parse(&tokens[..comma_idx])?;
        let mut args = parse_args(&tokens[comma_idx+1..])?;
        args.insert(0, arg);
        Ok(args)
    } else {
        Ok(vec![Expr::parse(&tokens[..])?])
    }
}

fn parse_table_literal(tokens:&[Token]) -> Result<TableLiteral> {
    parse_table_literal_rec(tokens, &mut 0)
}

fn parse_table_literal_rec(tokens:&[Token],idx:&mut i64) -> Result<TableLiteral> {
    if tokens.is_empty() {
        return Ok(vec![]);
    }

    if let Some(comma_idx) = Token::find_outside_of_brackets(tokens, &Token::Comma) {
        let elem = parse_table_literal_element(&tokens[..comma_idx],idx)?;
        let mut elems = parse_table_literal_rec(&tokens[comma_idx+1..],idx)?;
        elems.insert(0, elem);
        Ok(elems)
    } else {
        Ok(vec![parse_table_literal_element(&tokens[..],idx)?])
    }
}

fn parse_table_literal_element(tokens:&[Token],idx:&mut i64) -> Result<(TableLiteralIdx,Expr)> {
    let has_equal = if tokens.len() > 2 {
        tokens[1] == Token::Assing
    } else {
        false
    };

    let index = if has_equal {
        match &tokens[0] {
            Token::BoolLiteral(x) => TableLiteralIdx::BoolLiteral(*x), 
            Token::IntLiteral(x)   => TableLiteralIdx::IntLiteral(*x), 
            Token::FloatLiteral(x) => TableLiteralIdx::FloatLiteral(*x), 
            Token::StrLiteral(x)|Token::Ident(x) => TableLiteralIdx::StrLiteral(x.clone()), 
            _ => panic!("invalid idx {:?}",tokens[0])
        }
    } else {
        let temp = *idx;
        *idx += 1;
        TableLiteralIdx::IntLiteral(temp)
    };

    let val = if has_equal {
        Expr::parse(&tokens[2..])
    } else {
        Expr::parse(&tokens[..])
    }?;

    Ok((index,val))
}

fn find_highest_order_op(tokens:&[Token]) -> Option<usize> {
    let mut depth = 0;
    let mut highest_prio = 0;
    let mut highest_idx = usize::MAX;

    for (i,token) in tokens.iter().enumerate() {
        depth += token.brack_depth();
        if depth == 0 {
            let prio = token.op_priority();
            if prio >= highest_prio {
                highest_prio = prio;
                highest_idx = i;
            }
        }
    }

    if highest_prio != 0 {
        Some(highest_idx)
    } else {
        None
    }
}


impl Token {
    pub fn is_end_of_expr(prev:&Token,next:&Token) -> bool {
        *next == Token::Comma || (next.is_valid_start_of_statement() && prev.is_valid_end_of_expr())
    }

    pub fn is_valid_start_of_statement(&self) -> bool {
        match self {
            Token::Function|Token::Ident(_)|Token::Local|Token::If|
            Token::While|Token::Break|Token::Return|Token::For => true,
            _ => false,
        }
    }

    pub fn is_valid_end_of_expr(&self) -> bool {
        match self {
            Token::RoundC|Token::CurlyC|Token::SquareC|
            Token::Ident(_)|Token::Nil|Token::BoolLiteral(_)|Token::IntLiteral(_)|Token::FloatLiteral(_)|Token::StrLiteral(_) => true,
            _ => false
        }
    }

    pub fn op_priority(&self) -> u32 {
        match self {
            Token::Add|Token::Sub => 5,
            Token::Mul|Token::Div|Token::IDiv|Token::Mod => 4,
            Token::Pow => 4,
            Token::And|Token::Or|Token::Xor => 3,
            Token::Less|Token::LessEq|Token::Eq|Token::NotEq|Token::Greater|Token::GreaterEq => 2,
            Token::BoolAnd|Token::BoolOr => 1,
            _ => 0, 
        }
    }

    pub fn op(&self) -> Option<Op> {
        match self {
            Token::Add       => Some(Op::Add),
            Token::Sub       => Some(Op::Sub),
            Token::Mul       => Some(Op::Mul),
            Token::Div       => Some(Op::Div),
            Token::IDiv      => Some(Op::IDiv),
            Token::Pow       => Some(Op::Pow),
            Token::Mod       => Some(Op::Mod),
            Token::And       => Some(Op::And),
            Token::Or        => Some(Op::Or),
            Token::Xor       => Some(Op::Xor),
            Token::Less      => Some(Op::Less),
            Token::LessEq    => Some(Op::LessEq),
            Token::Eq        => Some(Op::Eq),
            Token::NotEq     => Some(Op::NotEq),
            Token::Greater   => Some(Op::Greater),
            Token::GreaterEq => Some(Op::GreaterEq),
            Token::BoolAnd   => Some(Op::BoolAnd),
            Token::BoolOr    => Some(Op::BoolOr),
            _ => None,
        }
    }

    pub fn unary_op(&self) -> Option<UnaryOp> {
        match self {
            Token::Neg     => Some(UnaryOp::Neg),
            Token::Not     => Some(UnaryOp::Not),
            Token::BoolNot => Some(UnaryOp::BoolNot),
            Token::Len     => Some(UnaryOp::Len),
            _ => None
        }
    }
}

#[test]
fn test_lowest_order_op() {
    use super::tokenizer;
    let tokens = tokenizer::parse("map(function(x) x+1 end, x[i+2+3.31], {x=32^1.5})").unwrap();
    assert_eq!(find_highest_order_op(&tokens),None);

    let tokens = tokenizer::parse("(32 and 3)*2^2+a+a[a^2]").unwrap();
    assert_eq!(find_highest_order_op(&tokens),Some(11));
    assert_eq!(tokens[find_highest_order_op(&tokens).unwrap()],Token::Add);
}

#[test]
fn test_parse() {
    use super::tokenizer;
    let tokens = tokenizer::parse("{1,2,x=function(a,x) {1+a+x} }[i]({},f(32,true))").unwrap();
    Expr::parse(&tokens).unwrap().display_tree(0);
}