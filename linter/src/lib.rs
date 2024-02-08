use ast::*;
use codelocation::*;
use lexer::*;
use perror::LinterError;
use perror::LinterErrorPoint;
use token::Token;
use types::*;

type ResultIndex = Result<Type, usize>;
type ResultError = Result<Type, LinterError>;

pub struct LintSource<'s> {
    buffer: &'s str,
    pub issues: Vec<LinterError>,
}

impl<'s> LintSource<'s> {
    pub fn new(buffer: &'s str) -> Self {
        LintSource {
            buffer,
            issues: vec![],
        }
    }
    pub fn type_check(&mut self, to_cmp: &mut Expr) -> ResultIndex {
        match to_cmp {
            Expr::TopDecl(x) => {
                let result = self.type_check(&mut x.expr)?;
                if let Some(typ) = &x.typ {
                    //if typ != result.unwrap() {}
                }
                return Ok(result);
            }

            Expr::Number(x) => match x.val.token {
                Token::Decimal => Ok(Type::F64(x.val.slice.parse::<f64>().unwrap())),
                Token::Num => Ok(Type::U64(x.val.slice.parse::<u64>().unwrap())),
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
trait PushErr {
    fn push_if_err(self, linter: &mut LintSource) -> ResultIndex;
}
impl PushErr for ResultError {
    fn push_if_err(self, linter: &mut LintSource) -> ResultIndex {
        match self {
            Ok(typ) => return Ok(typ),
            Err(err) => {
                linter.issues.push(err);
                return Err(linter.issues.len() - 1);
            }
        }
    }
}

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
