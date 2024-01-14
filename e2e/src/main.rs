use linker::link;
use std::path::Path;
use std::process::Command;

fn main() {
    objmaker::from_buffer(
        "pub const main = fn() { 
            let m = 7
            let x = 5 
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
}
