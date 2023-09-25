use std::fmt;

pub type Result<T> = std::result::Result<T, ParserError>;
pub type ResultFir<T> = std::result::Result<T, FirError>;

#[derive(Debug, PartialEq, Clone)]
pub struct FirError {
    title: String,
}

impl fmt::Display for FirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}\n
            {}\n",
            self.title, "Function IR Error"
        )
    }
}

impl FirError {
    pub fn new(title: String) -> FirError {
        FirError { title }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserError {
    title: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}\n
            {}\n",
            self.title, "ParseError"
        )
    }
}

impl ParserError {
    pub fn new(title: String) -> ParserError {
        ParserError { title }
    }
}
