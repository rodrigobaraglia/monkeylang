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
    IDENTIFIER(std::rc::Rc<Identifier<'a>>),
    INTEGER_LITERAL(IntegerLiteral),
}

struct Identifier<'a> {
    token: Token,
    key: &'a str,
    value: Expression<'a>
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
