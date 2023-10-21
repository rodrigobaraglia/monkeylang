use crate::token::Token;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]

struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]

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

#[derive(Debug, PartialEq, Clone)]
enum Expression<'a> {
    IDENTIFIER(Rc<Identifier<'a>>),
    INTEGER_LITERAL(IntegerLiteral),
    BIN_OP(Rc<BinaryOp<'a>>),
    UN_OP(Rc<UnaryOp<'a>>),
}

#[derive(Debug, PartialEq, Clone)]
struct Identifier<'a> {
    token: Token,
    key: &'a str,
    value: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone, Copy)]

struct IntegerLiteral {
    token: Token,
    value: i64,
}

#[derive(Debug, PartialEq, Clone)]


struct BinaryOp<'a> {
    op: Token,
    arg1: Expression<'a>,
    arg2: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]

struct UnaryOp<'a> {
    op: Token,
    arg: Expression<'a>,
}
