use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use token::Token;

type ResultCheck = Result<Type, LinterError>;

pub struct LintSource<'s> {
    buffer: &'s str,
    pub issues: Vec<LinterError>,
}

#[derive(Debug)]
pub enum Type {
    Num(u64),
    Dec(f64),
}

impl<'s> LintSource<'s> {
    pub fn new(buffer: &'s str) -> Self {
        LintSource {
            buffer,
            issues: vec![],
        }
    }
    pub fn type_check(&mut self, to_cmp: &mut Expr) -> ResultCheck {
        match to_cmp {
            Expr::TopDecl(x) => {
                let result = self.type_check(&mut x.expr)?;
                return Ok(result);
            }

            Expr::Number(x) => match x.val.token {
                Token::Decimal => Ok(Type::Dec(x.val.slice.parse::<f64>().unwrap())),
                Token::Num => Ok(Type::Num(x.val.slice.parse::<u64>().unwrap())),
                _ => panic!("type-lang linter issue"),
            },
            _ => panic!("type-lang linter issue"),
        }
    }
    pub fn update_error(
        &self,
        mut err: LinterError,
        suggestion: String,
        lexeme: Lexeme,
    ) -> ResultCheck {
        let xcl = CodeLocation::new(self.buffer, lexeme);
        let lep = LinterErrorPoint::new(xcl.code, xcl.line, xcl.col);
        err.add_point(lep, suggestion);
        return Err(err);
    }
    pub fn make_error(&self, title: String) -> LinterError {
        LinterError::new(title)
    }
}
