use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use symtable::*;
use token::Token;
use types::*;

type ResultIndex = Result<Type, usize>;
type ResultError = Result<(), LinterError>;

pub struct LintSource<'s, 't> {
    buffer: &'s str,
    slt: &'t mut SymTable,
    pub issues: Vec<LinterError>,
}

impl<'s, 't> LintSource<'s, 't> {
    pub fn new(buffer: &'s str, slt: &'t mut SymTable) -> Self {
        LintSource {
            buffer,
            slt,
            issues: vec![],
        }
    }
    pub fn type_check(&mut self, to_cmp: &Expr) -> ResultIndex {
        match to_cmp {
            Expr::BinOp(bin) => {
                let left = self.type_check(&bin.left)?;
                let right = self.type_check(&bin.right)?;

                return Err(0);
            }

            Expr::Number(x) => match x.val.token {
                Token::Decimal => Ok(Type::F64),
                Token::Number => Ok(Type::U64),
                _ => panic!("type-lang linter issue"),
            },
            _ => panic!("type-lang linter issue"),
        }
    }
    pub fn update_error(&self, mut err: LinterError, suggestion: String, lexeme: Lexeme) -> () {
        let xcl = CodeLocation::new(self.buffer, lexeme);
        let lep = LinterErrorPoint::new(xcl.code, xcl.line, xcl.col);
        err.add_point(lep, suggestion);
    }
    pub fn make_error(&self, title: String) -> LinterError {
        LinterError::new(title)
    }
}
