use ir::IRSource;
use lexer::TLexer;
use linter::LintSource;
use object::build_std_fn;
use object::flush_obj;
use object::new_obj_handler;
use parser::Parser;
use std::fs::create_dir;
use std::fs::write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use symtable::SymTable;
use typetable::TypeTable;

pub fn from_buffer(contents: &str, path: &Path) -> () {
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.all().unwrap();
    let mut type_table = TypeTable::new();
    let mut linter = LintSource::new(path.to_str().unwrap(), &mut type_table);
    let lint_res = linter.lint_check(&ast_parsed);
    let mut ir = IRSource::new(0, SymTable::new(), linter.ttbl);
    if linter.issues.len() > 0 {
        for x in linter.issues {
            println!("{}", x);
        }
        panic!("linter issues exist");
    }
    let rc_thing = lint_res.first().unwrap().to_owned();
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
    let mut om = new_obj_handler(filename);
    build_std_fn(&mut om, result, filename);
    write(output, flush_obj(om)).unwrap();
}

pub fn from_file(input_path: &PathBuf) -> () {
    let mut file = File::open(input_path.clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.all().unwrap();
    let mut type_table = TypeTable::new();
    let mut linter = LintSource::new(input_path.to_str().unwrap(), &mut type_table);
    let lint_res = linter.lint_check(&ast_parsed);
    let mut ir = IRSource::new(0, SymTable::new(), linter.ttbl);
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
    let mut om = new_obj_handler(filename);
    build_std_fn(&mut om, result, filename);
    write(output, flush_obj(om)).unwrap();
}
