use super::tokenizer::Token;

impl Token {
    pub fn find(tokens:&[Token],target:&Token) -> Option<usize> {
        for (i,token) in tokens.iter().enumerate() {
            if token == target {return Some(i);}
        }
        None
    }

    pub fn find_outside_of_brackets(tokens:&[Token],target:&Token) -> Option<usize> {
        let mut depth = 0;
        for (i,token) in tokens.iter().enumerate() {
            depth += token.brack_depth();
            if token == target && depth <= 0 {
                return Some(i);
            }
        }
        None
    }

    pub fn find_matching_bracket(tokens:&[Token]) -> Option<usize> {
        let open = tokens[0].clone();
        let close = match open {
            Token::RoundO => Token::RoundC,
            Token::CurlyO => Token::CurlyC,
            Token::SquareO => Token::SquareC,
            Token::Do|Token::Function => Token::End,
            _ => return None,
        };

        let mut depth = 0;
        for (i,token) in tokens.iter().enumerate() {
            if token == &open  {depth += 1}
            if token == &close {depth -= 1}
            if depth == 0 {
                return Some(i);
            }
        }
        None
    }

    pub fn find_matching_bracket_rev(tokens:&[Token]) -> Option<usize> {
        let open = tokens.last().unwrap().clone();
        let close = match open {
            Token::RoundC => Token::RoundO,
            Token::CurlyC => Token::CurlyO,
            Token::SquareC => Token::SquareO,
            _ => return None,
        };

        let mut depth = 0;
        for (i,token) in tokens.iter().enumerate().rev() {
            if token == &open  {depth += 1}
            if token == &close {depth -= 1}
            if depth == 0 {
                return Some(i);
            }
        }
        None
    }
}

fn find_start_of_next_statement(tokens:&[Token]) -> usize {
    let mut i = 1usize;
    let mut depth = tokens[0].brack_depth();
    loop {
        let prev = &tokens[i-1];
        let next = &tokens[i];
        println!("{}",depth);
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