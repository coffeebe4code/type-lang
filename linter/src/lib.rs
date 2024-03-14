use std::rc::Rc;

use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use symtable::*;
use token::Token;
use types::*;

type ResultTreeType = Result<(Rc<Box<TypeTree>>, Type), usize>;

pub struct LintSource<'buf, 'sym> {
    buffer: &'buf str,
    slt: &'sym mut SymTable,
    pub issues: Vec<LinterError>,
}

impl<'buf, 'sym> LintSource<'buf, 'sym> {
    pub fn new(buffer: &'buf str, slt: &'sym mut SymTable) -> Self {
        LintSource {
            buffer,
            slt,
            issues: vec![],
        }
    }

    pub fn lint_recurse(&mut self, to_cmp: &Expr) -> ResultTreeType {
        match to_cmp {
            Expr::InnerDecl(decl) => self.check_inner_decl(&decl),
            Expr::UndefinedValue(_) => self.check_undefined(),
            //Expr::TagDecl(decl) => self.check_tag_decl(&decl),
            Expr::UnOp(un) => match un.op.token {
                Token::Sub => self.check_negate(un),
                Token::NotLog => self.check_not(un),
                Token::And => self.check_borrow_ro(un),
                Token::Mul => self.check_borrow_mut(un),
                Token::WCopy => self.check_copy(un),
                Token::WClone => self.check_clone(un),
                _ => panic!("type-lang linter issue, unary op not implemented"),
            },
            Expr::BinOp(bin) => match bin.op.token {
                Token::Plus => self.check_plus(bin),
                _ => panic!("type-lang linter issue, binary op not implemented"),
            },
            Expr::Number(num) => match num.val.token {
                Token::Decimal => self.check_f64(num),
                Token::Num => self.check_u64(num),
                _ => panic!("type-lang linter issue, number not implemented"),
            },
            Expr::TopDecl(top) => self.check_top_decl(&top),
            Expr::Symbol(symbol) => self.check_symbol_ref(&symbol),
            Expr::Block(blk) => self.check_block(&blk),
            Expr::FuncDecl(fun) => self.check_func_decl(&fun),
            Expr::RetOp(ret) => self.check_ret_op(&ret),
            _ => panic!("type-lang linter issue, expr not implemented {:?}", to_cmp),
        }
    }

    pub fn check_func_decl(&mut self, td: &FuncDecl) -> ResultTreeType {
        let result = self.lint_recurse(&td.block)?;
        let slice = td.identifier.into_symbol().val.slice;
        let copy = slice.clone();

        let init = FunctionInitialize {
            name: slice,
            args: vec![],
            args_curried: vec![],
            block: result.0,
            block_curried: result.1,
        };
        let curried = init.block_curried.clone();
        let full = Rc::new(Box::new(TypeTree::FuncInit(init)));

        self.slt.table.insert(copy, (Rc::clone(&full), 0));
        return Ok((full, curried));
    }

    pub fn check_block(&mut self, td: &ast::Block) -> ResultTreeType {
        let result: Vec<ResultTreeType> = td.exprs.iter().map(|e| self.lint_recurse(&e)).collect();
        let mut blk = types::Block {
            exprs: vec![],
            curried: Type::Void,
        };
        let mut typ = Type::Void;
        result.into_iter().for_each(|res| {
            if let Ok(exp) = res {
                blk.exprs.push(exp.0);
                typ = exp.1;
            } else {
                typ = Type::Void;
            }
        });

        let curried = blk.curried.clone();
        let full = Rc::new(Box::new(TypeTree::Block(blk)));

        return Ok((full, curried));
    }

    pub fn check_undefined(&mut self) -> ResultTreeType {
        let typ = Type::Undefined;
        return ok_simple_tree!(UndefinedValue, typ);
    }

    pub fn check_symbol_ref(&mut self, symbol: &Symbol) -> ResultTreeType {
        let sym = SymbolAccess {
            ident: symbol.val.slice.clone(),
            curried: Type::Void,
        };
        let typ = Type::Void;
        return ok_tree!(SymbolAccess, sym, typ);
    }

    //pub fn check_tag_decl(&mut self, inner: &TagDecl) -> ResultTreeType {
    //    let slice = inner.identifier.into_symbol().val.slice;
    //    let copy = slice.clone();

    //    let tag = TagInfo {
    //        name: slice,
    //        props: inner.declarators,
    //        right: result.0,
    //        curried: result.1,
    //    };
    //    let curried = init.curried.clone();
    //    let full = tree!(ConstInit, tag);

    //    self.slt.table.insert(copy, (Rc::clone(&full), 0));
    //    return Ok((full, curried));
    //}

    pub fn check_inner_decl(&mut self, inner: &InnerDecl) -> ResultTreeType {
        let result = self.lint_recurse(&inner.expr)?;
        let slice = inner.identifier.into_symbol().val.slice;
        let copy = slice.clone();

        let init = Initialization {
            left: slice,
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        let full = Rc::new(Box::new(TypeTree::ConstInit(init)));

        self.slt.table.insert(copy, (Rc::clone(&full), 0));
        return Ok((full, curried));
    }

    pub fn check_top_decl(&mut self, td: &TopDecl) -> ResultTreeType {
        let result = self.lint_recurse(&td.expr)?;
        let slice = td.identifier.into_symbol().val.slice;
        let copy = slice.clone();

        let init = Initialization {
            left: slice,
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        let full = Rc::new(Box::new(TypeTree::ConstInit(init)));

        self.slt.table.insert(copy, (Rc::clone(&full), 0));
        return Ok((full, curried));
    }

    pub fn check_negate(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::F64(_) => unop.curried = Type::F64,
            TypeTree::U64(_) => unop.curried = Type::I64,
            TypeTree::U32(_) => unop.curried = Type::I32,
            TypeTree::I64(_) => unop.curried = Type::I64,
            TypeTree::I32(_) => unop.curried = Type::I32,
            _ => {
                let mut err = make_error("invalid negation".to_string());
                self.update_error(
                    &mut err,
                    format!(
                        "found type {}, expected negatable value",
                        unop.val.whatami()
                    ),
                    un.op.clone(),
                );
                println!("{}", err);
                self.issues.push(err);
            }
        }
        let curried = unop.curried.clone();
        return ok_tree!(Negate, unop, curried);
    }

    pub fn check_copy(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("copy checked failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(Copy, unop, curried);
    }

    pub fn check_clone(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("clone check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(Clone, unop, curried);
    }

    pub fn check_ret_op(&mut self, ret: &RetOp) -> ResultTreeType {
        let result = self.lint_recurse(&ret.expr)?;
        let unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };

        let curried = unop.curried.clone();
        return ok_tree!(Return, unop, curried);
    }

    pub fn check_not(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Type::Bool,
            _ => panic!("not check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(Not, unop, curried);
    }

    pub fn check_plus(&mut self, bin: &BinOp) -> ResultTreeType {
        let left = self.lint_recurse(&bin.left)?;
        let right = self.lint_recurse(&bin.right)?;
        let binop = BinaryOp {
            left: left.0,
            right: right.0,
            curried: left.1,
        };
        let curried = binop.curried.clone();

        return ok_tree!(Plus, binop, curried);
    }
    pub fn check_borrow_mut(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
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
        match unop.val.as_ref().as_ref() {
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
        let mut vals: Vec<Rc<Box<TypeTree>>> = vec![];
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
