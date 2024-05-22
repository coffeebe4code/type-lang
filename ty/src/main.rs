use clap::value_parser;
use clap::ArgMatches;
use std::path::PathBuf;
use std::process::exit;

use clap::arg;
use clap::ArgAction;
use clap::Command;

const TY: &str = "ty";
const OBJ: &str = "obj";
const LINK: &str = "link";

fn main() {
    let sub_o = Command::new(OBJ).about("generates object files from .ty files").arg(
        arg!([files] "path from the current working directory where the .ty files are located, provide a comma delimited list").action(ArgAction::Append).value_parser(value_parser!(PathBuf)).value_delimiter(',')
    );
    let link_o = Command::new(LINK).about("generates an executable from .o files").arg(arg!([name] "output name of the binary")).arg(
        arg!(-o --objects <FILES> "path from the current working directory where the .o files are located, provide a comma delimited list").action(ArgAction::Append).value_parser(value_parser!(PathBuf)).value_delimiter(','));
    let matches = Command::new(TY)
        .bin_name(TY)
        .arg_required_else_help(true)
        .arg(arg!(-v --version "gets the current version of ty"))
        .subcommand(sub_o)
        .subcommand(link_o)
        .get_matches();

    if matches.get_flag("version") {
        println!("version: {}", 0.1f32);
    }
    if let Some(obj) = matches.subcommand_matches(OBJ) {
        obj_command(obj);
    }
    if let Some(link) = matches.subcommand_matches(LINK) {
        link_command(link);
    }
}

fn obj_command(m: &ArgMatches) {
    let pre = m.get_many::<PathBuf>("files");
    if pre.is_none() {
        eprintln!("expected at least one file.\n`ty obj [files]`.\ntry `ty obj --help`");
        exit(1);
    }
    let files: Vec<_> = pre.unwrap().collect();
    for ele in files.into_iter() {
        objmaker::from_file(ele);
    }
}

fn link_command(m: &ArgMatches) {
    let output = m.get_one::<String>("name");
    if output.is_none() {
        eprintln!("expected name of output.\n`ty link [name] -o [files]`.\ntry `ty link --help`");
        exit(1);
    }
    let pre = m.get_many::<PathBuf>("objects");
    if pre.is_none() {
        eprintln!("expected at least one file.\nty link [name] -o [files].\ntry `ty link --help`");
        exit(1);
    }
    let files: Vec<&PathBuf> = pre.unwrap().collect();
    let mut outname = PathBuf::new();
    outname.push("target");
    outname.push(output.unwrap());
    linker::link(files, &outname);
}
