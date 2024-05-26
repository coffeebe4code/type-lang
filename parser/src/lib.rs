use ast::*;
use codelocation::*;
use lexer::Lexeme;
use lexer::TLexer;
use perror::*;
use token::Token;

pub type ResultOptExpr = Result<Option<Box<Expr>>>;
pub type ResultExpr = Result<Box<Expr>>;
pub type OptExpr = Option<Box<Expr>>;

pub struct Parser<'s> {
    lexer: TLexer<'s>,
}

impl<'s> Parser<'s> {
    // todo:: optimization: use an allocator for all expressions into a single vec, use either
    // references or id's
    pub fn new(lexer: TLexer<'s>) -> Self {
        Parser { lexer }
    }

    pub fn all(&mut self) -> ResultExpr {
        let mut tops: Vec<Box<Expr>> = vec![];
        while self.lexer.peek().is_some() {
            tops.push(self.top_decl()?);
        }
        return result_expr!(FileAll, tops);
    }

    pub fn _return(&mut self) -> ResultExpr {
        let span = self
            .lexer
            .collect_of_if(&[Token::Return, Token::Break])
            .xexpect_token(
                self,
                "expected 'break' or 'return' depending on block context".to_string(),
            )?;
        self.expr()
            .xresult_or(|expr| result_expr!(RetOp, span, expr))
    }

    pub fn _import(&mut self, mutability: Lexeme, identifier: Box<Expr>) -> ResultExpr {
        self.chars()
            .xconvert_to_result(
                self,
                "expected a string of characters with ' or \"".to_string(),
            )
            .xresult_or(|expr| result_expr!(Import, mutability, identifier, expr))
    }

