use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use scopetable::ScopeTable;
use std::rc::Rc;
use token::Token;
use types::*;
use typetable::*;

type ResultTreeType = Result<(Rc<Box<TypeTree>>, Ty), usize>;

pub struct LintSource<'buf, 'ttb, 'sco> {
    buffer: &'buf str,
    idx: usize,
    curr_scope: usize,
    pub scopes: &'sco mut Vec<ScopeTable>,
    pub ttbls: &'ttb mut Vec<TypeTable>,
    pub issues: Vec<LinterError>,
}

impl<'buf, 'ttb, 'sco> LintSource<'buf, 'ttb, 'sco> {
    pub fn new(
        buffer: &'buf str,
        scopes: &'sco mut Vec<ScopeTable>,
        ttbls: &'ttb mut Vec<TypeTable>,
    ) -> Self {
        ttbls.push(TypeTable::new());
        scopes.push(ScopeTable::new(0, 0));
        LintSource {
            buffer,
            idx: 0,
            curr_scope: 0,
            scopes,
            ttbls,
            issues: vec![],
        }
    }

    pub fn lint_recurse(&mut self, to_cmp: &Expr) -> ResultTreeType {
        match to_cmp {
            Expr::InnerDecl(decl) => self.check_inner_decl(&decl),
            Expr::Import(import) => self.check_import(&import),
            Expr::TagDecl(decl) => self.check_tag_decl(&decl),
            Expr::Sig(sig) => self.check_sig(&sig),
            Expr::ValueType(vt) => self.check_value_type(&vt),
            Expr::PropAssignments(props) => self.check_props_init(&props),
            Expr::PropAssignment(prop) => self.check_prop_init(&prop),
            Expr::StructDecl(decl) => self.check_struct_decl(&decl),
            Expr::Reassignment(reas) => self.check_reassignment(&reas),
            Expr::SelfValue(_) => self.check_self_value(),
            Expr::CharsValue(chars) => self.check_chars_value(&chars),
            Expr::ErrorDecl(decl) => self.check_error_decl(&decl),
            Expr::ArrayDecl(decl) => self.check_array_decl(&decl),
            Expr::AnonFuncDecl(decl) => self.check_anon_func(&decl),
            Expr::Declarator(declarator) => self.check_declarator(&declarator),
            Expr::Match(_match) => self.check_match(&_match),
            Expr::For(_for) => self.check_for(&_for),
            Expr::If(_if) => self.check_if(&_if),
            Expr::Invoke(invoke) => self.check_invoke(&invoke),
            Expr::PropAccess(prop) => self.check_prop_access(&prop),
            Expr::Arm(arm) => self.check_arm(&arm),
            Expr::Rest(_) => self.check_rest(),
            Expr::UndefinedValue(_) => self.check_undefined(),
            Expr::UnOp(un) => match un.op.token {
                Token::Dash => self.check_negate(un),
                Token::Exclam => self.check_not(un),
                Token::Ampersand => self.check_borrow_ro(un),
                Token::Asterisk => self.check_borrow_mut(un),
                Token::Copy => self.check_copy(un),
                Token::Clone => self.check_clone(un),
                _ => panic!("type-lang linter issue, unary op not implemented"),
            },
            Expr::BinOp(bin) => match bin.op.token {
                Token::Plus => self.check_plus(&bin),
                Token::Dash => self.check_minus(&bin),
                Token::Equality => self.check_equality(&bin),
                Token::Asterisk => self.check_mul(&bin),
                _ => panic!(
                    "type-lang linter issue, binary op not implemented {:?}",
                    bin
                ),
            },
            Expr::Number(num) => match num.val.token {
                Token::Decimal => self.check_dec(num),
                Token::Number => self.check_num(num),
                _ => panic!("type-lang linter issue, number not implemented"),
            },
            Expr::TopDecl(top) => self.check_top_decl(&top),
            Expr::Symbol(symbol) => self.check_symbol_ref(&symbol),
            Expr::SymbolDecl(symbol) => self.check_symbol_decl(&symbol),
            Expr::Block(blk) => self.check_block(&blk),
            Expr::FuncDecl(fun) => self.check_func_decl(&fun),
            Expr::RetOp(ret) => self.check_ret_op(&ret),
            Expr::ArgDef(arg) => self.check_arg_def(&arg),
            _ => panic!("type-lang linter issue, expr not implemented {:?}", to_cmp),
        }
    }

