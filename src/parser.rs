use tokens::*;
use ast::*;
use lexer::Lexer;

pub struct Parser<'a> {
    lexer : Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(src : &'a str) -> Self {
        Parser {
            lexer : Lexer::new(src)
        }
    }

    pub fn parse(&mut self) {
        let tokens = self.lexer.get_tokens();
        let mut i = 0usize;
        loop { 
            match tokens[i] {
                Token::LeftParen => {
                }
                Token::Num(ref num) => {

                },
                Token::Str(ref string) => {

                }
                _ => {}
            }
        }
    }

    fn parse_program(&mut self) {
        
    }
}
