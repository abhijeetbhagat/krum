pub enum Expression { 
    Ident(String),
    Literal,
    ProcedureCall,
    Lambda,
    Conditional,
    Assignment,
    Derived(DerivedExpression)
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

