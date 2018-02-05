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

    pub fn parse_expression(&mut self) {
        let tokens = self.lexer.get_tokens();
        let mut i = 0usize;
        let mut exp;
        while i < tokens.len() { 
            exp = match tokens[i] {
                Token::LeftParen => {
                    Expression::Dummy
                }
                Token::Num(ref num) => { 
                    Expression::Literal(LiteralExpression::Num(*num))
                },
                Token::Str(ref string) => { 
                    Expression::Literal(LiteralExpression::Str(string.clone()))
                }
                _ => Expression::Dummy
            };
        }
    }

    fn parse_program(&mut self) {
        
    }
}
