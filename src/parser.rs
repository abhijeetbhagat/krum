use tokens::*;
use ast::*;
use lexer::Lexer;

pub struct Parser<'a> {
    lexer : Lexer<'a>,
    cur_tok : Token,
    i : usize,
}

impl<'a> Parser<'a> {
    pub fn new(src : &'a str) -> Self {
        Parser {
            lexer : Lexer::new(src),
            cur_tok : Token::Eof,
            i : 0usize
        }
    }

    pub fn start(&mut self) { 
        self.lexer.tokenize();
    }
    
    fn get_cur_tok(&self) -> Token { 
        self.lexer.tokens[self.i].clone()
    }

    pub fn parse_expression(&mut self) -> Expression {
        let mut exp = match self.get_cur_tok() {
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
    parser.start();
    assert_eq!(parser.parse_expression(), Expression::Literal(LiteralExpression::Num(1f64)));
}