    pub fn check_func_decl(&mut self, td: &FuncDecl) -> ResultTreeType {
        let mut largs = vec![];
        let mut largs_curried = vec![];
        if let Some(args) = td.args.as_ref() {
            args.iter().for_each(|x| {
                let res = self.lint_recurse(x);
                if let Ok(a) = res {
                    largs.push(a.0);
                    largs_curried.push(a.1);
                    return;
                }
                largs.push(simple_tree!(UnknownValue));
                largs_curried.push(Ty::Unknown);
            });
        }
        let result = self.lint_recurse(&td.block)?;
        let slice = td.identifier.into_symbol().val.slice;

        let init = FunctionInitialize {
            name: slice.clone(),
            args: largs,
            args_curried: largs_curried,
            block: result.0,
            block_curried: result.1,
        };
        let curried = init.block_curried.clone();
        let full = tree!(FuncInit, init);
        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        Ok((full, curried))
    }

    pub fn check_block(&mut self, td: &ast::Block) -> ResultTreeType {
        let result: Vec<ResultTreeType> = td.exprs.iter().map(|e| self.lint_recurse(&e)).collect();
        let mut blk = types::Block {
            exprs: vec![],
            curried: Ty::Unknown,
        };
        result.into_iter().for_each(|res| {
            if let Ok(exp) = res {
                blk.exprs.push(exp.0);
            }
        });

        // todo:: get the last one if ret, curry, if not void
        if blk.exprs.last().is_some() {}
        let curried = blk.curried.clone();
        ok_tree!(Block, blk, curried)
    }

    pub fn check_undefined(&mut self) -> ResultTreeType {
        let typ = Ty::Undefined;
        ok_simple_tree!(UndefinedValue, typ)
    }

    pub fn check_if(&mut self, _if: &If) -> ResultTreeType {
        let res = self.lint_recurse(&_if.expr)?;
        let body = self.lint_recurse(&_if.body)?;
        let if_op = IfOp {
            in_expr: res.0,
            in_curried: res.1,
            body: body.0,
            body_curried: body.1,
        };
        let cur = if_op.body_curried.clone();
        ok_tree!(If, if_op, cur)
    }

    pub fn check_for(&mut self, _for: &For) -> ResultTreeType {
        let res = self.lint_recurse(&_for.expr)?;
        let body = self.lint_recurse(&_for.var_loop)?;
        let for_op = ForOp {
            in_expr: res.0,
            in_curried: res.1,
            body: body.0,
            body_curried: body.1,
        };
        let cur = for_op.body_curried.clone();
        ok_tree!(For, for_op, cur)
    }

    pub fn check_match(&mut self, _match: &Match) -> ResultTreeType {
        let res = self.lint_recurse(&_match.expr)?;
        let mut mat = MatchOp {
            expr: res.0,
            curried: res.1,
            arms: vec![],
            curried_arms: Ty::Tag(vec![]),
        };
        _match.arms.iter().for_each(|m| {
            let mres = self.lint_recurse(m);
            if let Ok(arm) = mres {
                mat.arms.push(arm.0);
                mat.curried_arms.into_vec().push(arm.1);
                return;
            }
            mat.arms.push(simple_tree!(UnknownValue));
            mat.curried_arms.into_vec().push(Ty::Unknown);
        });
        let cur = mat.curried.clone();
        return ok_tree!(Match, mat, cur);
    }

