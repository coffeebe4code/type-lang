use ir::IRFunc;
use lexer::TLexer;
use linter::LintSource;
use object::*;
use parser::Parser;
use std::fs::create_dir;
use std::fs::write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use symtable::SymTable;

pub fn from_buffer(contents: &str, path: &Path) -> () {
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.all().unwrap();
    let mut type_tables = vec![];
    let mut scopes = vec![];
    let mut linter = LintSource::new(path.to_str().unwrap(), &mut scopes, &mut type_tables);
    let lint_res = linter.lint_check(&ast_parsed);
    let mut ir = IRFunc::new(0, SymTable::new(), linter.ttbls.get(0).unwrap());
    if linter.issues.len() > 0 {
        for x in linter.issues {
            println!("{}", x);
        }
        panic!("linter issues exist");
    }
    let mut om = ObjectSource::new(path.to_str().unwrap());
    println!("lint res {:?}", lint_res);
    om.add_const_data(lint_res.get(0).unwrap().into_init().right.into_data());
    let rc_thing = lint_res.get(1).unwrap().to_owned();
    let result = ir.begin(rc_thing);
    if !Path::new(".ty-cache").is_dir() {
        create_dir(".ty-cache").unwrap();
    }
    let wo_extension = path.with_extension("");
    let filename = wo_extension.file_name().unwrap().to_str().unwrap();
    let mut output = PathBuf::new();
    output.push(".ty-cache");
    output.push(filename);
    output.set_extension("o");
    let mut om = ObjectSource::new(filename);
    om.add_fn(result);
    write(output, om.flush_self()).unwrap();
}

pub fn from_file(input_path: &PathBuf) -> () {
    let mut file = File::open(input_path.clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.all().unwrap();
    let mut type_tables = vec![];
    let mut scopes = vec![];
    let mut linter = LintSource::new(input_path.to_str().unwrap(), &mut scopes, &mut type_tables);
    let lint_res = linter.lint_check(&ast_parsed);
    let mut ir = IRFunc::new(0, SymTable::new(), linter.ttbls.get(0).unwrap());
    if linter.issues.len() > 0 {
        panic!("linter issues exist");
    }
    let rc_thing = lint_res.first().unwrap().to_owned();
    let result = ir.begin(rc_thing);
    if !Path::new(".ty-cache").is_dir() {
        create_dir(".ty-cache").unwrap();
    }
    let wo_extension = input_path.with_extension("");
    let filename = wo_extension.file_name().unwrap().to_str().unwrap();
    let mut output = PathBuf::new();
    output.push(".ty-cache");
    output.push(filename);
    output.set_extension("o");
    let mut om = ObjectSource::new(filename);
    om.add_fn(result);
    write(output, om.flush_self()).unwrap();
}
