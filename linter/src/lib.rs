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
    pub fn type_check(&mut self, to_cmp: &mut Expr) -> ResultIndex {
        match to_cmp {
            Expr::BinOp(bin) => {
                let left = self.type_check(&mut bin.left);
                let right = self.type_check(&mut bin.right);

                return Err(0);
            }

            Expr::Number(x) => match x.val.token {
                Token::Decimal => Ok(Type::F64(x.val.slice.parse::<f64>().unwrap())),
                Token::Num => Ok(Type::U64(x.val.slice.parse::<u64>().unwrap())),
                _ => panic!("type-lang linter issue"),
            },
            _ => panic!("type-lang linter issue"),
        }
    }
    //pub fn lint_top_decl(&self, left: &mut Expr, right: &Expr) -> ResultError {
    //    match right {
    //        Expr::StructDecl(x) => {
    //            return Ok(());
    //        }
    //        Expr::FuncDecl(x) => {}
    //        Expr::TraitDecl(x) => {}
    //        _ => Ok(()),
    //    }
    //}
    pub fn update_error(&self, mut err: LinterError, suggestion: String, lexeme: Lexeme) -> () {
        let xcl = CodeLocation::new(self.buffer, lexeme);
        let lep = LinterErrorPoint::new(xcl.code, xcl.line, xcl.col);
        err.add_point(lep, suggestion);
    }
    pub fn make_error(&self, title: String) -> LinterError {
        LinterError::new(title)
    }
}

//trait ModExpr {
//    fn sig_mod(&mut self, typ: Option<Expr>
//
//}

//trait PushErr {
//    fn push_if_err(self, linter: &mut LintSource) -> ResultIndex;
//}
//impl PushErr for ResultError {
//    fn push_if_err(self, linter: &mut LintSource) -> ResultIndex {
//        match self {
//            Ok(typ) => return Ok(typ),
//            Err(err) => {
//                linter.issues.push(err);
//                return Err(linter.issues.len() - 1);
//            }
//        }
//    }
//}

//pub trait SlideIntoFrom {
//    fn slide_into(&self, other: &Type) -> ResultCheck;
//    fn slide_from(&self, other: &Type) -> ResultCheck;
//}
//impl SlideIntoFrom for i64 {
//    fn slide_into(&self, other: &Type) -> ResultCheck {
//        match other {
//            Type::I64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//    fn can_slide_from(&self, other: &Type) -> bool {
//        match other {
//            Type::I64(_) | Type::U64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//}
//
//impl SlideIntoFrom for u64 {
//    fn can_slide_into(&self, other: &Type) -> bool {
//        match other {
//            Type::I64(_) | Type::U64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//    fn can_slide_from(&self, other: &Type) -> bool {
//        match other {
//            Type::I64(_) | Type::U64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//}
//
//impl SlideIntoFrom for f64 {
//    fn can_slide_into(&self, other: &Type) -> bool {
//        match other {
//            Type::F64(_) => true,
//            _ => false,
//        }
//    }
//    fn can_slide_from(&self, other: &Type) -> bool {
//        match other {
//            Type::I64(_) | Type::U64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//}
//
//impl SlideIntoFrom for Type {
//    fn can_slide_into(&self, other: &Type) -> bool {
//        match self {
//            Type::Struct(_) => true,
//            _ => false,
//        }
//    }
//    fn can_slide_from(&self, other: &Type) -> bool {
//        match other {
//            Type::I64(_) | Type::U64(_) | Type::F64(_) => true,
//            _ => false,
//        }
//    }
//}
