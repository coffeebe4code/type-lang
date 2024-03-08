use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use symtable::*;
use token::Token;
use types::*;

type ResultTreeType = Result<(usize, Type), usize>;

pub struct LintSource<'tt, 'buf, 'sym> {
    buffer: &'buf str,
    slt: &'sym mut SymTable<'tt>,
    trees: &'sym mut Vec<TypeTree<'tt>>,
    pub issues: Vec<LinterError>,
}

impl<'tt, 'buf, 'sym> LintSource<'tt, 'buf, 'sym> {
    pub fn new(
        buffer: &'buf str,
        slt: &'sym mut SymTable<'tt>,
        trees: &'sym mut Vec<TypeTree<'tt>>,
    ) -> Self {
        LintSource {
            buffer,
            slt,
            trees,
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
            Expr::TopDecl(top) => self.check_top_decl(&top),
            _ => panic!("type-lang linter issue, expr not implemented"),
        }
    }
    pub fn check_top_decl(&mut self, td: &TopDecl) -> ResultTreeType {
        let result = self.lint_recurse(&td.expr)?;
        let slice = td.identifier.into_symbol().val.slice;
        let copy = slice.clone();

        let init = Initialization {
            left: slice,
            right: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        let curried = init.curried.clone();
        self.trees.push(tree!(ConstInit, init));

        self.slt.table.insert(copy, &self.trees.last().unwrap());
        return Ok((self.trees.len() - 1, curried));
    }

    pub fn check_negate(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
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
        let typ = unop.curried.clone();
        self.trees.push(tree!(Negate, unop));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_copy(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let typ = unop.curried.clone();
        self.trees.push(tree!(Copy, unop));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_clone(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let typ = unop.curried.clone();
        self.trees.push(tree!(Clone, unop));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_not(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("negate_check failed"),
        }
        let typ = unop.curried.clone();
        self.trees.push(tree!(Not, unop));
        return Ok((&self.trees.len() - 1, typ));
    }
    pub fn check_borrow_mut(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
            TypeTree::SymbolAccess(sym) => {
                unop.curried = Type::MutBorrow(Box::new(sym.curried.clone()))
            }
            TypeTree::SelfRef(sym) => unop.curried = Type::MutBorrow(Box::new(sym.curried.clone())),
            _ => panic!("borrow_check failed"),
        }
        let typ = unop.curried.clone();
        self.trees.push(tree!(MutBorrow, unop));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_borrow_ro(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: &self.trees.get(result.0).unwrap(),
            curried: result.1,
        };
        match unop.val {
            TypeTree::SymbolAccess(sym) => {
                unop.curried = Type::ReadBorrow(Box::new(sym.curried.clone()))
            }
            _ => panic!("borrow_check failed"),
        }
        let typ = unop.curried.clone();
        self.trees.push(tree!(ReadBorrow, unop));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_f64(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<f64>().unwrap();
        let typ = Type::F64;
        self.trees.push(tree!(F64, val));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn check_u64(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<u64>().unwrap();
        let typ = Type::U64;
        self.trees.push(tree!(U64, val));
        return Ok((&self.trees.len() - 1, typ));
    }

    pub fn type_check(&mut self, start: &Expr) -> () {
        let mut vals: Vec<usize> = vec![];
        match start {
            Expr::FileAll(all) => {
                for x in &all.top_decls {
                    let res = self.lint_recurse(&x);
                    if res.is_ok() {
                        vals.push(res.unwrap().0);
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
