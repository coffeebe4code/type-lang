use ast::*;
use ir::*;
use lexer::*;
use linker::*;
use object::*;
use parser::*;
use slt::*;
use std::fs::{create_dir, write};
use std::path::Path;
use std::process::Command;

fn main() {
    let lex = ProseLexer::new("pub const main = fn() { let m = 7; let x = 5; return x + m; }");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.top_decl().unwrap();
    let mut ir = IRSource::new(0, SLT::new());
    match *ast_parsed {
        Expr::FuncDef(val) => {
            let result = ir.begin(val);
            println!("{}", ir.get_ir(&result).unwrap());
            if !Path::new(".ty-cache").is_dir() {
                create_dir(".ty-cache").unwrap();
            }
            write(".ty-cache/main.o", build_main(result)).unwrap();
            link(Path::new(".ty-cache/main.o"), Path::new(".ty-cache/main"));
            let status = Command::new("./.ty-cache/main").args(&[""]).status();
            println!("Status {:?}", status);
        }
        _ => panic!("not a func def!"),
    }
}
