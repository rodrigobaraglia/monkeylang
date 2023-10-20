#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT { begin: usize, end: usize },
    STRING { begin: usize, end: usize },
    NUMBER { begin: usize, end: usize, kind: Num },
    INTEGER(i64),
    FLOAT(f64),
    RATIONAL { num: i64, denom: i64 },
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    LTE,
    GTE,
    COMMA,
    SEMICOLON,
    FUNCTION,
    IF,
    ELSE,
    LET,
    TRUE,
    FALSE,
    RETURN,
    RPAREN,
    LPAREN,
    RBRACE,
    LBRACE,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Num {
    Int,
    Float,
    Rat,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interpolation {
    Var {
        begin: usize,
        end: usize,
        parent_key: usize,
    },
    Block {
        begin: usize,
        end: usize,
        parent_key: usize,
    },
}