    pub fn tag(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> ResultExpr {
        let mut variants: Vec<Box<Expr>> = vec![];
        while let Some(_) = self.lexer.collect_if(Token::Bar) {
            let x = self
                .ident()
                .xconvert_to_result(self, "expected identifier'".to_string())?;
            variants.push(x);
        }
        result_expr!(TagDecl, visibility, mutability, identifier, variants, sig)
    }

    pub fn _error(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> ResultExpr {
        let _ = self
            .lexer
            .collect_if(Token::OBrace)
            .xexpect_token(&self, "expected '{'".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::CBrace)
            .xexpect_token(&self, "expected '}'".to_string())?;
        result_expr!(ErrorDecl, visibility, mutability, identifier, sig)
    }

    pub fn _struct(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> ResultExpr {
        let _ = self
            .lexer
            .collect_if(Token::OBrace)
            .xexpect_token(&self, "expected '{'".to_string())?;
        let decls = self.declarators()?;
        let _ = self
            .lexer
            .collect_if(Token::CBrace)
            .xexpect_token(&self, "expected '}'".to_string())?;
        result_expr!(StructDecl, visibility, mutability, identifier, decls, sig)
    }

    pub fn top_decl(&mut self) -> ResultExpr {
        let has_pub = self.lexer.collect_if(Token::Pub);
        let mutability = self
            .lexer
            .collect_of_if(&[Token::Let, Token::Const, Token::Type, Token::Impl])
            .xexpect_token(&self, "expected mutability".to_string())?;
        let identifier = self
            .ident()
            .xexpect_expr(&self, "expected identifier".to_string())?;
        let sig = self.opt_signature()?;
        let _ = self
            .lexer
            .collect_if(Token::As)
            .xexpect_token(&self, "expected =".to_string())?;
        if let Some(val) = self.lexer.collect_of_if(&[
            Token::Struct,
            Token::Func,
            Token::Trait,
            Token::Import,
            Token::Tag,
            Token::Error,
        ]) {
            match val.token {
                Token::Struct => return self._struct(has_pub, mutability, identifier, sig),
                Token::Func => return self._fn(has_pub, mutability, identifier, sig),
                Token::Import => return self._import(mutability, identifier),
                Token::Tag => return self.tag(has_pub, mutability, identifier, sig),
                Token::Trait => return self._trait(has_pub, mutability, identifier, sig),
                Token::Error => return self._error(has_pub, mutability, identifier, sig),
                _ => panic!("type-lang error unreachable code hit"),
            }
        }
        let asgn = self.expr()?;
        result_expr!(TopDecl, has_pub, mutability, identifier, sig, asgn)
    }
    pub fn _trait(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> ResultExpr {
        if let Some(_) = self.lexer.collect_if(Token::OParen) {
            let args = self.args()?;
            let _ = self
                .lexer
                .collect_if(Token::CParen)
                .xexpect_token(&self, "expected ')'".to_string())?;
            let block = self.block()?;
            return result_expr!(
                TraitDecl,
                visibility,
                mutability,
                identifier,
                args,
                Some(block),
                sig
            );
        }
        result_expr!(TraitDecl, visibility, mutability, identifier, None, None, sig)
    }
    pub fn _fn_type(&mut self) -> ResultOptExpr {
        if let Some(_) = self.lexer.collect_if(Token::Func) {
            let _ = self
                .lexer
                .collect_if(Token::OParen)
                .xexpect_token(&self, "expected '('".to_string())?;
            let args = self.sig_union()?;
            let mut args_list: Vec<Box<Expr>> = vec![];
            if args.is_some() {
                args_list.push(args.unwrap());
            }
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                args_list.push(
                    self.sig_union()
                        .xexpect_expr(&self, "expected a type signature".to_string())?,
                );
            }
            let _ = self
                .lexer
                .collect_if(Token::CParen)
                .xexpect_token(&self, "expected one of ')' or ','".to_string())?;
            let ret_type = self
                .sig_union()
                .xexpect_expr(&self, "expected function return type".to_string())?;
            return result_expr!(FuncType, Some(args_list), ret_type).xconvert_to_result_opt();
        }
        Ok(None)
    }
    pub fn _fn(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
        sig: Option<Box<Expr>>,
    ) -> ResultExpr {
        let _ = self
            .lexer
            .collect_if(Token::OParen)
            .xexpect_token(&self, "expected '('".to_string())?;
        let args = self.args()?;
        let _ = self.lexer.collect_if(Token::CParen).xexpect_token(
            &self,
            "expected one of ')' or ',' <more arguments>".to_string(),
        )?;
        let ret_type = self
            .sig_union()
            .xexpect_expr(&self, "expected function return type".to_string())?;
        let block = self.block()?;
        result_expr!(FuncDecl, visibility, mutability, identifier, args, ret_type, block, sig)
    }
    pub fn chars(&mut self) -> OptExpr {
        self.lexer
            .collect_if(Token::Chars)
            .xconvert_expr(|span| expr!(CharsValue, span))
    }
    pub fn declarators(&mut self) -> Result<Option<Vec<Box<Expr>>>> {
        if let Some(local) = self.declarator()? {
            let mut decl_list: Vec<Box<Expr>> = vec![];
            decl_list.push(local);
            while let Some(_) = self.lexer.collect_if(Token::Comma) {
                let decl = self.declarator()?;
                if decl.is_none() {
                    break;
                }
                decl_list.push(decl.unwrap());
            }
            return Ok(Some(decl_list));
        }
        Ok(None)
    }
    pub fn declarator(&mut self) -> ResultOptExpr {
        let id = self.ident();
        if id.is_none() {
            return Ok(None);
        }
        let sig = self.opt_signature()?;
        return result_expr!(Declarator, id.unwrap(), sig).xconvert_to_result_opt();
    }
    pub fn args(&mut self) -> Result<Option<Vec<Box<Expr>>>> {
        let mut arg_list: Vec<Box<Expr>> = vec![];
        if let Ok(Some(arg_local)) = self.arg() {
            arg_list.push(arg_local);
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                arg_list.push(
                    self.arg()?
                        .xexpect_expr(&self, "expected argument".to_string())?,
                );
            }
            return Ok(Some(arg_list));
        }
        Ok(None)
    }
    pub fn array_decl(&mut self) -> ResultOptExpr {
        if let Some(_) = self.lexer.collect_if(Token::OBracket) {
            if let Some(_) = self.lexer.collect_if(Token::CBracket) {
                return result_expr!(ArrayDecl, None).xconvert_to_result_opt();
            }
            let res = self.or()?;
            let mut args: Vec<Box<Expr>> = vec![];
            args.push(res);
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                args.push(self.or()?);
            }
            let _ = self
                .lexer
                .collect_if(Token::CBracket)
                .xexpect_token(&self, "expected ']'".to_string())?;
            return result_expr!(ArrayDecl, Some(args)).xconvert_to_result_opt();
        }
        Ok(None)
    }
    pub fn arg(&mut self) -> ResultOptExpr {
        if let Some(id) = self.ident() {
            if let Some(sig) = self.opt_signature()? {
                return result_expr!(ArgDef, id, Some(sig)).xconvert_to_result_opt();
            }
            return result_expr!(ArgDef, id, None).xconvert_to_result_opt();
        }
        if let Some(id) = self._self() {
            if let Some(sig) = self.opt_signature()? {
                return result_expr!(ArgDef, id, Some(sig)).xconvert_to_result_opt();
            }
            return result_expr!(ArgDef, id, None).xconvert_to_result_opt();
        }
        Ok(None)
    }
    pub fn sig_union(&mut self) -> ResultOptExpr {
        let start = self.signature_no_colon();
        let err = self.lexer.collect_if(Token::Exclam);
        let undef = self.lexer.collect_if(Token::Question);
        let fin = self.signature_no_colon();
        if start.is_err() {
            return start;
        }
        if fin.is_err() {
            return fin;
        }
        match start? {
            None => match fin? {
                None => {
                    if err.is_none() {
                        return Err(self.make_error(
                            "expected at least one of error type, bubbled error '!', or type"
                                .to_string(),
                        ))?;
                    }
                    return result_expr!(Sig, None, err, undef, None).xconvert_to_result_opt();
                }
                Some(x) => {
                    return result_expr!(Sig, None, err, undef, Some(x)).xconvert_to_result_opt();
                }
            },
            Some(x) => {
                return result_expr!(Sig, Some(x), err, undef, fin.unwrap())
                    .xconvert_to_result_opt();
            }
        };
    }
    pub fn signature_no_colon(&mut self) -> ResultOptExpr {
        let muta = self.lexer.collect_of_if(&[
            Token::Const,
            Token::Let,
            Token::Ampersand,
            Token::Asterisk,
        ]);
        if let Some(x) = self.val_type() {
            return Ok(Some(x));
        }
        if let Some(arr) = self.arr_type()? {
            return Ok(Some(arr));
        }
        if let Some(id) = self.ident() {
            return Ok(Some(id));
        }
        if let Some(fn_typ) = self._fn_type()? {
            if muta.is_some() {
                return Err(
                    self.make_error("no mutability tokens allowed on functions".to_string())
                );
            }
            return Ok(Some(fn_typ));
        }
        Ok(None)
    }
    pub fn opt_signature(&mut self) -> ResultOptExpr {
        if let Some(_) = self.lexer.collect_if(Token::Colon) {
            return self
                .sig_union()
                .xexpect_expr(&self, "expected a type signature".to_string())
                .xconvert_to_result_opt();
        }
        Ok(None)
    }
    pub fn reassign(&mut self) -> ResultOptExpr {
        let acc = self.access()?;
        if let Some(a) = acc {
            if let Some(op) = self.lexer.collect_of_if(&[
                Token::As,
                Token::AddAs,
                Token::OrAs,
                Token::NotAs,
                Token::XorAs,
                Token::LShiftAs,
                Token::RShiftAs,
                Token::MulAs,
                Token::ModAs,
                Token::SubAs,
                Token::DivAs,
            ]) {
                let x = self.expr()?;
                return bubble_expr!(Reassignment, a, x, op);
            }
            return Ok(None);
        }
        Ok(acc)
    }
    pub fn inner_decl(&mut self) -> ResultOptExpr {
        let mutability = self.lexer.collect_of_if(&[Token::Let, Token::Const]);
        if let Some(muta) = mutability {
            let identifier = self
                .ident()
                .xexpect_expr(&self, "expected an identifier".to_string())?;
            let sig = self.opt_signature()?;
            let _ = self
                .lexer
                .collect_of_if(&[Token::As])
                .xexpect_token(&self, "expected '='".to_string())?;
            return self
                .expr()
                .xconvert_to_result_opt()
                .xresult_opt_or(|asgn| bubble_expr!(InnerDecl, muta, identifier, sig, asgn));
        }
        Ok(None)
    }
    pub fn _if(&mut self) -> ResultOptExpr {
        let i = self.lexer.collect_if(Token::If);
        if i.is_none() {
            return Ok(None);
        }
        let _ = self
            .lexer
            .collect_if(Token::OParen)
            .xexpect_token(&self, "expected '('".to_string())?;
        let x = self.or()?;
        let _ = self
            .lexer
            .collect_if(Token::CParen)
            .xexpect_token(&self, "expected ')'".to_string())?;
        let blk = self.block()?;
        return bubble_expr!(For, x, blk);
    }
    pub fn _for(&mut self) -> ResultOptExpr {
        let f = self.lexer.collect_if(Token::For);
        if f.is_none() {
            return Ok(None);
        }
        let _ = self
            .lexer
            .collect_if(Token::OParen)
            .xexpect_token(&self, "expected '('".to_string())?;
        let x = self.or()?;
        let _ = self
            .lexer
            .collect_if(Token::CParen)
            .xexpect_token(&self, "expected ')'".to_string())?;
        if let Some(_fn) = self.anon_fn()? {
            return bubble_expr!(For, x, _fn);
        }
        let blk = self.block()?;
        return bubble_expr!(For, x, blk);
    }
    pub fn block(&mut self) -> ResultExpr {
        self.lexer
            .collect_if(Token::OBrace)
            .xexpect_token(&self, "expected '{'".to_string())?;
        let mut exprs: Vec<Box<Expr>> = vec![];
        loop {
            match self.inner_decl()? {
                Some(x) => exprs.push(x),
                None => match self.reassign()? {
                    Some(x) => exprs.push(x),
                    None => match self._for()? {
                        Some(x) => exprs.push(x),
                        None => match self._if()? {
                            Some(x) => exprs.push(x),
                            None => break,
                        },
                    },
                },
            }
        }

        if let Ok(x) = self._return() {
            exprs.push(x);
        }
        self.lexer
            .collect_if(Token::CBrace)
            .xexpect_token(&self, "expected '}'".to_string())?;
        result_expr!(Block, exprs)
    }
    pub fn expr(&mut self) -> ResultExpr {
        if self.lexer.peek().is_some_and(|l| {
            return l.token == Token::OBrace;
        }) {
            return self.block();
        }
        let mresult = self._match()?;
        if let None = mresult {
            return self.or().xresult_or(|mut left| {
                while let Some(bin) = self.lexer.collect_if(Token::Bar) {
                    left = self
                        .and()
                        .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
                }
                return Ok(left);
            });
        }
        return mresult.xconvert_to_result(
            &self,
            "expected one of: 'match', '{' with block of <exprs>, or a single <expr>".to_string(),
        );
    }
    pub fn _match(&mut self) -> ResultOptExpr {
        if let Some(_) = self.lexer.collect_if(Token::Match) {
            let _ = self
                .lexer
                .collect_if(Token::OParen)
                .xexpect_token(&self, "expected '('".to_string())?;
            let expr = self.expr()?;

            let _ = self
                .lexer
                .collect_if(Token::CParen)
                .xexpect_token(&self, "expected ')'".to_string())?;

            let _ = self
                .lexer
                .collect_if(Token::OBrace)
                .xexpect_token(&self, "expected '{'".to_string())?;

            let mut arms: Vec<Box<Expr>> = vec![];
            let first_arm = self.arm()?.xexpect_expr(
                &self,
                "expected match arm '<expr> => (<fn> | <block>)'".to_string(),
            )?;
            arms.push(first_arm);
            let second_arm = self.arm()?.xexpect_expr(
                &self,
                "expected at least 2 match arms '<expr> => (<fn> | <block>)'".to_string(),
            )?;
            arms.push(second_arm);
            loop {
                match self.arm() {
                    Ok(Some(x)) => arms.push(x),
                    Ok(None) => break,
                    Err(e) => return Err(e),
                }
            }
            let _ = self
                .lexer
                .collect_if(Token::CBrace)
                .xexpect_token(&self, "expected '}'".to_string())?;

            return bubble_expr!(Match, expr, arms);
        }
        return Ok(None);
    }

    pub fn arm(&mut self) -> ResultOptExpr {
        if self.lexer.peek().is_some_and(|l| {
            return l.token == Token::CBrace;
        }) {
            return Ok(None);
        }
        let or = self.or()?;
        let _ = self
            .lexer
            .collect_if(Token::Arrow)
            .xexpect_token(self, "expected '=>'".to_string())?;
        if let Some(blk) = self.ident() {
            return result_expr!(Arm, or, blk).xconvert_to_result_opt();
        }
        let anon = self.anon_fn()?.xconvert_to_result(
            self,
            "expected identifier or anonymous function'".to_string(),
        )?;
        result_expr!(Arm, or, anon).xconvert_to_result_opt()
    }

    pub fn or(&mut self) -> ResultExpr {
        self.and().xresult_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::Bar) {
                left = self
                    .and()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn and(&mut self) -> ResultExpr {
        self.equality().xresult_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::Ampersand) {
                left = self
                    .equality()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn equality(&mut self) -> ResultExpr {
        self.cmp().xresult_or(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Equality, Token::NotEquality])
            {
                left = self
                    .cmp()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn cmp(&mut self) -> ResultExpr {
        self.low_bin().xresult_or(|mut left| {
            while let Some(bin) =
                self.lexer
                    .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq])
            {
                left = self
                    .low_bin()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn low_bin(&mut self) -> ResultExpr {
        self.high_bin().xresult_or(|mut left| {
            while let Some(bin) = self.lexer.collect_of_if(&[Token::Plus, Token::Dash]) {
                left = self
                    .high_bin()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn high_bin(&mut self) -> ResultExpr {
        self.unary().xresult_or(|mut left| {
            while let Some(bin) = self.lexer.collect_of_if(&[
                Token::Slash,
                Token::Asterisk,
                Token::Percent,
                Token::Range,
                Token::CastAs,
            ]) {
                left = self
                    .unary()
                    .xresult_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            return Ok(left);
        })
    }
    pub fn unary(&mut self) -> ResultExpr {
        let lexeme = self.lexer.collect_of_if(&[
            Token::Ampersand,
            Token::Asterisk,
            Token::Exclam,
            Token::Dash,
            Token::Copy,
        ]);
        if let Some(x) = lexeme {
            let expr = self.unary();
            return expr.xresult_or(|result| result_expr!(UnOp, x, result));
        }
        self.access().xexpect_expr(
                &self,
                "rest, number, string, self, true, false, undefined, never, array, an expression, or identifier expected"
                    .to_string())
    }
    pub fn resolve_access(&mut self, prev: Box<Expr>) -> ResultOptExpr {
        if let Some(x) = self.lexer.collect_of_if(&[
            Token::Question,
            Token::Period,
            Token::Tilde,
            Token::OBracket,
            Token::OParen,
            Token::OBrace,
        ]) {
            match x.token {
                Token::Question => self.resolve_access(expr!(UndefBubble, prev)),
                Token::Tilde => self.resolve_access(expr!(ErrBubble, prev)),
                Token::Period => {
                    let ident = self
                        .ident()
                        .xconvert_to_result(&self, "expected identifier".to_string())?;

                    self.resolve_access(expr!(PropAccess, prev, ident))
                }
                Token::OBracket => {
                    let expr = self.array_access()?;
                    self.resolve_access(expr!(ArrayAccess, expr, prev))
                }
                Token::OParen => {
                    if let None = self.lexer.collect_if(Token::CParen) {
                        let arg_local = self.or()?;
                        let mut arg_list: Vec<Box<Expr>> = vec![];
                        arg_list.push(arg_local);
                        while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                            arg_list.push(self.or()?);
                        }

                        let _cparen = self
                            .lexer
                            .collect_if(Token::CParen)
                            .xexpect_token(&self, "expected ')'".to_string())?;
                        return self.resolve_access(expr!(Invoke, prev, Some(arg_list)));
                    }
                    return self.resolve_access(expr!(Invoke, prev, None));
                }
                Token::OBrace => {
                    if let None = self.lexer.collect_if(Token::CBrace) {
                        let ident = self
                            .ident()
                            .xexpect_expr(&self, "expected identifier".to_string())?;
                        let mut props: Vec<Box<Expr>> = vec![];
                        let _ = self
                            .lexer
                            .collect_if(Token::Colon)
                            .xexpect_token(&self, "expected ':'".to_string())?;
                        let expr = self.or()?;
                        props.push(expr!(PropAssignment, ident, expr));
                        while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                            let id = self
                                .ident()
                                .xexpect_expr(&self, "expected identifier".to_string())?;
                            let _ = self
                                .lexer
                                .collect_if(Token::Colon)
                                .xexpect_token(&self, "expected ':'".to_string())?;
                            let ex = self.or()?;
                            props.push(expr!(PropAssignment, id, ex));
                        }

                        let _ = self
                            .lexer
                            .collect_if(Token::CBrace)
                            .xexpect_token(&self, "expected '}'".to_string())?;
                        return self.resolve_access(expr!(PropAssignments, prev, Some(props)));
                    }
                    return self.resolve_access(expr!(PropAssignments, prev, None));
                }
                _ => panic!("developer error"),
            }
        } else {
            Ok(Some(prev))
        }
    }
    pub fn access(&mut self) -> ResultOptExpr {
        let term = self.terminal()?;
        if let Some(t) = term {
            return self.resolve_access(t);
        }
        Ok(None)
    }
    pub fn terminal(&mut self) -> ResultOptExpr {
        let easy = self
            .rest()
            .xif_none(|| self._true())
            .xif_none(|| self._false())
            .xif_none(|| self.undefined())
            .xif_none(|| self._self())
            .xif_none(|| self.never())
            .xif_none(|| self.num())
            .xif_none(|| self.ident())
            .xif_none(|| self.chars());
        if easy.is_none() {
            if let Some(x) = self.parens()? {
                return Ok(Some(x));
            }
            if let Some(x) = self.anon_fn()? {
                return Ok(Some(x));
            }
            return self.array_decl();
        } else {
            Ok(easy)
        }
    }
    pub fn rest(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Underscore)?;
        opt_expr!(Rest, lexeme)
    }
    pub fn num(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_of_if(&[Token::Number, Token::Decimal])?;
        opt_expr!(Number, lexeme)
    }
    pub fn ident(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Symbol)?;
        opt_expr!(Symbol, lexeme)
    }
    pub fn _true(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::True)?;
        opt_expr!(BoolValue, lexeme)
    }
    pub fn _false(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::False)?;
        opt_expr!(BoolValue, lexeme)
    }
    pub fn undefined(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Undefined)?;
        opt_expr!(UndefinedValue, lexeme)
    }
    pub fn _self(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::TSelf)?;
        opt_expr!(SelfValue, lexeme)
    }
    pub fn never(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Never)?;
        opt_expr!(Never, lexeme)
    }
    pub fn array_access(&mut self) -> ResultExpr {
        let expr = self.expr()?;
        let _ = self
            .lexer
            .collect_if(Token::CBracket)
            .xexpect_token(&self, "expected ']'".to_string())?;

        Ok(expr)
    }
    pub fn anon_fn(&mut self) -> ResultOptExpr {
        if let Some(_) = self.lexer.collect_if(Token::Func) {
            let _ = self
                .lexer
                .collect_if(Token::OParen)
                .xexpect_token(&self, "expected one of '('".to_string())?;
            let args = self.args()?;
            let _ = self
                .lexer
                .collect_if(Token::CParen)
                .xexpect_token(&self, "expected one of ')'".to_string())?;
            let ret_type = self
                .sig_union()
                .xexpect_expr(&self, "expected function return type".to_string())?;
            let block = self.block()?;
            return result_expr!(AnonFuncDecl, args, ret_type, block).xconvert_to_result_opt();
        }
        Ok(None)
    }
    pub fn parens(&mut self) -> ResultOptExpr {
        let lexeme = self.lexer.collect_if(Token::OParen);
        if lexeme.is_none() {
            return Ok(None);
        }
        let expr = self.expr()?;
        let _ = self
            .lexer
            .collect_if(Token::CParen)
            .xexpect_token(&self, "expected ')'".to_string())?;
        Ok(Some(expr))
    }
    pub fn arr_type(&mut self) -> ResultOptExpr {
        let lexeme = self.lexer.collect_if(Token::OBracket);
        if lexeme.is_none() {
            return Ok(None);
        }
        if let Some(local) = self.signature_no_colon()? {
            if let Some(_) = self.lexer.collect_if(Token::SColon) {
                let size = self
                    .num()
                    .xconvert_to_result(&self, "expected comptime size".to_string())?;
                let _ = self
                    .lexer
                    .collect_if(Token::CBracket)
                    .xexpect_token(&self, "expected ']'".to_string());
                return bubble_expr!(ArrayType, local, Some(size));
            }
            let _ = self
                .lexer
                .collect_if(Token::CBracket)
                .xexpect_token(&self, "expected ']'".to_string());
            return bubble_expr!(ArrayType, local, None);
        }
        Err(self.make_error("expected array type".to_string()))
    }
    pub fn val_type(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_of_if(&[
            Token::I32,
            Token::U32,
            Token::I64,
            Token::U64,
            Token::I16,
            Token::U16,
            Token::U8,
            Token::I8,
            Token::Bit,
            Token::F64,
            Token::D64,
            Token::F32,
            Token::D32,
            Token::D128,
            Token::F128,
            Token::ISize,
            Token::USize,
            Token::Char,
            Token::Utf8,
            Token::Utf16,
            Token::Utf32,
            Token::Utf64,
            Token::Bool,
            Token::Any,
            Token::Sized,
            Token::Scalar,
            Token::Void,
            Token::TSelf,
        ]);
        return lexeme.xconvert_expr(|span| expr!(ValueType, span));
    }
    fn make_error(&self, title: String) -> ParserError {
        let x = CodeLocation::new_lexer_stop_point(&self.lexer.lexer);
        return ParserError::new(title, x.code, x.line, x.col, x.val);
    }
}

trait ExpectToken {
    fn xexpect_token(self, parser: &Parser, title: String) -> Result<Lexeme>;
}

trait ExpectExpr {
    fn xexpect_expr(self, parser: &Parser, title: String) -> ResultExpr;
}

trait ResultOr {
    fn xresult_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr;
}

trait ResultOptOr {
    fn xresult_opt_or(self, func: impl FnOnce(Box<Expr>) -> ResultOptExpr) -> ResultOptExpr;
}

trait IfNoneDo {
    fn xif_none(self, func: impl FnOnce() -> OptExpr) -> OptExpr;
}

trait ConvertToResult {
    fn xconvert_to_result(self, parser: &Parser, title: String) -> ResultExpr;
}

trait ConvertToResultOpt {
    fn xconvert_to_result_opt(self) -> ResultOptExpr;
}

trait ConvertOptExpr {
    fn xconvert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr;
}

impl ResultOr for ResultExpr {
    fn xresult_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(inner) => func(inner),
        }
    }
}

