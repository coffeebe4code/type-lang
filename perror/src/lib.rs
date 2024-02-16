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
pub struct LinterError {
    title: String,
    points: Vec<LinterErrorPoint>,
    suggestions: Vec<String>,
}

impl LinterError {
    pub fn new(title: String) -> LinterError {
        LinterError {
            title,
            points: vec![],
            suggestions: vec![],
        }
    }
    pub fn add_point(&mut self, lep: LinterErrorPoint, sug: String) {
        self.suggestions.push(sug);
        self.points.push(lep);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LinterErrorPoint {
    code: String,
    line: usize,
    col: usize,
}

impl LinterErrorPoint {
    pub fn new(code: String, line: usize, col: usize) -> LinterErrorPoint {
        LinterErrorPoint { code, line, col }
    }
}

impl fmt::Display for LinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self
            .points
            .iter()
            .map(|x| return format!("\t{}\n", x))
            .collect::<String>();
        let sug = self
            .suggestions
            .iter()
            .map(|x| return format!("\t{}\n", x))
            .collect::<String>();
        write!(
            f,
            "title: {}\nerrors:\n{}suggestions: {}",
            self.title, x, sug
        )
    }
}

impl fmt::Display for LinterErrorPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = (0..self.col - 1).map(|_| return " ").collect::<String>();
        x.push('^');
        write!(
            f,
            "code:\n  {}\n  {}\nline: {}\ncol: {}\n",
            self.code, x, self.line, self.col
        )
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
