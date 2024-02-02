use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;

type ResultCheck = Result<Type, LinterError>;

pub struct LintSource<'s> {
    buffer: &'s str,
}

enum Type {
    Custom(usize),
    U64,
    F64(f64),
}

impl<'s> LintSource {
    pub fn new(buffer: &'s str) -> Self {
        LintSource { buffer }
    }

    pub fn type_check(&mut self) -> ResultCheck {
        match to_cmp {
            Expr::Number(x) => {
                if x.val.slice.parse::<f64>().is_ok() {
                    Ok(Type::F64(x.val.slice.parse::<f64>().unwrap()))
                }
            }
            _ => panic!("type-lang linter issue"),
        }
    }
    pub fn lint_binop(&mut self, binop: &BinOp) -> ResultCheck {
        Ok(())
    }
    pub fn make_(&self, title: String, lexeme: &Lexeme) -> LinterError {
        let x = CodeLocation::new(self.buffer, lexeme: &Lexeme);
        return LinterError::new(title, x.code, x.line, x.col, x.val);
    }
}
