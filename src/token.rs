#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT { begin: usize, end: usize },
    STRING { begin: usize, end: usize },
    NUMBER {begin: usize, end: usize, kind: Num},
    INTEGER(i64),
    FLOAT(f64),
    RATIONAL{num:i64, denom:i64},
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
    // TODO: avoid potential allocations for tokens and reduce the size of the Token type.
    // Don't use a vector.
    // Approach the relation between strings and interpolated code as a database normalization problem.
    // Use indices. One for the parent (PK), three for children (begin, end, parent-PK).
    // Tokens of interpolated  code (index triplets) may be stored in a dedicated space of memory, maybe preallocated at the start of the program.
    SUPER_STRING{begin:usize, end:usize, quotes: Vec<Interpolation>}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Num {
    Int,
    Float,
    Rat
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interpolation {
    Var{begin:usize, end:usize},
    Block{begin:usize, end:usize},
}

