#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
pub fn make_binop(thing: u64, small: char, big: u64) -> BinOp {
    return BinOp { thing, small, big };
}

pub struct BinOp {
    thing: u64,
    small: char,
    big: u64,
}
