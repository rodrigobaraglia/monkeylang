use crate::token::{Interpolation, Num, Token};

pub struct Cursor<'a> {
    len: usize,
    chars: std::str::Chars<'a>,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        let chars = input.chars();
        Cursor {
            len: chars.as_str().len(),
            chars,
        }
    }

    pub fn offset(&self) -> usize {
        self.len - self.chars.as_str().len()
    }

    pub fn next_token(&mut self) -> Token {
        let mut first_char = self.bump();

        if first_char.is_whitespace() {
            self.eat_while(|ch| ch.is_whitespace());
            first_char = self.bump();
        }

        match first_char {
            '\0' => Token::EOF,
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '"' => self.double_quote_string(),
            '\'' => self.single_quote_string(),
            '`' => self.super_string(),
            ch if ch.is_alphabetic() => self.identifier(),
            ch if ch.is_digit(10) => self.number(),
            _ => Token::ILLEGAL,
        }
    }

    pub fn peek(&mut self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    pub fn bump(&mut self) -> char {
        self.chars.next().unwrap_or_default()
    }

    pub fn not_empty(&self) -> bool {
        !self.chars.as_str().is_empty()
    }

    pub fn eat_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while pred(self.peek()) && self.not_empty() {
            self.chars.next();
        }
    }

    pub fn eat_until(&mut self, mut pred: impl FnMut(char) -> bool) {
        while pred(self.peek()) && self.not_empty() {
            self.chars.next();
        }
        self.chars.next();
    }

    pub fn identifier(&mut self) -> Token {
        let begin = self.offset() - 1;
        self.eat_while(|ch| ch.is_alphanumeric());
        let end = self.offset();
        Token::IDENT { begin, end }
    }

    pub fn number(&mut self) -> Token {
        let begin = self.offset() - 1;
        self.eat_while(|ch| ch.is_digit(10));
        match self.peek() {
            '.' => {
                self.bump();
                self.eat_while(|ch| ch.is_digit(10));
                let end = self.offset();
                Token::NUMBER {
                    begin,
                    end,
                    kind: crate::token::Num::Float,
                }
            }
            '/' => {
                self.bump();
                self.eat_while(|ch| ch.is_digit(10));
                let end = self.offset();
                Token::NUMBER {
                    begin,
                    end,
                    kind: crate::token::Num::Rat,
                }
            }
            _ => {
                let end = self.offset();
                Token::NUMBER {
                    begin,
                    end,
                    kind: crate::token::Num::Int,
                }
            }
        }
    }

    pub fn double_quote_string(&mut self) -> Token {
        let begin = self.offset() - 1;
        self.eat_until(|ch| ch != '"');
        let end = self.offset();
        Token::STRING { begin, end }
    }
    pub fn single_quote_string(&mut self) -> Token {
        let begin = self.offset() - 1;
        self.eat_until(|ch| ch != '\'');
        let end = self.offset();
        Token::STRING { begin, end }
    }

    pub fn super_string(&mut self) -> Token {
        let begin = self.offset() - 1;
        let mut ch = self.bump();
        let mut quotes = vec![];
        while ch != '`' {
            if ch == '$' {
                match self.interpolation() {
                    Some(quote) => {
                        quotes.push(quote);
                    }
                    None => return Token::ILLEGAL,
                }
            }
            ch = self.bump();
        }

        let end = self.offset();
        Token::SUPER_STRING { begin, end, quotes }
    }

    pub fn interpolation(&mut self) -> Option<Interpolation> {
        let begin = self.offset() - 1;
        match self.bump() {
            ch if ch.is_alphabetic() => {
                self.eat_while(|ch| ch.is_alphanumeric());
                let end = self.offset();
                let var = Interpolation::Var { begin, end };
                Some(var)
            }
            '{' => {
                let mut lbrace = 1;
                let mut rbrace = 0;
                let mut open = true;

                self.eat_until(|ch| {
                    if ch == '{' {
                        lbrace += 1;
                        open = true;
                    }
                    if ch == '}' {
                        rbrace += 1;
                        open = false;
                    }
                    lbrace != rbrace || open
                });
                let end = self.offset();
                let block = Interpolation::Block { begin, end };
                Some(block)
            }
            _ => None,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            source: input,
            cursor: Cursor::new(input),
        }
    }

    fn next_token(&mut self) -> Token {
        let tok = self.cursor.next_token();
        match tok {
            Token::IDENT { begin, end } => match &self.source[begin..end] {
                "fn" => Token::FUNCTION,
                "let" => Token::LET,
                "true" => Token::TRUE,
                "false" => Token::FALSE,
                "if" => Token::IF,
                "else" => Token::ELSE,
                "return" => Token::RETURN,
                _ => tok,
            },
            Token::NUMBER { begin, end, kind } => match kind {
                Num::Int => {
                    let n = &self.source[begin..end];
                    match n.parse() {
                        Ok(n) => Token::INTEGER(n),
                        Err(e) => {
                            println!("error parsing integer {e}");
                            Token::ILLEGAL
                        }
                    }
                }
                Num::Float => {
                    let f = &self.source[begin..end];
                    match f.parse() {
                        Ok(n) => Token::FLOAT(n),
                        Err(e) => {
                            println!("error parsing float {e}");
                            Token::ILLEGAL
                        }
                    }
                }
                Num::Rat => {
                    let n: Vec<&str> = self.source[begin..end].split('/').collect();
                    match (n[0].parse(), n[1].parse()) {
                        (Ok(num), Ok(denom)) => Token::RATIONAL { num, denom },
                        err => {
                            println!("error parsing rational {:?}", err);
                            Token::ILLEGAL
                        }
                    }
                }
            },
            _ => tok,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_char_tokens() {
        let input = "=+(){},;";

        let tests: Vec<_> = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
        ];
        let mut cursor = Cursor::new(input);
        tests.iter().for_each(|expected_token| {
            let tok: Token = cursor.next_token();
            assert_eq!(
                &tok, expected_token,
                "failed to match token token::Type, expected {:?} got {:?}",
                expected_token, tok
            );
        });
    }

    #[test]
    fn test_identifier_after_whitespace() {
        let input = "   rodrigo";
        let mut cursor = Cursor::new(input);
        let token = cursor.next_token();
        if let Token::IDENT { begin, end } = token {
            let name = &input[begin..end];
            assert_eq!(name, "rodrigo")
        } else {
            panic!("test failed")
        }
    }
    #[test]
    fn test_rational() {
        let input = "   1234/5678";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::RATIONAL { num, denom } = token {
            assert_eq!(num, 1234);
            assert_eq!(denom, 5678)
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_float() {
        let input = "   1234.5678";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::FLOAT(f) = token {
            assert_eq!(f, 1234.5678);
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_integer() {
        let input = "   12345678";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::INTEGER(n) = token {
            assert_eq!(n, 12345678);
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_double_quote_string() {
        let input = "   \"hello 'world'!\"";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::STRING { begin, end } = token {
            let string = &input[begin..end];
            assert_eq!(string, "\"hello 'world'!\"");
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_single_quote_string() {
        let input = "   \'hello \"world\"!\'";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::STRING { begin, end } = token {
            let string = &input[begin..end];
            assert_eq!(string, "\'hello \"world\"!\'");
        } else {
            panic!("test failed")
        }
    }
    #[test]
    fn test_super_string_var() {
        let input = "   `hello $world!`";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::SUPER_STRING { begin, end, quotes } = token {
            let string = &input[begin..end];
            assert_eq!("`hello $world!`", string);
            if let Interpolation::Var { begin, end } = quotes[0] {
                let var = &input[begin..end];
                assert_eq!("$world", var);
            }
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_super_string_block() {
        let input = "   `hello ${if world {foo()} else {bar()}}`";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        if let Token::SUPER_STRING { begin, end, quotes } = token {
            let string = &input[begin..end];
            assert_eq!("`hello ${if world {foo()} else {bar()}}`", string);
            for quote in quotes {
                if let Interpolation::Block { begin, end } = quote {
                    let block = &input[begin..end];
                    assert_eq!("${if world {foo()} else {bar()}}", block);
                }
            }
        } else {
            panic!("test failed")
        }
    }

    #[test]
    fn test_keywords() {
        let input = " fn if else return";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token, Token::FUNCTION);
        let token = lexer.next_token();
        assert_eq!(token, Token::IF);
        let token = lexer.next_token();
        assert_eq!(token, Token::ELSE);
        let token = lexer.next_token();
        assert_eq!(token, Token::RETURN);
    }
}