impl ResultOptOr for ResultOptExpr {
    fn xresult_opt_or(self, func: impl FnOnce(Box<Expr>) -> ResultOptExpr) -> ResultOptExpr {
        match self {
            Err(err) => Err(err),
            Ok(Some(inner)) => func(inner),
            Ok(None) => Ok(None),
        }
    }
}

impl IfNoneDo for OptExpr {
    fn xif_none(self, func: impl FnOnce() -> OptExpr) -> OptExpr {
        match self {
            None => return func(),
            Some(val) => return Some(val),
        }
    }
}

impl ConvertOptExpr for Option<Lexeme> {
    fn xconvert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr {
        match self {
            None => None,
            Some(val) => Some(func(val)),
        }
    }
}

impl ExpectToken for Option<Lexeme> {
    fn xexpect_token(self, parser: &Parser, title: String) -> Result<Lexeme> {
        match self {
            None => Err(parser.make_error(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ExpectExpr for ResultOptExpr {
    fn xexpect_expr(self, parser: &Parser, title: String) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(Some(inner)) => Ok(inner),
            Ok(None) => Err(parser.make_error(title)),
        }
    }
}

impl ExpectExpr for OptExpr {
    fn xexpect_expr(self, parser: &Parser, title: String) -> ResultExpr {
        match self {
            None => Err(parser.make_error(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ConvertToResult for OptExpr {
    fn xconvert_to_result(self, parser: &Parser, title: String) -> ResultExpr {
        match self {
            None => Err(parser.make_error(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ConvertToResult for ResultOptExpr {
    fn xconvert_to_result(self, parser: &Parser, title: String) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(Some(val)) => Ok(val),
            Ok(None) => Err(parser.make_error(title)),
        }
    }
}

impl ConvertToResultOpt for ResultExpr {
    fn xconvert_to_result_opt(self) -> ResultOptExpr {
        match self {
            Err(err) => Err(err),
            Ok(val) => Ok(Some(val)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexeme;
    #[test]
    fn it_should_parse_unary() {
        let lexer = TLexer::new("-5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = UnOp::new(
            Lexeme {
                slice: String::from("-"),
                token: Token::Dash,
                span: 0..1,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Number,
                    span: 1..2,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::UnOp(first)));
    }
    #[test]
    fn it_should_parse_unary_num() {
        let lexer = TLexer::new("5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = Number::new(Lexeme {
            slice: String::from("5"),
            token: Token::Number,
            span: 0..1,
        });
        assert_eq!(result.unwrap(), Box::new(Expr::Number(first)));
    }
    #[test]
    fn it_should_error_unary() {
        let lexer = TLexer::new("-");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let error = ParserError::new(
           "rest, number, string, self, true, false, undefined, never, array, an expression, or identifier expected"
                .to_string(),
            "-".to_string(),
            1,
            2,
            "".to_string(),
        );
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_error_high_bin() {
        let lexer = TLexer::new("5 *");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let error = ParserError::new(
            "rest, number, string, self, true, false, undefined, never, array, an expression, or identifier expected"
                .to_string(),
            "5 *".to_string(),
            1,
            4,
            "".to_string(),
        );
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_parse_high_bin() {
        let lexer = TLexer::new("5 * 2");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let expr = BinOp::new(
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Number,
                    span: 0..1,
                }
            ),
            Lexeme {
                slice: String::from("*"),
                token: Token::Asterisk,
                span: 2..3,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("2"),
                    token: Token::Number,
                    span: 4..5,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::BinOp(expr)));
    }
    #[test]
    fn it_should_parse_low_bin_mul() {
        let lexer = TLexer::new("5 + 3 * 2 + 1");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let expr = expr!(
            BinOp,
            expr!(
                BinOp,
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("5"),
                        token: Token::Number,
                        span: 0..1,
                    }
                ),
                Lexeme {
                    slice: String::from("+"),
                    token: Token::Plus,
                    span: 2..3,
                },
                expr!(
                    BinOp,
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("3"),
                            token: Token::Number,
                            span: 4..5
                        }
                    ),
                    Lexeme {
                        slice: String::from("*"),
                        token: Token::Asterisk,
                        span: 6..7
                    },
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("2"),
                            token: Token::Number,
                            span: 8..9
                        }
                    )
                ),
            ),
            Lexeme {
                slice: String::from("+"),
                token: Token::Plus,
                span: 10..11,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("1"),
                    token: Token::Number,
                    span: 12..13,
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }

    #[test]
    fn it_should_parse_block() {
        let lexer = TLexer::new("{ let x = 5 return x }");
        let mut parser = Parser::new(lexer);
        let result = parser.block();
        let expr = expr!(
            Block,
            vec![
                expr!(
                    InnerDecl,
                    Lexeme {
                        slice: String::from("let"),
                        token: Token::Let,
                        span: 2..5,
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: String::from("x"),
                            token: Token::Symbol,
                            span: 6..7
                        }
                    ),
                    None,
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("5"),
                            token: Token::Number,
                            span: 10..11
                        }
                    ),
                ),
                expr!(
                    RetOp,
                    Lexeme {
                        slice: String::from("return"),
                        token: Token::Return,
                        span: 12..18,
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: String::from("x"),
                            token: Token::Symbol,
                            span: 19..20
                        }
                    )
                )
            ]
        );
        assert_eq!(result.unwrap(), expr);
    }

    #[test]
    fn it_should_parse_low_bin() {
        let lexer = TLexer::new("5 + 3 + 2");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let expr = expr!(
            BinOp,
            expr!(
                BinOp,
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("5"),
                        token: Token::Number,
                        span: 0..1,
                    }
                ),
                Lexeme {
                    slice: String::from("+"),
                    token: Token::Plus,
                    span: 2..3,
                },
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("3"),
                        token: Token::Number,
                        span: 4..5
                    }
                )
            ),
            Lexeme {
                slice: String::from("+"),
                token: Token::Plus,
                span: 6..7
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("2"),
                    token: Token::Number,
                    span: 8..9
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }
    #[test]
    fn it_should_parse_fn() {
        let lexer = TLexer::new("pub const add = fn(x) usize { return x }");
        let mut parser = Parser::new(lexer);
        let result = parser.top_decl();
        let expr = expr!(
            FuncDecl,
            Some(Lexeme {
                slice: String::from("pub"),
                token: Token::Pub,
                span: 0..3
            }),
            Lexeme {
                slice: "const".to_string(),
                token: Token::Const,
                span: 4..9
            },
            expr!(
                Symbol,
                Lexeme {
                    slice: "add".to_string(),
                    token: Token::Symbol,
                    span: 10..13
                }
            ),
            Some(vec![expr!(
                ArgDef,
                expr!(
                    Symbol,
                    Lexeme {
                        slice: "x".to_string(),
                        token: Token::Symbol,
                        span: 19..20
                    }
                ),
                None
            )]),
            expr!(
                Sig,
                Some(expr!(
                    ValueType,
                    Lexeme {
                        slice: "usize".to_string(),
                        token: Token::USize,
                        span: 22..27
                    }
                )),
                None,
                None,
                None
            ),
            expr!(
                Block,
                vec![expr!(
                    RetOp,
                    Lexeme {
                        slice: "return".to_string(),
                        token: Token::Return,
                        span: 30..36
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: "x".to_string(),
                            token: Token::Symbol,
                            span: 37..38
                        }
                    )
                )]
            ),
            None
        );
        assert_eq!(result.unwrap(), expr);
    }
}
