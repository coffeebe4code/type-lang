use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use symtable::*;
use token::Token;
use types::*;

type ResultTreeType = Result<(Box<TypeTree>, Type), usize>;
type ResultTree = Result<TypeTree, usize>;
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
    pub fn lint_recurse(&mut self, to_cmp: &Expr) -> ResultTreeType {
        match to_cmp {
            Expr::UnOp(un) => match un.op.token {
                Token::Sub => {
                    let result = self.lint_recurse(&un.val)?;
                    let mut unop = UnaryOp {
                        val: result.0,
                        curried: result.1,
                    };
                    match unop.val.as_ref() {
                        TypeTree::F64(_) => unop.curried = Type::F64,
                        TypeTree::U64(_) => unop.curried = Type::I64,
                        TypeTree::U32(_) => unop.curried = Type::I32,
                        TypeTree::I64(_) => unop.curried = Type::I64,
                        TypeTree::I32(_) => unop.curried = Type::I32,
                        _ => panic!("negate_check failed"),
                    }
                    let curried = unop.curried.clone();
                    return Ok((Box::new(TypeTree::Negate(unop)), curried));
                }
                _ => panic!("type-lang linter issue"),
            },
            Expr::Number(num) => match num.val.token {
                Token::Decimal => Ok((
                    Box::new(TypeTree::F64(num.val.slice.parse::<f64>().unwrap())),
                    Type::F64,
                )),
                Token::Num => Ok((
                    Box::new(TypeTree::U64(num.val.slice.parse::<u64>().unwrap())),
                    Type::U64,
                )),
                _ => panic!("type-lang linter issue"),
            },
            _ => panic!("type-lang linter issue"),
        }
    }
    pub fn type_check(&mut self, start: &mut Expr) -> () {
        let _ = self.lint_recurse(start);
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
