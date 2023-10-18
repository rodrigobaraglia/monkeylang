use crate::token::Token;

struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

enum Statement<'a> {
    LET {
        token: Token,
        name: Identifier<'a>,
        value: Expression<'a>,
    },
    RETURN {
        token: Token,
        value: Expression<'a>,
    },
}

enum Expression<'a> {
    IDENTIFIER(Identifier<'a>),
    INTEGER_LITERAL(IntegerLiteral),
}

struct Identifier<'a> {
    token: Token,
    value: &'a str,
}

struct IntegerLiteral {
    token: Token,
    value: i64,
}
