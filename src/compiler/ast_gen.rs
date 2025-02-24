use super::{expr::Expr, tokenizer::Token, err::Result};

pub type Block = Vec<AstNode>;

pub enum AstNode {
    Declaration(Assing),
    Assing(Assing),
    If(IfElseStatement),
    For(ForStatement),
    While(WhileStatement),
    Break,
    Return(Vec<Expr>),
}

pub struct Assing {
    lhs:Vec<Expr>,
    rhs:Vec<Expr>
}

pub struct IfElseStatement {
    cond:Option<Box<Expr>>,
    block:Block,
    next:Option<Box<Self>>
}

pub struct WhileStatement {
    cond:Box<Expr>,
    block:Block,
}

pub struct ForStatement {
    for_var1:Box<str>,
    for_var2:Option<Box<str>>,
    iter_type:IterType,
    table:Expr,
    block:Block,
}

#[derive(PartialEq, Eq, Debug)]
enum IterType {
    IPairs,
    KVPairs,
    Range,
    Generic
}

pub struct Function {
    local:bool,
    name:Box<str>,
    args:Vec<Box<str>>,
    block:Block,
}


fn find_start_of_next_statement(tokens:&[Token]) -> usize {
    let mut i = 1usize;
    let mut depth = tokens[0].brack_depth();
    loop {
        if i == tokens.len() {
            return i;
        }
        
        let prev = &tokens[i-1];
        let next = &tokens[i];
        if depth == 0 {
            if Token::is_end_of_expr(prev, next) {
                return i;
            }
        } else if depth < 0 {
            return i;
        };

        depth += next.brack_depth();
        i += 1;
    }
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
        i += offset;
        //println!("{} {}",i,offset);
        block.push(statement);
    }
    return Ok(block);
}


fn parse_statement(tokens:&[Token]) -> Result<(AstNode,usize)> {
    match tokens[0] {
        Token::If => {
            let (s,i) = parse_if_else_statement(tokens,0)?;
            Ok((AstNode::If(s.unwrap()),i+1))
        }

        Token::While => {
            let (s,i) = parse_while_statement(tokens)?;
            Ok((AstNode::While(s),i+1))
        }

        Token::Break => {
            Ok((AstNode::Break,1))
        }

        Token::Return => {
            let (s,i) = parse_list_of_expr(&tokens[1..])?;
            Ok((AstNode::Return(s),i+1))
        }

        Token::For => {
            let (s,i) = parse_for_statement(tokens)?;
            Ok((AstNode::For(s),i+1))
        }

        Token::Ident(_) => {
            let (s,i) = parse_assing(tokens)?;
            Ok((AstNode::Assing(s),i+1))
        }
        
        Token::Local => {
            let (s,i) = parse_assing(&tokens[1..])?;
            Ok((AstNode::Declaration(s),i+2))
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


fn parse_if_else_statement(tokens:&[Token],depth:u32) -> Result<(Option<IfElseStatement>,usize)> {
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
        
            let (next,offset) = parse_if_else_statement(&tokens[bracket_close_idx+1..],depth+1)?;
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


fn parse_while_statement(tokens:&[Token]) -> Result<(WhileStatement,usize)> {
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

fn parse_for_statement(tokens:&[Token]) -> Result<(ForStatement,usize)> {
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


fn parse_list_of_expr(tokens:&[Token]) -> Result<(Vec<Expr>,usize)> {
    let start_of_next_statement = find_start_of_next_statement(tokens);
    let expr = Expr::parse(&tokens[..start_of_next_statement])?;
    if tokens.len() > start_of_next_statement {
        if tokens[start_of_next_statement] == Token::Comma {
            let (mut exprs,offset) = parse_list_of_expr(&tokens[start_of_next_statement+1..])?;
            exprs.insert(0, expr);
            return Ok((exprs,start_of_next_statement+offset+1));
        }
    }
    Ok((vec![expr],start_of_next_statement))
}


fn parse_assing(tokens:&[Token]) -> Result<(Assing,usize)> {
    let assing_idx = Token::find(tokens, &Token::Assing).unwrap();
    let (lhs,offset1) = parse_list_of_expr(&tokens[..assing_idx])?;
    let (rhs,offset2) = parse_list_of_expr(&tokens[offset1+1..])?;
    Ok((
        Assing{lhs,rhs},
        offset1+offset2
    ))
}

#[test]
fn test_find_start_of_next_statement() {
    use super::tokenizer;
    let tokens = tokenizer::parse("map(function(x) x+1 end, x[i+2+3.31]) local x = 10\n").unwrap();
    let i = find_start_of_next_statement(&tokens);
    println!("{} {:?}",i,tokens[i]);

    let tokens = tokenizer::parse("function(x) x+1 end if y = 10\n").unwrap();
    let i = find_start_of_next_statement(&tokens);
    println!("{} {:?}",i,tokens[i]);

    let tokens = tokenizer::parse("function(x) x+1 end[{x=10,y=\"a\"}] function").unwrap();
    let i = find_start_of_next_statement(&tokens);
    println!("{} {:?}",i,tokens[i]);
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
fn expr_list_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("x,(1*3),f(a,b) local").unwrap();
    let (x,i) = parse_list_of_expr(&tokens).unwrap();
    assert_eq!(tokens[i],Token::Local);
    assert!(x.len() == 3);
    for expr in x {
        expr.display_tree(0);
    }
}

#[test]
fn return_break_test() {
    use super::tokenizer;
    let tokens = tokenizer::parse("return a,x+1 break").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::Return(x) => {
            x.iter().for_each(|x| x.display_tree(0));
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
    let tokens = tokenizer::parse("for k,v in kvpairs a[x] {} break").unwrap();
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
    let tokens = tokenizer::parse("x,y[i] = f(x),foo break").unwrap();
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

    let tokens = tokenizer::parse("local x = function(x,y) {return x+y} break").unwrap();
    let x = parse_block(&tokens).unwrap();
    assert!(x.len() == 2);
    match &x[0] {
        AstNode::Declaration(x) => {
            x.lhs.iter().for_each(|x| x.display_tree(0));
            x.rhs.iter().for_each(|x| x.display_tree(0));
        }

        _ => panic!() 
    }

    match x[1] {
        AstNode::Break => {}
        _ => panic!()
    }
}