use ast::*;
use lexer::Lexeme;
use lexer::ProseLexer;
use perror::*;
use token::Token;

pub type ResultOptExpr = Result<Option<Box<Expr>>>;
pub type ResultExpr = Result<Box<Expr>>;
pub type OptExpr = Option<Box<Expr>>;

pub struct Parser<'s> {
    lexer: ProseLexer<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: ProseLexer<'s>) -> Self {
        Parser { lexer }
    }
    pub fn _return(&mut self) -> ResultExpr {
        let span = self
            .lexer
            .collect_if(Token::Return)
            .expect_token("expected return keyword".to_string())?;
        self.or().result_or(|expr| {
            self.lexer
                .collect_if(Token::SColon)
                .expect_token("expected ';'".to_string())?;
            result_expr!(RetOp, span, expr)
        })
    }
    pub fn _struct(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
    ) -> ResultExpr {
        let _ = self
            .lexer
            .collect_if(Token::Struct)
            .expect_token("expected struct keyword".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::OBrace)
            .expect_token("expected '{'".to_string())?;
        let decls = self.declarators()?;
        let _ = self
            .lexer
            .collect_if(Token::CBrace)
            .expect_token("expected '}'".to_string())?;
        result_expr!(StructDef, visibility, mutability, identifier, decls)
    }
    pub fn top_decl(&mut self) -> ResultExpr {
        let has_pub = self.lexer.collect_if(Token::Pub);
        let mutability = self
            .lexer
            .collect_of_if(&[Token::Let, Token::Const])
            .expect_token("expected mutability".to_string())?;
        let identifier = self
            .ident()
            .expect_expr("expected identifier".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::As)
            .expect_token("expected =".to_string())?;
        if let Some(val) = self.lexer.collect_of_if(&[Token::Struct, Token::Func]) {
            match val.token {
                Token::Struct => return self._struct(has_pub, mutability, identifier),
                Token::Func => return self._fn(has_pub, mutability, identifier),
                _ => panic!("developer error"),
            }
        }
        self.or()
    }
    pub fn _fn(
        &mut self,
        visibility: Option<Lexeme>,
        mutability: Lexeme,
        identifier: Box<Expr>,
    ) -> ResultExpr {
        let _ = self
            .lexer
            .collect_if(Token::OParen)
            .expect_token("expected '('".to_string())?;
        let args = self.args()?;
        let _ = self
            .lexer
            .collect_if(Token::CParen)
            .expect_token("expected ')'".to_string())?;
        let block = self.block()?;
        result_expr!(FuncDef, visibility, mutability, identifier, args, block)
    }
    pub fn _type(&mut self) -> ResultExpr {
        self.lexer
            .collect_of_if(&[Token::Num, Token::Any, Token::U64])
            .convert_expr(|span| expr!(TypeSimple, span))
            .convert_to_result("expected type".to_string())
    }
    pub fn declarators(&mut self) -> Result<Option<Vec<Box<Expr>>>> {
        if let Some(local) = self.declarator() {
            let mut decl_list: Vec<Box<Expr>> = vec![];
            decl_list.push(local);
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                decl_list.push(
                    self.declarator()
                        .expect_expr("expected argument definition".to_string())?,
                );
            }
            return Ok(Some(decl_list));
        }
        Ok(None)
    }
    pub fn declarator(&mut self) -> OptExpr {
        self.ident()
    }
    pub fn args(&mut self) -> Result<Option<Vec<Box<Expr>>>> {
        if let Some(arg_local) = self.arg() {
            let mut arg_list: Vec<Box<Expr>> = vec![];
            arg_list.push(arg_local);
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                arg_list.push(
                    self.arg()
                        .expect_expr("expected argument definition".to_string())?,
                );
            }
            return Ok(Some(arg_list));
        }
        Ok(None)
    }
    pub fn arg(&mut self) -> OptExpr {
        self.ident()
    }
    pub fn inner_assign(&mut self) -> ResultOptExpr {
        let mutability = self.lexer.collect_of_if(&[Token::Let, Token::Const]);
        if let Some(muta) = mutability {
            let identifier = self
                .ident()
                .expect_expr("expected identifier".to_string())?;
            let _ = self
                .lexer
                .collect_of_if(&[Token::As])
                .expect_token("expected =".to_string())?;
            return self.or().convert_to_result_opt().result_opt_or(|asgn| {
                self.lexer
                    .collect_if(Token::SColon)
                    .expect_token("expected ';'".to_string())?;
                bubble_expr!(AsDef, muta, identifier, asgn)
            });
        }
        Ok(None)
    }
    pub fn block(&mut self) -> ResultExpr {
        self.lexer
            .collect_if(Token::OBrace)
            .expect_token("expected '{'".to_string())?;
        let mut exprs: Vec<Box<Expr>> = vec![];
        loop {
            match self.inner_assign() {
                Ok(Some(x)) => exprs.push(x),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }

        if let Ok(x) = self._return() {
            exprs.push(x);
        }
        self.lexer
            .collect_if(Token::CBrace)
            .expect_token("expected '}'".to_string())?;
        result_expr!(Block, exprs)
    }
    pub fn or(&mut self) -> ResultExpr {
        self.and().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::Or) {
                left = self
                    .and()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn and(&mut self) -> ResultExpr {
        self.equality().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::And) {
                left = self
                    .equality()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn equality(&mut self) -> ResultExpr {
        self.cmp().result_or(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Equality, Token::NotEquality])
            {
                left = self
                    .cmp()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn cmp(&mut self) -> ResultExpr {
        self.low_bin().result_or(|mut left| {
            while let Some(bin) =
                self.lexer
                    .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq])
            {
                left = self
                    .low_bin()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn low_bin(&mut self) -> ResultExpr {
        self.high_bin().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_of_if(&[Token::Plus, Token::Sub]) {
                left = self
                    .high_bin()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn high_bin(&mut self) -> ResultExpr {
        self.unary().result_or(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Div, Token::Mul, Token::Mod])
            {
                left = self
                    .unary()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            return Ok(left);
        })
    }
    pub fn unary(&mut self) -> ResultExpr {
        let lexeme = self.lexer.collect_of_if(&[Token::Not, Token::Sub]);
        if let Some(x) = lexeme {
            let expr = self.unary();
            return expr.result_or(|result| result_expr!(UnOp, x, result));
        }
        self.num()
            .if_none_do(|| self.ident())
            .convert_to_result("number or identifier".to_string())
    }
    pub fn num(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Num)?;
        opt_expr!(Number, lexeme)
    }
    pub fn ident(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Symbol)?;
        opt_expr!(Symbol, lexeme)
    }
}

trait ExpectToken {
    fn expect_token(self, title: String) -> Result<Lexeme>;
}

trait ExpectExpr {
    fn expect_expr(self, title: String) -> ResultExpr;
}

trait ResultOr {
    fn result_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr;
}

trait ResultOptOr {
    fn result_opt_or(self, func: impl FnOnce(Box<Expr>) -> ResultOptExpr) -> ResultOptExpr;
}

trait IfNoneDo {
    fn if_none_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr;
}

trait ConvertToResult {
    fn convert_to_result(self, title: String) -> ResultExpr;
}

trait ConvertToResultOpt {
    fn convert_to_result_opt(self) -> ResultOptExpr;
}

trait ConvertOptExpr {
    fn convert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr;
}

trait ChainExpect {
    fn chain_expect(self, title: String) -> ResultExpr;
}

impl ResultOr for ResultExpr {
    fn result_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(inner) => func(inner),
        }
    }
}

impl ResultOptOr for ResultOptExpr {
    fn result_opt_or(self, func: impl FnOnce(Box<Expr>) -> ResultOptExpr) -> ResultOptExpr {
        match self {
            Err(err) => Err(err),
            Ok(Some(inner)) => func(inner),
            Ok(None) => Ok(None),
        }
    }
}

impl IfNoneDo for OptExpr {
    fn if_none_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr {
        match self {
            None => return func(),
            Some(val) => return Some(val),
        }
    }
}

impl ConvertOptExpr for Option<Lexeme> {
    fn convert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr {
        match self {
            None => None,
            Some(val) => Some(func(val)),
        }
    }
}

impl ExpectToken for Option<Lexeme> {
    fn expect_token(self, title: String) -> Result<Lexeme> {
        match self {
            None => Err(ParserError::new(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ExpectExpr for OptExpr {
    fn expect_expr(self, title: String) -> ResultExpr {
        match self {
            None => Err(ParserError::new(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ConvertToResult for OptExpr {
    fn convert_to_result(self, title: String) -> ResultExpr {
        match self {
            None => Err(ParserError::new(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ConvertToResultOpt for ResultExpr {
    fn convert_to_result_opt(self) -> ResultOptExpr {
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
        let lexer = ProseLexer::new("-5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = UnOp::new(
            Lexeme {
                slice: String::from("-"),
                token: Token::Sub,
                span: 0..1,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Num,
                    span: 1..2,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::UnOp(first)));
    }
    #[test]
    fn it_should_parse_unary_num() {
        let lexer = ProseLexer::new("5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = Number::new(Lexeme {
            slice: String::from("5"),
            token: Token::Num,
            span: 0..1,
        });
        assert_eq!(result.unwrap(), Box::new(Expr::Number(first)));
    }
    #[test]
    fn it_should_error_unary() {
        let lexer = ProseLexer::new("-");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let error = ParserError::new("number or identifier".to_string());
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_error_high_bin() {
        let lexer = ProseLexer::new("5 *");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let error = ParserError::new("number or identifier".to_string());
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_parse_high_bin() {
        let lexer = ProseLexer::new("5 * 2");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let expr = BinOp::new(
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Num,
                    span: 0..1,
                }
            ),
            Lexeme {
                slice: String::from("*"),
                token: Token::Mul,
                span: 2..3,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("2"),
                    token: Token::Num,
                    span: 4..5,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::BinOp(expr)));
    }
    #[test]
    fn it_should_parse_low_bin_mul() {
        let lexer = ProseLexer::new("5 + 3 * 2 + 1");
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
                        token: Token::Num,
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
                            token: Token::Num,
                            span: 4..5
                        }
                    ),
                    Lexeme {
                        slice: String::from("*"),
                        token: Token::Mul,
                        span: 6..7
                    },
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("2"),
                            token: Token::Num,
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
                    token: Token::Num,
                    span: 12..13,
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }

    #[test]
    fn it_should_parse_block() {
        let lexer = ProseLexer::new("{ let x = 5; return x; }");
        let mut parser = Parser::new(lexer);
        let result = parser.block();
        let expr = expr!(
            Block,
            vec![
                expr!(
                    AsDef,
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
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("5"),
                            token: Token::Num,
                            span: 10..11
                        }
                    )
                ),
                expr!(
                    RetOp,
                    Lexeme {
                        slice: String::from("return"),
                        token: Token::Return,
                        span: 13..19,
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: String::from("x"),
                            token: Token::Symbol,
                            span: 20..21
                        }
                    )
                )
            ]
        );
        assert_eq!(result.unwrap(), expr);
    }

    #[test]
    fn it_should_parse_low_bin() {
        let lexer = ProseLexer::new("5 + 3 + 2");
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
                        token: Token::Num,
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
                        token: Token::Num,
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
                    token: Token::Num,
                    span: 8..9
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }
    #[test]
    fn it_should_parse_fn() {
        let lexer = ProseLexer::new("pub const add = fn(x) { return x; }");
        let mut parser = Parser::new(lexer);
        let result = parser.top_decl();
        let expr = expr!(
            FuncDef,
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
                Symbol,
                Lexeme {
                    slice: "x".to_string(),
                    token: Token::Symbol,
                    span: 19..20
                }
            )]),
            expr!(
                Block,
                vec![expr!(
                    RetOp,
                    Lexeme {
                        slice: "return".to_string(),
                        token: Token::Return,
                        span: 24..30
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: "x".to_string(),
                            token: Token::Symbol,
                            span: 31..32
                        }
                    )
                )]
            )
        );
        assert_eq!(result.unwrap(), expr);
    }
}
