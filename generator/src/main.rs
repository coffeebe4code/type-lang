use cranelift_codegen_meta::isa::*;
use cranelift_codegen_meta::*;
use std::error::Error;

fn main() -> () {
    return generate(
        Isa::all(),
        "../cranelift/isa/src/out",
        "../cranelift/isa/src/isle",
    )
    .unwrap();
}
