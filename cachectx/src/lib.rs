use ast::*;
use lexer::*;
use parser::*;
use std::{fs::File, io::Read, path::PathBuf};

pub struct CacheContext {
    sources: Vec<PathBuf>,
    parsers: Vec<Box<Expr>>,
    errors: Vec<Box<Expr>>,
}

impl CacheContext {
    pub fn new() -> Self {
        CacheContext {
            sources: vec![],
            parsers: vec![],
            errors: vec![],
        }
    }
    pub fn add_file(&mut self, source: PathBuf) -> () {
        self.sources.push(source);
    }
    pub fn take_parsed(&mut self) -> () {
        self.sources.iter().for_each(|s| {
            let mut file = File::open(s).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let lexer = TLexer::new(&contents);
            let mut parser = Parser::new(lexer);
            let result = parser.all().unwrap();
            self.parsers.push(result);
        })
    }
}
