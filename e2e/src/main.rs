use lexer::TLexer;
use linker::link;
use linter::LintSource;
use parser::Parser;
use std::fs::File;
use std::io::Read;
use symtable::TypeTable;

use std::path::Path;
use std::process::Command;

fn main() {
    println!("[run] simple exe");
    objmaker::from_buffer(
        "pub const main = fn() usize { 
            const m = 7
            const x = 5 
            return x + m 
        }",
        Path::new("main.ty"),
    );
    let input = Path::new(".ty-cache/main.o").to_path_buf();
    let output = Path::new(".ty-cache/main").to_path_buf();
    link(vec![&input], &output);
    let output = Command::new(".ty-cache/main")
        .args(&[""])
        .spawn()
        .expect("main to run")
        .wait()
        .expect("expected child to finish")
        .code();

    assert!(output == Some(12));
    println!("  [ok] simple exe success!");

    println!("[run] full parse");

    let mut ty = File::open("test/test.ty").unwrap();
    let mut contents = String::new();
    ty.read_to_string(&mut contents).unwrap();
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let res = parse.all();
    match res {
        Ok(_) => println!("  [ok] full parse success!"),
        Err(x) => {
            println!("  [fail]\n{}", x);
            std::process::exit(1);
        }
    }
    println!("[run] full linting without cache context");
    let mut typ_table = TypeTable::new();
    let mut linter = LintSource::new(&contents, &mut typ_table);
    let borrow = res.unwrap();
    let result = linter.lint_check(&mut borrow.to_owned());

    if linter.issues.len() > 0 {
        println!("  [fail]\n issues: {:?}\n", linter.issues);
        std::process::exit(1);
    }
    println!("  [ok] full lint success!");
}
