use tokens::Token;

pub struct Lexer<'a> {
    src : &'a [u8],
    cur_tok_line : usize,
    cur_tok_col : usize,
    i : usize,
    src_len : usize,
    pub tokens : Vec<Token>
}

impl<'a> Lexer<'a> { 
    pub fn new(src : &'a str) -> Self { 
        Lexer {
            src : src.as_bytes(),
            cur_tok_line : 0,
            cur_tok_col : 0,
            i : 0,
            src_len : src.len(),
            tokens : vec![]
        }
    }


    pub fn tokenize(&mut self) {
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
                self.tokens.push(tok.unwrap());
                self.i = self.i + 1;
            } else if self.src[self.i] as char == ' ' {
                self.i = self.i + 1;
            }
            if self.i >= (self.src.len()){
                break;
            }
        }
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
    l.tokenize();
    assert_eq!(l.tokens.len(), 2);
    assert_eq!(l.tokens[0], Token::LeftParen);
    assert_eq!(l.tokens[1], Token::RightParen);
}

#[test]
fn test_token_read_string() {
    let mut l = Lexer::new("\"abhi\"");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Str("abhi".to_owned()), l.tokens[0]);
}

#[test]
fn test_token_read_bool_true() {
    let mut l = Lexer::new("#t");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Bool(true), l.tokens[0]);
}

#[test]
fn test_token_read_bool_false() {
    let mut l = Lexer::new("#f");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Bool(false), l.tokens[0]);
}

#[test]
fn test_token_read_num_with_explicit_pos_sign() {
    let mut l = Lexer::new("+1");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Num(1f64), l.tokens[0]);
}

#[test]
fn test_token_read_num_with_explicit_neg_sign() {
    let mut l = Lexer::new("-1");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Num(-1f64), l.tokens[0]);
}

#[test]
fn test_token_read_symbol_len_1() {
    let mut l = Lexer::new("a");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Ident("a".to_owned()), l.tokens[0]);
}

#[test]
fn test_token_read_symbol_multiple_chars() {
    let mut l = Lexer::new("foo");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Ident("foo".to_owned()), l.tokens[0]);
}

#[test]
fn test_token_read_symbol_plus() {
    let mut l = Lexer::new("+");
    l.tokenize();
    assert_eq!(l.tokens.len(), 1);
    assert_eq!(Token::Ident("+".to_owned()), l.tokens[0]);
}

#[test]
fn test_token_read_add_expression() {
    let mut l = Lexer::new("(+ 1 2)");
    l.tokenize();
    assert_eq!(l.tokens.len(), 5);
    assert_eq!(Token::LeftParen, l.tokens[0]);
    assert_eq!(Token::Ident("+".to_owned()), l.tokens[1]);
    assert_eq!(Token::Num(1f64), l.tokens[2]);
    assert_eq!(Token::Num(2f64), l.tokens[3]);
    assert_eq!(Token::RightParen, l.tokens[4]);
}
