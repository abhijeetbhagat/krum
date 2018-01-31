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
                tok = self.parse_bool();
            }
            if tok.is_none() {
                tok = self.parse_symbol();
            }
            if tok.is_some() {
                tokens.push(tok.unwrap());
                self.i = self.i + 1;
            } else if self.src[self.i] as char == ' ' {
                self.i = self.i + 1;
            }
            if self.i >= (self.src.len()){
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

    fn parse_symbol(&mut self) -> Option<Token> {
        let mut symbol = String::new();
        loop { 
            match self.src[self.i] as char { 
                '(' | 
                ')' |
                '.' |
                '#' |
                '\'' |
                ' ' |
                '\n' |
                '\t' |
                '"' |
                '[' |
                ']' => break,
                c @ _ => {
                    symbol.push(c);
                }
            }
            self.i = self.i + 1;
            //println!("self.i - {}, symbol - {}", self.i, symbol);
            if self.i == self.src.len() {
                //self.i = self.i - 1;
                break;
            }
        }
        if !symbol.is_empty() {
            //println!("sym {} with len - {}", symbol, symbol.len());
            self.i = self.i - 1;
            match symbol.parse::<f64>() {
                Ok(n) => return Some(Token::Num(n)),
                Err(_) => {}
            }

            return Some(Token::Ident(symbol))
        }

        None
    } 

    fn parse_bool(&mut self) -> Option<Token> {
        if self.src[self.i] as char == '#' {
            match self.src[self.i + 1] as char  {
                't' => {self.i = self.i + 1; return Some(Token::Bool(true))},
                'f' => {self.i = self.i + 1; return Some(Token::Bool(false))},
                _ => return None
            }

        }
        None
    }
}

#[test]
fn test_token_read_empty_parens() {
    let mut l = Lexer::new("()");
    assert_eq!(l.get_cur_tok(), Token::Eof);
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::LeftParen);
    assert_eq!(tokens[1], Token::RightParen);
}

#[test]
fn test_token_read_string() {
    let mut l = Lexer::new("\"abhi\"");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Str("abhi".to_owned()), tokens[0]);
}

#[test]
fn test_token_read_bool_true() {
    let mut l = Lexer::new("#t");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Bool(true), tokens[0]);
}

#[test]
fn test_token_read_bool_false() {
    let mut l = Lexer::new("#f");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Bool(false), tokens[0]);
}

#[test]
fn test_token_read_num_with_explicit_pos_sign() {
    let mut l = Lexer::new("+1");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Num(1f64), tokens[0]);
}

#[test]
fn test_token_read_num_with_explicit_neg_sign() {
    let mut l = Lexer::new("-1");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Num(-1f64), tokens[0]);
}

#[test]
fn test_token_read_symbol_len_1() {
    let mut l = Lexer::new("a");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Ident("a".to_owned()), tokens[0]);
}

#[test]
fn test_token_read_symbol_multiple_chars() {
    let mut l = Lexer::new("foo");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Ident("foo".to_owned()), tokens[0]);
}

#[test]
fn test_token_read_symbol_plus() {
    let mut l = Lexer::new("+");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 1);
    assert_eq!(Token::Ident("+".to_owned()), tokens[0]);
}

#[test]
fn test_token_read_add_expression() {
    let mut l = Lexer::new("(+ 1 2)");
    let tokens = l.get_tokens();
    assert_eq!(tokens.len(), 5);
    assert_eq!(Token::LeftParen, tokens[0]);
    assert_eq!(Token::Ident("+".to_owned()), tokens[1]);
    assert_eq!(Token::Num(1f64), tokens[2]);
    assert_eq!(Token::Num(2f64), tokens[3]);
    assert_eq!(Token::RightParen, tokens[4]);
}
