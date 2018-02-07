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

    pub fn start() {

    }

    pub fn parse_expression(&mut self) -> Expression {
        let mut i = 0usize;
        let mut exp = match self.lexer.get_cur_tok() {
            Token::LeftParen => {
                return Expression::Dummy
            }
            Token::Num(ref num) => { 
                return Expression::Literal(LiteralExpression::Num(*num))
            },
            Token::Str(ref string) => { 
                return Expression::Literal(LiteralExpression::Str(string.clone()))
            },
            _ => return Expression::Dummy
        };
        return exp
    }

    fn parse_program(&mut self) {
        
    }
}

#[test]
fn test_num_expression() {
    let mut parser = Parser::new("1");
    assert_eq!(parser.parse_expression())
}
