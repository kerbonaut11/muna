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

    pub fn find_matching_bracket(tokens:&[Token],start:usize) -> Option<usize> {
        let open = tokens[start].clone();
        let close = match open {
            Token::RoundO => Token::RoundC,
            Token::CurlyO => Token::CurlyC,
            Token::SquareO => Token::SquareC,
            _ => return None,
        };

        let mut depth = 0;
        for (i,token) in tokens[start..].iter().enumerate() {
            if token == &open  {depth += 1}
            if token == &close {depth -= 1}
            if depth == 0 {
                return Some(i+start);
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

    pub fn brack_depth(&self) -> i32 {
        match self {
            Token::RoundO|Token::CurlyO|Token::SquareO =>  1,
            Token::RoundC|Token::CurlyC|Token::SquareC => -1,
            _ => 0,
        }
    }

    pub fn parse_list_of_idents(tokens:&[Token]) -> Vec<Box<str>> {
        let mut ident_iter = tokens.into_iter();
        let mut idents:Vec<Box<str>> = vec![];
        loop {
            let name = match ident_iter.next() {
                Some(token) => match token {
                    Token::Ident(name) => name,
                    _ => panic!("invalid arg {:?}",token)
                },

                None => break
            };
            idents.push(name.clone());

            match ident_iter.next() {
                Some(token) => match token {
                    Token::Comma => {},
                    _ => panic!("expected comma, got {:?}",token)
                },

                None => break
            }
        }
        idents
    }
}
