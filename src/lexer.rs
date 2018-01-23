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

        match self.src[self.i] as char {
            '(' => {self.i = self.i + 1; return Token::LeftParen},
            ')' => {self.i = self.i + 1; return Token::RightParen}, 
            _ => Token::Invalid
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
