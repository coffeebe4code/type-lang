use ast::Expr;
use ir::IRSource;
use lexer::TLexer;
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

pub fn from_buffer(contents: &str, path: &Path) -> () {
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.top_decl().unwrap();
    let mut ir = IRSource::new(0, SymTable::new("one".to_string()));
    match *ast_parsed {
        Expr::FuncDecl(val) => {
            let result = ir.begin(val);
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
        _ => panic!("not a func def!"),
    }
}

pub fn from_file(input_path: &PathBuf) -> () {
    let mut ty = File::open(input_path.clone()).unwrap();
    let mut contents = String::new();
    ty.read_to_string(&mut contents).unwrap();
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.top_decl().unwrap();
    let mut ir = IRSource::new(0, SymTable::new("file".to_string()));
    match *ast_parsed {
        Expr::FuncDecl(val) => {
            let result = ir.begin(val);
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
        _ => panic!("not a func def!"),
    }
}
