pub enum Expression { 
    Ident(String),
    Literal(LiteralExpression),
    ProcedureCall,
    Lambda,
    Conditional,
    Assignment,
    Derived(DerivedExpression)
}

pub enum LiteralExpression {
    Bool(bool),
    Num(f64),
    Char(char),
    Str(String),
}

pub enum DerivedExpression {
    Cond,
    CondElse,
    Case,
    CaseElse,
    And,
    Or,
    When,
    Unless,
    Let 
}

pub struct Program {
    expression : Expression
}

