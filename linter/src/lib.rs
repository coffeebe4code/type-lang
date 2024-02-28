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
                Token::Sub => self.check_negate(un),
                Token::NotLog => self.check_not(un),
                Token::And => self.check_borrow_ro(un),
                Token::Mul => self.check_borrow_mut(un),
                Token::WCopy => self.check_copy(un),
                Token::WClone => self.check_clone(un),
                _ => panic!("type-lang linter issue, unary op not implemented"),
            },
            Expr::Number(num) => match num.val.token {
                Token::Decimal => self.check_f64(num),
                Token::Num => self.check_u64(num),
                _ => panic!("type-lang linter issue, number not implemented"),
            },
            Expr::TopDecl(top) => {
                let result = self.check_top_decl(&top);
            }
            _ => panic!("type-lang linter issue, expr not implemented"),
        }
    }
    pub fn check_top_decl(&mut self, td: &TopDecl) -> ResultTreeType {
        let result = self.lint_recurse(&td.expr)?;

        let mut init = Initialization {
            left: td.identifier.into_symbol(),
            right: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::F64(_) => unop.curried = Type::F64,
            TypeTree::U64(_) => unop.curried = Type::I64,
            TypeTree::U32(_) => unop.curried = Type::I32,
            TypeTree::I64(_) => unop.curried = Type::I64,
            TypeTree::I32(_) => unop.curried = Type::I32,
            _ => {
                let mut err = make_error("invalid negation".to_string());
                self.update_error(
                    &mut err,
                    format!("found {}", unop.val.whatami()),
                    un.op.clone(),
                );
                self.issues.push(err);
            }
        }
        let curried = unop.curried.clone();
        return Ok((Box::new(TypeTree::Negate(unop)), curried));
    }

    pub fn check_negate(&mut self, un: &UnOp) -> ResultTreeType {
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
            _ => {
                let mut err = make_error("invalid negation".to_string());
                self.update_error(
                    &mut err,
                    format!("found {}", unop.val.whatami()),
                    un.op.clone(),
                );
                self.issues.push(err);
            }
        }
        let curried = unop.curried.clone();
        return Ok((Box::new(TypeTree::Negate(unop)), curried));
    }

    pub fn check_copy(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let curried = unop.curried.clone();
        return Ok((Box::new(TypeTree::Copy(unop)), curried));
    }

    pub fn check_clone(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let curried = unop.curried.clone();
        return Ok((Box::new(TypeTree::Not(unop)), curried));
    }

    pub fn check_not(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let curried = unop.curried.clone();
        return Ok((Box::new(TypeTree::Not(unop)), curried));
    }
    pub fn check_borrow_mut(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::SymbolAccess(sym) => {
                unop.curried = Type::MutBorrow(Box::new(sym.curried.clone()))
            }
            TypeTree::SelfRef(sym) => unop.curried = Type::MutBorrow(Box::new(sym.curried.clone())),
            _ => panic!("borrow_check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(MutBorrow, unop, curried);
    }

    pub fn check_borrow_ro(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref() {
            TypeTree::SymbolAccess(sym) => {
                unop.curried = Type::ReadBorrow(Box::new(sym.curried.clone()))
            }
            _ => panic!("borrow_check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(ReadBorrow, unop, curried);
    }

    pub fn check_f64(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<f64>().unwrap();
        let typ = Type::F64;
        return ok_tree!(F64, val, typ);
    }

    pub fn check_u64(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<u64>().unwrap();
        let typ = Type::U64;
        return ok_tree!(U64, val, typ);
    }

    pub fn type_check(&mut self, start: &Expr) -> () {
        let mut vals: Vec<TypeTree> = vec![];
        match start {
            Expr::FileAll(all) => {
                for x in &all.top_decls {
                    let res = self.lint_recurse(&x);
                    if res.is_ok() {
                        vals.push(*res.unwrap().0);
                    }
                }
            }
            _ => panic!("type-lang linter issue expected all at type_check"),
        }
    }

    fn update_error(&self, err: &mut LinterError, suggestion: String, lexeme: Lexeme) -> () {
        let xcl = CodeLocation::new(self.buffer, lexeme);
        let lep = LinterErrorPoint::new(xcl.code, xcl.line, xcl.col);
        err.add_point(lep, suggestion);
    }
}

pub fn make_error(title: String) -> LinterError {
    LinterError::new(title)
}
