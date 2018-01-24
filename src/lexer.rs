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

    pub fn get_tokens(&mut self) -> Vec<Token> { 
        self.tokenize()
    }

    fn tokenize(&mut self) -> Vec<Token> {
        if self.i == self.src_len {
            return vec![]
        }

        let mut tokens = vec![];
        loop { 
            let mut tok = self.parse_left_paren();
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
                tokens.push(tok.unwrap());
                self.i = self.i + 1;
            } 
            if self.i == (self.src.len()){
                break;
            }
        }
        tokens
    }

    fn parse_left_paren(&mut self) -> Option<Token>{
        match self.src[self.i] as char { 
            '(' => Some(Token::LeftParen),
            _ => None
        }
    }

    fn parse_right_paren(&mut self) -> Option<Token>{
        match self.src[self.i] as char { 
            ')' => Some(Token::RightParen),
            _ => None
        }
    }

    fn parse_string(&mut self) -> Option<Token> {
        let mut s = String::new();
        match self.src[self.i] as char {
            '"' => {
                loop { 
                    self.i = self.i + 1;
                    if self.src[self.i] as char == '"' {
                        break;
                    }
                    s.push(self.src[self.i].clone() as char);
                }
            },
            _ => return None
        }
        Some(Token::Str(s))
    }

    fn parse_ident(&mut self) -> Option<Token> {
        unimplemented!();
    }
}

#[test]
fn test_token_read_1() {
    let mut l = Lexer::new("()");
    assert_eq!(l.get_cur_tok(), Token::Eof);
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::LeftParen);
    assert_eq!(tokens[1], Token::RightParen);
}
