use lexer::*;
use linter::*;
use parser::*;
use scopetable::*;
use std::{fs::File, io::Read, path::PathBuf, rc::Rc};
use symtable::*;
use types::*;
use typetable::*;

struct InternalContext {
    source: PathBuf,
    parsed: Option<ResultExpr>,
    ttbls: Option<Vec<TypeTable>>,
    scopes: Option<Vec<ScopeTable>>,
    tree: Option<Vec<Rc<Box<TypeTree>>>>,
}

pub struct CacheContext {
    files: Vec<InternalContext>,
}

impl CacheContext {
    pub fn new() -> Self {
        CacheContext { files: vec![] }
    }
    pub fn add_file(&mut self, source: PathBuf) -> () {
        self.files.push(InternalContext {
            source,
            parsed: None,
            ttbls: None,
            scopes: None,
            tree: None,
        });
    }
    pub fn analysis(&mut self) -> () {
        self.files.iter_mut().enumerate().for_each(|(_i, ic)| {
            let mut file = File::open(&ic.source).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let lexer = TLexer::new(&contents);
            let mut parser = Parser::new(lexer);
            match parser.all() {
                Ok(mut val) => {
                    let mut ttbls = vec![];
                    let mut scopes = vec![];
                    let mut linter = LintSource::new(&contents, &mut scopes, &mut ttbls);

                    let analysis = linter.lint_check(&mut val);

                    ic.tree = Some(analysis);
                    ic.ttbls = Some(ttbls);
                }
                Err(perr) => ic.parsed = Some(Err(perr)),
            }
        })
    }
}
