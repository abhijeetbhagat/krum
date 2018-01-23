use tokens::Token;

pub struct Lexer<'a> {
    src : &'a [u8],
    cur_tok : Token,
    cur_tok_line : usize,
    cur_tok_col : usize,
    i : usize,
    src_len : usize
}

impl<'a> Lexer<'a> { 
    pub fn new(src : &'a str) -> Self { 
        Lexer {
            src : src.as_bytes(),
            cur_tok : Token::Eof,
            cur_tok_line : 0,
            cur_tok_col : 0,
            i : 0,
            src_len : src.len()
        }
    }

    pub fn get_cur_tok(&self) -> Token { 
        self.cur_tok.clone()
    }

    pub fn get_tok(&mut self) -> Token { 
        self.read_and_advance()
    }

    fn read_and_advance(&mut self) -> Token {
        if self.i == self.src_len {
            return Token::Eof
        }

        let mut tokens = vec![];
        loop { 
            let tok = self.parse_left_paren();
            if tok.is_none() {
                tok = self.parse_right_paren();
            }
            if tok.is_none() {
                tok = self.parse_string();
            }
            if tok.is_none() {
                tok = self.parse_ident();
            }
            if tok.is_some() {
                tokens.push(tok);
            }
        }
        match self.src[self.i] as char {
            '(' => {self.i = self.i + 1; return Token::LeftParen},
            ')' => {self.i = self.i + 1; return Token::RightParen}, 
            _ => Token::Invalid
        }
    }

    fn parse_left_paren(&mut self) -> Option<Token>{
        match self.src[i] { 
            '(' => Ok(Token::LeftParen),
            _ => None
        }
    }

    fn parse_right_paren(&mut self) -> Option<Token>{
        match self.src[i] { 
            ')' => Ok(Token::RightParen),
            _ => None
        }
    }

    fn parse_string(&mut self) -> Option<Token> {
        let mut s = String::new();
        match self.src[i] {
            '"' => loop { 
                self.i = self.i + 1;
            },
            _ => None
        }
    }
}

#[test]
fn test_token_read_1() { 
    let mut l = Lexer::new("()");
    assert_eq!(l.get_cur_tok(), Token::Eof);
    assert_eq!(l.get_tok(), Token::LeftParen);
    assert_eq!(l.get_tok(), Token::RightParen);
}
