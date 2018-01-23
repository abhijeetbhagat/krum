#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident,
    Bool,
    Num,
    Char,
    Str,
    HashLeftParen,
    HashU8LeftParen,
    Apostrophe,
    BackTick,
    Comma,
    CommaAtTheRate,
    Dot,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    BackwardSlash,
    Exclaimation,
    Dollar,
    Percent,
    Ampersand,
    Colon,
    LessThan,
    EqualTo,
    GreaterThan,
    Question,
    AtTheRate,
    Caret,
    Underscore,
    Tilde,
    LeftParen,
    RightParen,
    DoubleQuote,
    SemiColon,
    Eof,
    Invalid
}
