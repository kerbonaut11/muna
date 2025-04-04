use crate::{expr::Expr, tokenizer::Token, err::Result};

pub type Block = Vec<AstNode>;

#[derive(Clone)]
pub enum AstNode {
    Declaration(Declaration),
    Assing(Assing),
    Call(Expr),
    If(IfElseStatement),
    For(ForStatement),
    While(WhileStatement),
    Break,
    Return(Option<Expr>),
    Function(Function),
}

#[derive(Clone)]
pub struct Assing {
    pub lhs:Vec<Expr>,
    pub rhs:Vec<Expr>
}

#[derive(Clone)]
pub struct Declaration {
    pub lhs:Vec<Box<str>>,
    pub rhs:Vec<Expr>
}

#[derive(Clone)]
pub struct IfElseStatement {
    pub cond:Option<Box<Expr>>,
    pub block:Block,
    pub next:Option<Box<Self>>
}

#[derive(Clone)]
pub struct WhileStatement {
    pub cond:Box<Expr>,
    pub block:Block,
}

#[derive(Clone)]
pub struct ForStatement {
    pub for_var1:Box<str>,
    pub for_var2:Option<Box<str>>,
    pub iter_type:IterType,
    pub table:Expr,
    pub block:Block,
}

#[derive(Clone)]
pub struct Function {
    pub name:Box<str>,
    pub is_local:bool,
    pub args:Vec<Box<str>>,
    pub block:Block
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum IterType {
    IPairs,
    KVPairs,
    Range,
    Generic
}




pub fn parse_block(tokens:&[Token]) -> Result<Block> {
    if tokens.is_empty() {
        return Ok(vec![]);
    }
    let mut i = 0;
    let mut block = vec![];
    loop {
        if tokens[i..].is_empty() {
            break;
        }
        let (statement,offset) = parse_statement(&tokens[i..])?;
        i += offset+1;
        //println!("{} {}",i,offset);
        block.push(statement);
    }
    return Ok(block);
}


fn parse_statement(tokens:&[Token]) -> Result<(AstNode,usize)> {
    match tokens[0] {
        Token::If => {
            let (s,i) = parse_if_else(tokens,0)?;
            Ok((AstNode::If(s.unwrap()),i))
        }

        Token::While => {
            let (s,i) = parse_while(tokens)?;
            Ok((AstNode::While(s),i))
        }

        Token::Break => {
            Ok((AstNode::Break,1))
        }

        Token::Return => {
            let end_idx = Token::find_outside_of_brackets(tokens, &Token::Endline).unwrap();
            Ok((AstNode::Return(Some(Expr::parse(&tokens[1..end_idx])?)),end_idx))
        }

        Token::For => {
            let (s,i) = parse_for(tokens)?;
            Ok((AstNode::For(s),i))
        }

        Token::Ident(_) => {
            let end_idx = Token::find_outside_of_brackets(tokens, &Token::Endline).unwrap();
            match Token::find_outside_of_brackets(tokens, &Token::Endline)  {
                Some(_) => Ok((AstNode::Assing(parse_assing(&tokens[..end_idx])?),end_idx)),
                None => Ok((AstNode::Call(Expr::parse(&tokens[..end_idx])?),end_idx))
            }
        }
        
        Token::Local => {
            if tokens[1] != Token::Function { 
                let end_idx = Token::find_outside_of_brackets(tokens, &Token::Endline).unwrap();
                let s = parse_declaration(&tokens[..end_idx])?;
                Ok((AstNode::Declaration(s),end_idx))
            } else {
                let (mut f,i) = parse_function(&tokens[1..])?;
                f.is_local = true;
                Ok((AstNode::Function(f),i+1))
            }
        }

        Token::Function => {
            let (f,i) = parse_function(tokens)?;
            Ok((AstNode::Function(f),i))
        }

        _ => panic!("invalid token {:?} {:?}",tokens[0],tokens)
    }
}


fn parse_cond(tokens:&[Token]) -> Result<(Expr,usize)> {
    let mut i = 1;
    let mut depth = 0;
    loop {
        if depth == 0 && tokens[i] == Token::CurlyO {
            break;
        }
        depth += tokens[i].brack_depth();
        i += 1;
    }

    let bracket_open_idx = i;
    let cond = Expr::parse(&tokens[1..bracket_open_idx])?;
    Ok((cond,bracket_open_idx))
}


fn parse_if_else(tokens:&[Token],depth:u32) -> Result<(Option<IfElseStatement>,usize)> {
    //println!("parsing elif {:?}",tokens);
    if tokens.is_empty() {
        //println!("got empty tokens, returning none");
        return Ok((None,(depth-1) as usize));
    }

    if !(depth == 0 || tokens[0] == Token::Elif|| tokens[0] == Token::Else) {
        //println!("got no if/else, returning none");
        return Ok((None,(depth-1) as usize));
    }

    match tokens[0] {
        Token::If | Token::Elif => {
            //println!("got if/elif");
            let (cond,bracket_open_idx) = parse_cond(tokens).unwrap();
            let bracket_close_idx = Token::find_matching_bracket(tokens,bracket_open_idx).unwrap();
            let block = parse_block(&tokens[bracket_open_idx+1..bracket_close_idx]).unwrap();
        
            let (next,offset) = parse_if_else(&tokens[bracket_close_idx+1..],depth+1)?;
            //println!("offset {}",bracket_close_idx+offset);
            return Ok((
                Some(IfElseStatement{
                    cond:Some(Box::new(cond)),
                    block,
                    next:next.map(|x| Box::new(x))
                }),
                bracket_close_idx+offset
            ));
        }

        Token::Else => {
            //println!("got else");
            let bracket_close_idx = Token::find_matching_bracket(tokens,1).unwrap();
            let block = parse_block(&tokens[2..bracket_close_idx]).unwrap();
            //println!("offset {}",bracket_close_idx);
            return Ok((
                Some(IfElseStatement{
                    cond: None,
                    block,
                    next: None
                }),
                bracket_close_idx+depth as usize
            ));
        }

        _ => panic!()
    }
}


fn parse_while(tokens:&[Token]) -> Result<(WhileStatement,usize)> {
    let (cond,bracket_open_idx) = parse_cond(tokens).unwrap();
    let bracket_close_idx = Token::find_matching_bracket(tokens,bracket_open_idx).unwrap();
    let block = parse_block(&tokens[bracket_open_idx+1..bracket_close_idx]).unwrap();
    return Ok((
        WhileStatement{
            cond:Box::new(cond),
            block,
        },
        bracket_close_idx
    ));
}

fn parse_for(tokens:&[Token]) -> Result<(ForStatement,usize)> {
    let in_idx = Token::find(tokens, &Token::In).unwrap();
    let for_vars = Token::parse_list_of_idents(&tokens[1..in_idx]);
    let iter_type = match tokens[in_idx+1] {
        Token::IPairs  => IterType::IPairs,
        Token::KVPairs => IterType::KVPairs,
        Token::Range   => IterType::Range,
        _ => panic!()
    };

    let (table,open_bracket_idx) = parse_cond(&tokens[in_idx+1..])?;
    let open_bracket_idx = open_bracket_idx+in_idx+1;
    let close_bracket_idx = Token::find_matching_bracket(tokens, open_bracket_idx).unwrap();
    let block = parse_block(&tokens[open_bracket_idx+1..close_bracket_idx]).unwrap();

    Ok((
        ForStatement{
            for_var1:for_vars[0].clone(),
            for_var2:if tokens.len() != 1 {
                Some(for_vars[1].clone())
            } else {
                None
            },
            iter_type,
            table,
            block
        },
        close_bracket_idx
    ))
}


fn parse_list_of_expr(tokens:&[Token]) -> Result<Vec<Expr>> {
    let mut out = vec![];
    parse_list_of_expr_rec(tokens,&mut out)?;
    Ok(out)
}

fn parse_list_of_expr_rec(tokens:&[Token], result: &mut Vec<Expr>) -> Result<()> {
    if tokens.is_empty() {return Ok(());}
    match Token::find_outside_of_brackets(tokens, &Token::Comma) {
        Some(i) => {
            result.push(Expr::parse(&tokens[..i])?);
            parse_list_of_expr_rec(&tokens[i+1..], result)?;
            Ok(())
        }

        None => {
            result.push(Expr::parse(tokens)?);
            Ok(())
        }
    }
}


fn parse_assing(tokens:&[Token]) -> Result<Assing> {
    let assing_idx = Token::find(tokens, &Token::Assing).unwrap();
    let lhs = parse_list_of_expr(&tokens[..assing_idx])?;
    let rhs = parse_list_of_expr(&tokens[assing_idx+1..])?;
    Ok(Assing{lhs,rhs})
}

fn parse_declaration(tokens:&[Token]) -> Result<Declaration> {
    let assing_idx = Token::find(tokens, &Token::Assing).unwrap();
    let lhs = Token::parse_list_of_idents(&tokens[1..assing_idx]);
    let rhs = parse_list_of_expr(&tokens[assing_idx+1..])?;
    Ok(Declaration{lhs,rhs})
}

fn parse_function(tokens:&[Token]) -> Result<(Function,usize)> {
    let name = match &tokens[1]  {
        Token::Ident(name) => name.clone(),
        _ => unreachable!()
    };

    let open_block_idx = Token::find(tokens, &Token::CurlyO).unwrap();
    let close_block_idx = Token::find_matching_bracket(tokens, open_block_idx).unwrap();

    let args = Token::parse_list_of_idents(&tokens[3..open_block_idx-1]);
    let block = parse_block(&tokens[open_block_idx+1..close_block_idx])?;

    return Ok((
        Function{
            is_local:false,
            name,
            args,
            block
        },
        close_block_idx
    ));
}


#[test]
fn if_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("if x == 2 {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    match &x[0] {
        AstNode::If(IfElseStatement { cond, block, next }) => {
            cond.as_ref().expect("expected x == 2").display_tree(0);
            assert!(block.is_empty());
            assert!(next.is_none());
        }
        _ => panic!()
    }

