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
    found: String,
    code: String,
    line: usize,
    col: usize,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = (0..self.col - 1).map(|_| return " ").collect::<String>();
        x.push('^');
        write!(
            f,
            "title: {}, but found '{}'\ncode:\n  {}\n  {}\nline: {}\ncol: {}\n",
            self.title, self.found, self.code, x, self.line, self.col
        )
    }
}

impl ParserError {
    pub fn new(title: String, code: String, line: usize, col: usize, found: String) -> ParserError {
        ParserError {
            title,
            code,
            line,
            col,
            found,
        }
    }
}