    pub fn check_declarator(&mut self, decl: &Declarator) -> ResultTreeType {
        let dec = DeclaratorInfo {
            name: decl.ident.into_symbol().val.slice.clone(),
            curried: Ty::Undefined,
        };
        let curried = dec.curried.clone();
        return ok_tree!(DeclaratorInfo, dec, curried);
    }

    pub fn check_symbol_decl(&mut self, symbol: &Symbol) -> ResultTreeType {
        let sym = SymbolInit {
            ident: symbol.val.slice.clone(),
            curried: Ty::Unknown,
        };
        let curried = sym.curried.clone();
        let full = tree!(SymbolInit, sym);

        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(symbol.val.slice.clone(), Rc::clone(&full));
        return Ok((full, curried));
    }

    pub fn check_symbol_ref(&mut self, symbol: &Symbol) -> ResultTreeType {
        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();
        let sym = SymbolAccess {
            ident: symbol.val.slice.clone(),
            curried: tbl
                .table
                .get(&symbol.val.slice)
                .unwrap()
                .get_curried()
                .clone(),
        };
        let curried = sym.curried.clone();
        return ok_tree!(SymbolAccess, sym, curried);
    }

    pub fn check_array_decl(&mut self, arr: &ArrayDecl) -> ResultTreeType {
        let mut array = ArrayInitialize {
            vals: vec![],
            vals_curried: vec![],
            curried: Ty::Unknown,
        };
        if let Some(args) = &arr.args {
            args.into_iter().for_each(|e| {
                if let Ok(r) = self.lint_recurse(&e) {
                    array.vals.push(r.0);
                } else {
                    array.vals.push(simple_tree!(UnknownValue));
                    array.vals_curried.push(Ty::Unknown);
                    array.curried = Ty::Unknown;
                }
            });
        }

        let curried = array.curried.clone();
        return ok_tree!(ArrayInit, array, curried);
    }