    let tokens = tokenizer::parse("if x == 2 {} elif y {} else {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    match &x[0] {
        AstNode::If(IfElseStatement { cond, block, next }) => {
            cond.as_ref().unwrap().display_tree(0);
            assert!(block.is_empty());
            let next = next.as_ref().expect("expected next");
            assert!(next.next.as_ref().unwrap().cond.is_none());
            assert!(next.next.as_ref().unwrap().next.is_none());
        }
        _ => panic!()
    }

    let tokens = tokenizer::parse("if x == 2 {} if x {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);


    let tokens = tokenizer::parse("if x == 2 {} else {} if x {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);

    let tokens = tokenizer::parse("if x == 2 {} elif x {} elif x {} elif x {} elif x {} elif x {}   if x {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
}

#[test]
fn while_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("while x == 2 {}").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(match &x[0] {
        AstNode::While(x) => {
            x.cond.display_tree(0);
            assert!(x.block.is_empty());
            true
        }

        _ => false
    })
}


#[test]
fn return_break_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("return x+1; break;").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::Return(x) => {
            x.as_ref().unwrap().display_tree(0);
        }
        _ => panic!() 
    }


    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }
}

#[test]
fn for_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("for k,v in kvpairs a[x] {} break;").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::For(x) => {
            assert!(x.for_var1 == "k".into());
            assert!(x.for_var2 == Some("v".into()));
            assert!(x.block.is_empty());
            assert!(match x.table {
                Expr::Index{ .. } => true,
                _ => false
            });
            assert!(x.iter_type == IterType::KVPairs);
        }

        _ => panic!() 
    }

    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }
}

#[test]
fn assing_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("x,y[i] = f(x),foo; break;").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::Assing(x) => {
            x.lhs.iter().for_each(|x| x.display_tree(0));
            x.rhs.iter().for_each(|x| x.display_tree(0));
        }

        _ => panic!() 
    }

    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }

    let tokens = tokenizer::parse("local x = function(x,y) {return x+y;}; break;").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert_eq!(x.len(),2);
    match &x[0] {
        AstNode::Declaration(x) => {
            x.lhs.iter().for_each(|x| println!("x"));
            x.rhs.iter().for_each(|x| x.display_tree(0));
        }

        _ => panic!() 
    }

    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }
}


#[test]
fn function_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("local function f(a,b,hello) {return {1,2,\"e\"}; } break;").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::Function(f) => {
            assert_eq!(f.name,"f".into());
            assert_eq!(f.args[0],"a".into());
            assert_eq!(f.args[1],"b".into());
            assert_eq!(f.args[2],"hello".into());
            assert!(match f.block[0] {
               AstNode::Return(_) => true,
               _ => false 
            });
        }

        _ => panic!() 
    }

    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }
}