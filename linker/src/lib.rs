use std::path::Path;

pub fn link(obj_file: &Path, output: &Path) -> () {
    use std::process::Command;
    println!("output {}", output.to_str().unwrap());

    // link the .o file using host linker
    if cfg!(windows) {
    Command::new("link")
        .arg(format!("{}{}{}","/out:", output.to_str().unwrap(), ".exe"))
        .arg(&obj_file)
        .arg("/entry:main")
        .status().unwrap();
    } else {
    Command::new("cc")
        .args(&[&obj_file, Path::new("-o"), output])
        .status().unwrap();
    }
}