    pub fn check_error_decl(&mut self, err: &ErrorDecl) -> ResultTreeType {
        let slice = err.identifier.into_symbol().val.slice;
        let err_info = ErrorInfo {
            message: "".to_string(),
            code: 0,
            curried: Ty::Error,
        };

        let curried = err_info.curried.clone();
        let full = tree!(ErrorInfo, err_info);

        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));

        return Ok((full, curried));
    }

    pub fn check_value_type(&mut self, _vt: &ValueType) -> ResultTreeType {
        let mut curried = Ty::Unknown;
        match _vt.val.token {
            Token::U64 => curried = Ty::U64,
            Token::USize => curried = Ty::USize,
            Token::F64 => curried = Ty::F64,
            _ => panic!("type lang issue, unmatched value type"),
        };
        let copied = curried.clone();
        let full = tree!(ValueType, copied);
        return Ok((full, curried));
    }

    pub fn check_sig(&mut self, _sig: &Sig) -> ResultTreeType {
        let mut sig_info = SigTypes {
            left: Ty::Unknown,
            err: Some(Ty::Unknown),
            undefined: Some(Ty::Unknown),
            right: Some(Ty::Unknown),
        };
        let mut c_right: Option<Ty> = None;
        let mut c_left: Ty = Ty::Unknown;
        let mut c_err: Option<Ty> = None;
        let mut c_undefined: Option<Ty> = None;
        let mut curried = Ty::Unknown;

        if let Some(right) = &_sig.right_most_type {
            c_right = match self.lint_recurse(&right) {
                Err(_) => Some(Ty::Unknown),
                Ok(v) => Some(v.1),
            }
        }
        if let Some(left) = &_sig.left_most_type {
            c_left = match self.lint_recurse(&left) {
                Err(_) => Ty::Unknown,
                Ok(v) => v.1,
            }
        }
        if let Some(_) = &_sig.err {
            c_err = Some(Ty::Error);
        }
        if let Some(_) = &_sig.undef {
            c_undefined = Some(Ty::Undefined);
        }
        if c_right.is_some() || c_err.is_some() || c_undefined.is_some() {
            sig_info.left = c_left;
            sig_info.err = c_err;
            sig_info.undefined = c_undefined;
            sig_info.right = c_right;
            let full = tree!(SigTypes, sig_info);

            return Ok((full, curried));
        }

        let full = tree!(SingleType, c_left);
        return Ok((full, curried));
    }

    pub fn check_self_value(&mut self) -> ResultTreeType {
        let curr_self = self
            .curr_self
            .as_ref()
            .expect("expected self to be defined");
        let self_ref = NoOp {
            curried: curr_self.clone(),
        };
        let curried = self_ref.curried.clone();
        ok_tree!(SelfAccess, self_ref, curried)
    }

    pub fn check_chars_value(&mut self, chars: &ast::CharsValue) -> ResultTreeType {
        let mut vals: Vec<Rc<Box<TypeTree>>> = vec![];
        let mut vals_curried: Vec<Ty> = vec![];
        chars.val.slice.chars().for_each(|x| {
            vals.push(tree!(Char, x));
            vals_curried.push(Ty::Char);
        });

        let chars_result = ArrayInitialize {
            vals,
            vals_curried,
            curried: Ty::String,
        };
        let curried = chars_result.curried.clone();
        ok_tree!(StringInit, chars_result, curried)
    }

    pub fn check_reassignment(&mut self, reas: &ast::Reassignment) -> ResultTreeType {
        let maybe_access = self.lint_recurse(&reas.left)?;
        let result = self.lint_recurse(&reas.expr)?;
        let reassignment = types::Reassignment {
            left: maybe_access.0,
            right: result.0,
            curried: maybe_access.1,
        };
        // assert left type == right type or elidable

        // need to ensure constness is checked on the property
        reassignment.left.get_curried().ensure_mut().or_else(|x| {
            Err(self.set_error(
                format!("found {}", x),
                format!("{} is immutable, did you mean to declare with let?", x),
                reas.left.into_symbol().val,
            ))
        })?;
        let curried = reassignment.curried.clone();
        return ok_tree!(As, reassignment, curried);
    }

    pub fn check_struct_decl(&mut self, obj: &StructDecl) -> ResultTreeType {
        if let Some(x) = &obj.declarators {
            let result: Vec<ResultTreeType> =
                x.into_iter().map(|e| self.lint_recurse(&e)).collect();
            let slice = obj.identifier.into_symbol().val.slice;
            let mut obj_info = StructInfo {
                props: vec![],
                types: vec![],
                curried: Ty::Custom(slice.clone()),
            };
            result.into_iter().for_each(|res| {
                if let Ok(exp) = res {
                    obj_info.props.push(exp.0.into_declarator().name.clone());
                    obj_info.types.push(exp.1);
                    return;
                }
                obj_info
                    .props
                    .push(res.unwrap().0.into_declarator().name.clone());
                obj_info.types.push(Ty::Unknown);
            });

            let curried = obj_info.curried.clone();
            let full = tree!(StructInfo, obj_info);

            let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

            tbl.table.insert(slice, Rc::clone(&full));
            return Ok((full, curried));
        }
        Err(self.set_error(
            "expected at least one declarator".to_string(),
            format!("found empty {{}}, expected declarator"),
            obj.identifier.into_symbol().val,
        ))
    }

    pub fn check_prop_init(&mut self, prop: &PropAssignment) -> ResultTreeType {
        let result = self.lint_recurse(&prop.val)?;
        let slice = prop.ident.into_symbol().val.slice.clone();
        let init = Initialization {
            left: slice.clone(),
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        let full = tree!(PropInit, init);

        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        return Ok((full, curried));
    }

    pub fn check_props_init(&mut self, props: &PropAssignments) -> ResultTreeType {
        let prev = self.lint_recurse(&props.prev)?;
        if let Some(p) = &props.props {
            let result: Vec<ResultTreeType> =
                p.into_iter().map(|e| self.lint_recurse(&e)).collect();
            let mut struct_init = StructInitialize {
                idents: vec![],
                vals: vec![],
                vals_curried: vec![],
                curried: prev.0.into_symbol_access().curried.clone(),
            };
            result.into_iter().for_each(|res| {
                if let Ok(x) = res {
                    struct_init.idents.push(x.0.into_prop_init().left.clone());
                    struct_init
                        .vals_curried
                        .push(x.0.into_prop_init().curried.clone());
                } else {
                    struct_init.idents.push("unknown".to_string());
                    struct_init.vals_curried.push(Ty::Unknown);
                }
            });

            let curried = struct_init.curried.clone();
            let full = tree!(StructInit, struct_init);

            let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

            tbl.table
                .insert(prev.0.into_symbol_access().ident.clone(), Rc::clone(&full));
            return Ok((full, curried));
        }
        Err(self.set_error(
            "expected at least one property".to_string(),
            format!("found empty {{}}, expected property"),
            props.prev.into_symbol().val,
        ))
    }

    pub fn check_tag_decl(&mut self, tag: &TagDecl) -> ResultTreeType {
        let result: Vec<ResultTreeType> = tag
            .declarators
            .iter()
            .map(|e| self.lint_recurse(&e))
            .collect();
        let slice = tag.identifier.into_symbol().val.slice;
        let copy = slice.clone();
        let mut tag_info = TagInfo {
            name: slice,
            props: vec![],
            types: vec![],
            curried: Ty::Custom(copy.clone()),
        };
        result.into_iter().for_each(|res| {
            if let Ok(exp) = res {
                tag_info.props.push(exp.0);
                tag_info.types.push(exp.1);
                return;
            }
            tag_info.props.push(simple_tree!(UnknownValue));
            tag_info.types.push(Ty::Unknown);
        });

        let curried = tag_info.curried.clone();
        let full = tree!(TagInfo, tag_info);

        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(copy, Rc::clone(&full));
        return Ok((full, curried));
    }

    pub fn check_anon_func(&mut self, anon: &AnonFuncDecl) -> ResultTreeType {
        let mut largs = vec![];
        let mut largs_curried = vec![];
        if let Some(args) = anon.args.as_ref() {
            args.iter().for_each(|x| {
                let res = self.lint_recurse(x);
                if let Ok(a) = res {
                    largs.push(a.0);
                    largs_curried.push(a.1);
                    return;
                }
                largs.push(simple_tree!(UnknownValue));
                largs_curried.push(Ty::Unknown);
            });
        }
        let result = self.lint_recurse(&anon.block)?;
        let slice = format!(":anon_{}", self.idx);
        self.idx += 1;

        let init = FunctionInitialize {
            name: slice.clone(),
            args: largs,
            args_curried: largs_curried,
            block: result.0,
            block_curried: result.1,
        };
        let curried = init.block_curried.clone();
        let full = tree!(FuncInit, init);

        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        Ok((full, curried))
    }

    pub fn check_import(&mut self, import: &Import) -> ResultTreeType {
        let result = self.lint_recurse(&import.expr)?;
        let slice = import.expr.into_chars_value().val.slice;

        let init = Initialization {
            left: slice.clone(),
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        if import.mutability.token == Token::Const {
            let full: Rc<Box<TypeTree>> = tree!(ConstInit, init);
            let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

            tbl.table.insert(slice, Rc::clone(&full));
            return Ok((full, Ty::Const(Box::new(curried))));
        }
        let full: Rc<Box<TypeTree>> = tree!(MutInit, init);
        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        return Ok((full, Ty::Mut(Box::new(curried))));
    }

    pub fn check_inner_decl(&mut self, inner: &InnerDecl) -> ResultTreeType {
        let result = self.lint_recurse(&inner.expr)?;
        let slice = inner.identifier.into_symbol().val.slice;

        let mut init = Initialization {
            left: slice.clone(),
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        if inner.mutability.token == Token::Const {
            init.curried = Ty::Const(Box::new(init.curried));
            let full: Rc<Box<TypeTree>> = tree!(ConstInit, init);
            let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

            tbl.table.insert(slice, Rc::clone(&full));
            return Ok((full, Ty::Const(Box::new(curried))));
        }
        init.curried = Ty::Mut(Box::new(init.curried));
        let full: Rc<Box<TypeTree>> = tree!(MutInit, init);
        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        return Ok((full, Ty::Mut(Box::new(curried))));
    }

    pub fn check_top_decl(&mut self, td: &TopDecl) -> ResultTreeType {
        let result = self.lint_recurse(&td.expr)?;
        let slice = td.identifier.into_symbol().val.slice;

        let init = Initialization {
            left: slice.clone(),
            right: result.0,
            curried: result.1,
        };
        let curried = init.curried.clone();
        if td.mutability.token == Token::Const {
            let full: Rc<Box<TypeTree>> = tree!(ConstInit, init);
            let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

            tbl.table.insert(slice, Rc::clone(&full));
            return Ok((full, Ty::Const(Box::new(curried))));
        }
        let full: Rc<Box<TypeTree>> = tree!(MutInit, init);
        let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

        tbl.table.insert(slice, Rc::clone(&full));
        return Ok((full, Ty::Mut(Box::new(curried))));
    }

    pub fn check_negate(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::F64(_) => unop.curried = Ty::F64,
            TypeTree::U64(_) => unop.curried = Ty::I64,
            TypeTree::U32(_) => unop.curried = Ty::I32,
            TypeTree::I64(_) => unop.curried = Ty::I64,
            TypeTree::I32(_) => unop.curried = Ty::I32,
            TypeTree::Negate(_) => {
                return Err(self.set_error(
                    "invalid negation".to_string(),
                    "double negation superfluous. decrement must be done with (val - 1)"
                        .to_string(),
                    un.op.clone(),
                ));
            }
            _ => {
                return Err(self.set_error(
                    "invalid negation".to_string(),
                    format!(
                        "found type {}, expected negatable value",
                        unop.val.whatami()
                    ),
                    un.op.clone(),
                ));
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
            TypeTree::BoolValue(_) => unop.curried = Ty::Bool,
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
            TypeTree::BoolValue(_) => unop.curried = Ty::Bool,
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

    pub fn check_arg_def(&mut self, arg: &ArgDef) -> ResultTreeType {
        match arg.ident.as_ref() {
            Expr::SymbolDecl(x) => {
                let slice = x.val.slice.clone();
                let typ = self.lint_recurse(&arg.typ)?;
                let a = SymbolInit {
                    ident: slice.clone(),
                    curried: typ.1,
                };

                let curried = a.curried.clone();
                let full: Rc<Box<TypeTree>> = tree!(ArgInit, a);
                let tbl = self.ttbls.get_mut(self.curr_scope).unwrap();

                tbl.table.insert(slice, Rc::clone(&full));

                return Ok((full, curried));
            }
            Expr::SelfDecl(_) => {
                let typ = self.lint_recurse(&arg.typ)?;
                let a = NoOp { curried: typ.1 };

                let curried = a.curried.clone();
                self.curr_self = Some(a.curried.clone());

                let full: Rc<Box<TypeTree>> = tree!(SelfAccess, a);

                return Ok((full, curried));
            }
            _ => panic!("unexpected expression in arg_def"),
        }
    }

    pub fn check_not(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::BoolValue(_) => unop.curried = Ty::Bool,
            _ => panic!("not check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(Not, unop, curried);
    }

    pub fn check_prop_access(&mut self, prop: &ast::PropAccess) -> ResultTreeType {
        let prev = self.lint_recurse(&prop.prev)?;
        let access = types::PropAccess {
            prev: prev.0,
            ident: prop.identifier.into_symbol().val.slice,
            curried: prev.1,
        };
        let curried = access.curried.clone();

        return ok_tree!(PropAccess, access, curried);
    }

    pub fn check_invoke(&mut self, inv: &ast::Invoke) -> ResultTreeType {
        let prev = self.lint_recurse(&inv.prev)?;
        let mut invoke = types::Invoke {
            args: vec![],
            args_curried: vec![],
            ident: prev.0,
            curried: prev.1,
        };
        if let Some(args) = &inv.args {
            args.iter().for_each(|a| {
                if let Ok(prev) = self.lint_recurse(&a) {
                    invoke.args.push(prev.0);
                    invoke.args_curried.push(prev.1);
                    return;
                }
                invoke.args.push(simple_tree!(UnknownValue));
                invoke.args_curried.push(Ty::Unknown);
            })
        };
        let curried = invoke.curried.clone();

        return ok_tree!(Invoke, invoke, curried);
    }

    pub fn check_mul(&mut self, bin: &BinOp) -> ResultTreeType {
        let left = self.lint_recurse(&bin.left)?;
        let right = self.lint_recurse(&bin.right)?;
        let binop = BinaryOp {
            left: left.0,
            right: right.0,
            curried: Ty::F64,
        };
        let curried = binop.curried.clone();

        ok_tree!(Multiply, binop, curried)
    }

    pub fn check_equality(&mut self, bin: &BinOp) -> ResultTreeType {
        let left = self.lint_recurse(&bin.left)?;
        let right = self.lint_recurse(&bin.right)?;
        let binop = BinaryOp {
            left: left.0,
            right: right.0,
            curried: Ty::Bool,
        };
        let curried = binop.curried.clone();

        ok_tree!(Plus, binop, curried)
    }

    pub fn check_minus(&mut self, bin: &BinOp) -> ResultTreeType {
        let left = self.lint_recurse(&bin.left)?;
        let right = self.lint_recurse(&bin.right)?;
        let binop = BinaryOp {
            left: left.0,
            right: right.0,
            curried: left.1,
        };
        let curried = binop.curried.clone();

        return ok_tree!(Minus, binop, curried);
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

    pub fn check_rest(&mut self) -> ResultTreeType {
        let restop = NoOp { curried: Ty::Rest };
        let curried = restop.curried.clone();

        return ok_tree!(RestAccess, restop, curried);
    }

    pub fn check_arm(&mut self, arm: &Arm) -> ResultTreeType {
        let left = self.lint_recurse(&arm.left)?;
        let right = self.lint_recurse(&arm.right)?;
        let binop = BinaryOp {
            left: left.0,
            right: right.0,
            curried: left.1,
        };
        let curried = binop.curried.clone();

        return ok_tree!(Arm, binop, curried);
    }

    pub fn check_borrow_mut(&mut self, un: &UnOp) -> ResultTreeType {
        let result = self.lint_recurse(&un.val)?;
        let mut unop = UnaryOp {
            val: result.0,
            curried: result.1,
        };
        match unop.val.as_ref().as_ref() {
            TypeTree::SymbolAccess(sym) => {
                unop.curried = Ty::MutBorrow(Box::new(sym.curried.clone()))
            }
            TypeTree::SelfAccess(sym) => {
                unop.curried = Ty::MutBorrow(Box::new(sym.curried.clone()))
            }
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
                unop.curried = Ty::ReadBorrow(Box::new(sym.curried.clone()))
            }
            _ => panic!("borrow_check failed"),
        }
        let curried = unop.curried.clone();
        return ok_tree!(ReadBorrow, unop, curried);
    }

    pub fn check_dec(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<f64>().unwrap();
        let typ = Ty::F64;
        return ok_tree!(F64, val, typ);
    }

    // todo:: convert this back to u64, need to check to see if it fits in i64 and return type
    pub fn check_num(&mut self, num: &Number) -> ResultTreeType {
        let val = num.val.slice.parse::<u64>().unwrap();
        let typ = Ty::U64;
        return ok_tree!(U64, val, typ);
    }

    pub fn lint_check(&mut self, start: &Expr) -> Vec<Rc<Box<TypeTree>>> {
        let mut vals: Vec<Rc<Box<TypeTree>>> = vec![];
        match start {
            Expr::FileAll(all) => {
                for x in &all.top_decls {
                    let res = self.lint_recurse(&x);
                    if res.is_ok() {
                        vals.push(res.unwrap().0);
                    }
                }
                return vals;
            }
            _ => panic!("type-lang linter issue expected all at lint_check"),
        }
    }

    fn set_error(&mut self, title: String, suggestion: String, lexeme: Lexeme) -> usize {
        let mut le = LinterError::new(title);
        let xcl = CodeLocation::new(self.buffer, lexeme);
        let lep = LinterErrorPoint::new(xcl.code, xcl.line, xcl.col);
        le.add_point(lep, suggestion);

        self.issues.push(le);
        return self.issues.len() - 1;
    }
}

pub fn make_error(title: String) -> LinterError {
    LinterError::new(title)
}

trait DoConvert {
    fn into_type(self) -> Ty;
}

impl DoConvert for &Expr {
    fn into_type(self) -> Ty {
        match self {
            Expr::Number(num) => match num.val.token {
                Token::Decimal => Ty::F64,
                // todo:: check if it fits in u64
                Token::Number => Ty::I64,
                _ => panic!("type-lang linter issue, number not implemented"),
            },
            Expr::ValueType(val) => match val.val.token {
                Token::I32 => Ty::I32,
                Token::U32 => Ty::U32,
                Token::I64 => Ty::I64,
                Token::U64 => Ty::U64,
                Token::I16 => Ty::I32,
                Token::U16 => Ty::U32,
                Token::U8 => Ty::U32,
                Token::I8 => Ty::U32,
                Token::Bit => Ty::U32,
                Token::F64 => Ty::F64,
                Token::D64 => Ty::U32,
                Token::F32 => Ty::U32,
                Token::D32 => Ty::U32,
                Token::D128 => Ty::U32,
                Token::F128 => Ty::U32,
                Token::ISize => Ty::U32,
                Token::USize => Ty::U32,
                Token::Char => Ty::Char,
                Token::Utf8 => Ty::Char,
                Token::Utf16 => Ty::Char,
                Token::Utf32 => Ty::Char,
                Token::Utf64 => Ty::Char,
                Token::Bool => Ty::Char,
                Token::Any => Ty::Custom("any".to_string()),
                Token::Sized => Ty::Custom("sized".to_string()),
                Token::Scalar => Ty::Custom("scalar".to_string()),
                Token::Void => Ty::Void,
                Token::TSelf => Ty::TSelf,
                _ => panic!("type-lang linter issue, not a value type"),
            },
            _ => panic!("type-lang linter issue, unhandled expr"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::*;
    #[test]
    fn it_should_check_double_negation() {
        let lexer = TLexer::new("8 + --5");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let mut tts = vec![];
        let mut scps = vec![];
        let mut linter = LintSource::new("8 + --5", &mut scps, &mut tts);
        let test = linter.lint_recurse(&result.unwrap());

        assert!(test.is_err_and(|x| { x == 0 }));
        assert_eq!(
            linter.issues.get(0).unwrap().points.get(0).unwrap(),
            &LinterErrorPoint {
                code: "8 + -".to_string(),
                line: 1,
                col: 5
            }
        );
    }
    #[test]
    fn it_should_tree_good() {
        const TEST_STR: &'static str = "const val = 7
            const main = fn() void { return 7 + val }
        ";
        let lexer = TLexer::new(TEST_STR);
        let mut parser = Parser::new(lexer);
        let result = parser.all();
        let mut tts = vec![];
        let mut scps = vec![];
        let mut linter = LintSource::new(TEST_STR, &mut scps, &mut tts);
        let test = linter.lint_check(&result.unwrap());
        println!("{:?}", test);

        assert!(test.len() == 5);
    }
}
