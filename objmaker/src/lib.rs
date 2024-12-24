use lexer::TLexer;
use linter::LintSource;
use parser::Parser;
use scir::Scir;
use std::fs::create_dir;
use std::fs::write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub fn from_buffer(contents: &str, path: &Path) -> () {
    let lex = TLexer::new(&contents);
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.all().unwrap();
    let mut type_tables = vec![];
    let mut scopes = vec![];
    let mut linter = LintSource::new(path.to_str().unwrap(), &mut scopes, &mut type_tables);
    let lint_res = linter.lint_check(&ast_parsed);
    if linter.issues.len() > 0 {
        for x in linter.issues {
            println!("{}", x);
        }
        panic!("linter issues exist");
    }
    if !Path::new(".ty").is_dir() {
        create_dir(".ty").unwrap();
    }
    let wo_extension = path.with_extension("");
    let filename = wo_extension.file_name().unwrap().to_str().unwrap();
    let mut output = PathBuf::new();
    output.push(".ty");
    output.push(filename);
    output.set_extension("o");
    let mut scir = Scir::new(filename, scopes, type_tables);
    scir.loopf(lint_res);
    write(output, scir.flush_self()).unwrap();
}

pub fn from_file(input_path: &PathBuf) -> () {
    let mut file = File::open(input_path.clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    from_buffer(&contents, input_path);
}
