#[derive(Debug)]
pub enum Type {
    I64(i64),
    U64(u64),
    F64(f64),
    Struct(Vec<(usize, Box<Type>)>),
    MutRef(Box<Type>),
    ConstRef(Box<Type>),
    Signature(Vec<Box<Type>>),
}
