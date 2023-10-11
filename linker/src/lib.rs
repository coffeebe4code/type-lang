use std::path::Path;
use std::path::PathBuf;

pub fn link(obj_file: Vec<&PathBuf>, output: &PathBuf) -> () {
    use std::process::Command;

    // link the .o file using host linker
    if cfg!(windows) {
        Command::new("link")
            .arg(format!("{}{}{}", "/out:", output.to_str().unwrap(), ".exe"))
            .args(obj_file)
            .arg("/entry:main")
            .status()
            .unwrap();
    } else {
        Command::new("cc")
            .args(obj_file)
            .args(&[Path::new("-o"), output])
            .status()
            .unwrap();
    }
}
