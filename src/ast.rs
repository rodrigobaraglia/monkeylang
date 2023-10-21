use crate::token::Token;
use std::rc::Rc;

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
    EXPRESSION(Expression<'a>),
}

enum Expression<'a> {
    IDENTIFIER(Rc<Identifier<'a>>),
    INTEGER_LITERAL(IntegerLiteral),
    BIN_OP(Rc<BinaryOp<'a>>),
    UN_OP(Rc<UnaryOp<'a>>),
}

struct Identifier<'a> {
    token: Token,
    key: &'a str,
    value: Expression<'a>,
}

struct IntegerLiteral {
    token: Token,
    value: i64,
}

struct BinaryOp<'a> {
    op: Token,
    arg1: Expression<'a>,
    arg2: Expression<'a>,
}

struct UnaryOp<'a> {
    op: Token,
    arg: Expression<'a>,
}
