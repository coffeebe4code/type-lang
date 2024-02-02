use logos::{Lexer, Logos};
use std::ops::Range;
use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub token: Token,
    pub span: Range<usize>,
    pub slice: String,
}

//#[derive(Debug, Clone, PartialEq)]
//pub struct CodeLocation {
//    pub val: String,
//    pub line: usize,
//    pub code: String,
//    pub col: usize,
//}
//
//impl<'s> CodeLocation {
//    pub fn new(lexer: &Lexer<Token>) -> Self {
//        let chars = lexer
//            .source()
//            .get(0..lexer.span().end)
//            .unwrap_or_else(|| "");
//        let val = lexer
//            .source()
//            .get(lexer.span().start..lexer.span().end)
//            .unwrap_or_else(|| "(empty)");
//        let mut lines = chars.split('\n').collect::<Vec<&str>>();
//        let mut code = "".to_string();
//        let mut col = 0;
//        if lines.len() > 0 {
//            code.push_str(lines.pop().unwrap());
//            for x in lines.iter() {
//                for _ in x.bytes() {
//                    col += 1;
//                }
//            }
//        } else {
//            code.push_str(lexer.source());
//        }
//
//        return CodeLocation {
//            val: val.to_string(),
//            line: lines.len() + 1,
//            code,
//            col: lexer.span().start - col - lines.len() + 1,
//        };
//    }
////}

pub struct TLexer<'s> {
    current: Option<Lexeme>,
    pub lexer: Lexer<'s, Token>,
}

impl<'s> TLexer<'s> {
    pub fn new(buffer: &'s str) -> Self {
        return TLexer {
            current: None,
            lexer: Token::lexer(buffer),
        };
    }
    pub fn collect_if(&mut self, token: Token) -> Option<Lexeme> {
        if self.peek()?.token.is_kind(token) {
            return Some(self.collect());
        }
        return None;
    }
    pub fn collect_of_if(&mut self, token: &[Token]) -> Option<Lexeme> {
        if self.peek()?.token.is_of_kind(token) {
            return Some(self.collect());
        }
        return None;
    }
    pub fn peek(&mut self) -> Option<Lexeme> {
        if self.current.is_none() {
            match self.lexer.next() {
                Some(val) => {
                    self.current = Some(Lexeme {
                        token: val.unwrap(),
                        span: self.lexer.span(),
                        slice: String::from(self.lexer.slice()),
                    });
                }
                None => self.current = None,
            }
        }
        self.current.clone()
    }
    pub fn has_token_consume(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(lexeme) => {
                if lexeme.token == token {
                    self.collect();
                    return true;
                }
                return false;
            }
            None => false,
        }
    }
    pub fn collect(&mut self) -> Lexeme {
        let temp = self.current.clone().unwrap();
        self.current = None;
        return temp;
    }
}
