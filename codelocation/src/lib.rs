use lexer::*;
use logos::Lexer;
use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeLocation {
    pub val: String,
    pub line: usize,
    pub code: String,
    pub col: usize,
}

impl<'s> CodeLocation {
    pub fn new_lexer_stop_point(lexer: &Lexer<Token>) -> Self {
        let chars = lexer
            .source()
            .get(0..lexer.span().end)
            .unwrap_or_else(|| "");
        let val = lexer
            .source()
            .get(lexer.span().start..lexer.span().end)
            .unwrap_or_else(|| "(empty)");
        let mut lines = chars.split('\n').collect::<Vec<&str>>();
        let mut code = "".to_string();
        let mut col = 0;
        if lines.len() > 0 {
            code.push_str(lines.pop().unwrap());
            for x in lines.iter() {
                for _ in x.bytes() {
                    col += 1;
                }
            }
        } else {
            code.push_str(lexer.source());
        }

        return CodeLocation {
            val: val.to_string(),
            line: lines.len() + 1,
            code,
            col: lexer.span().start - col - lines.len() + 1,
        };
    }
}

impl<'s> CodeLocation {
    pub fn new(raw: &'s str, lexeme: Lexeme) -> Self {
        let chars = raw.get(0..lexeme.span.end).unwrap_or_else(|| "");
        let val = raw
            .get(lexeme.span.start..lexeme.span.end)
            .unwrap_or_else(|| "(empty)");
        let mut lines = chars.split('\n').collect::<Vec<&str>>();
        let mut code = "".to_string();
        let mut col = 0;
        if lines.len() > 0 {
            code.push_str(lines.pop().unwrap());
            for x in lines.iter() {
                for _ in x.bytes() {
                    col += 1;
                }
            }
        } else {
            code.push_str(raw);
        }

        return CodeLocation {
            val: val.to_string(),
            line: lines.len() + 1,
            code,
            col: lexeme.span.start - col - lines.len() + 1,
        };
    }
}
