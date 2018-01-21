use tokens::Token;

pub struct Lexer {
    cur_tok : Token,
    cur_tok_line : usize,
    cur_tok_col : usize,
}

impl Lexer { 
    fn new() -> Self { 
        Lexer {
            cur_tok : Token::Eof,
            cur_tok_line : 0,
            cur_tok_col : 0,
        }
    }
}
