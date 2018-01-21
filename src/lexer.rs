use tokens::Token;

pub struct Lexer<'a> {
    src : &'a str,
    cur_tok : Token,
    cur_tok_line : usize,
    cur_tok_col : usize,
    i : usize,
    src_len : usize
}

impl<'a> Lexer<'a> { 
    pub fn new(src : &'a str) -> Self { 
        Lexer {
            src,
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

    pub fn get_tok(&self) -> Token {

    }

    fn read_and_advance(&mut self) -> Token {

    }
}

#[test]
fn test_token_read_1() { 
    let l = Lexer::new("");
    assert_eq!(l.get_cur_tok(), Token::Eof);
}
