use super::{expr::Expr, tokenizer::Token, err::Result};

pub type Block = Vec<AstNode>;

pub enum AstNode {
    Declaration(Assing),
    Assing(Assing),
    If(IfElseStatement),
    For(ForStatement),
    While(WhileStatement),
}

pub struct Assing {
    lhs:Vec<Box<str>>,
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
    for_var:Vec<Box<str>>,
    iter_type:IterType,
    expr:Expr,
    block:Block,
}

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
        if i == tokens.len() {
            return i;
        }
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
        i += offset+1;
        //println!("{} {}",i,offset);
        block.push(statement);
    }
    return Ok(block);
}


fn parse_statement(tokens:&[Token]) -> Result<(AstNode,usize)> {
    match tokens[0] {
        Token::If => {
            let (s,i) = parse_if_else_statement(tokens,0)?;
            Ok((AstNode::If(s.unwrap()),i))
        }

        Token::While => {
            let (s,i) = parse_while_statement(tokens)?;
            Ok((AstNode::While(s),i))
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