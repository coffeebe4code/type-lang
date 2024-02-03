use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;

type ResultCheck = Result<Type, LinterError>;

pub struct LintSource<'s> {
    buffer: &'s str,
}

#[derive(Debug)]
pub enum Type {
    F64(f64),
}

impl<'s> LintSource<'s> {
    pub fn new(buffer: &'s str) -> Self {
        LintSource { buffer }
    }

    pub fn type_check(&mut self, to_cmp: &Expr) -> ResultCheck {
        match to_cmp {
            Expr::TopDecl(x) => self.type_check(&x.expr),
            Expr::Number(x) => {
                //if x.val.slice.parse::<f64>().is_ok() {
                //    return Ok(Type::F64(x.val.slice.parse::<f64>().unwrap()));
                //}
                let err = LinterError::new("invalid range of f64".to_string());
                self.update_error(err, "reduce scope of f64".to_string(), x.val.clone())
            }
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
