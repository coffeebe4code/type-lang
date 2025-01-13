use types::*;

pub struct LayoutBuilder {}

pub enum Layout {
    Simple(SimpleLayout),
    Container(ContainerLayout),
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {}
    }
    pub fn scalar_layout(&self, ty: &Ty) -> SimpleLayout {
        match ty {
            Ty::Bool => direct!(1, 1),
            Ty::Char => direct!(1, 1),
            Ty::F128 => direct!(128, 16),
            Ty::F64 => direct!(64, 8),
            Ty::F32 => direct!(32, 4),
            Ty::I128 => direct!(128, 16),
            Ty::I64 => direct!(64, 8),
            Ty::I32 => direct!(32, 4),
            Ty::I16 => direct!(2, 2),
            Ty::I8 => direct!(1, 1),
            Ty::U128 => direct!(128, 16),
            Ty::U64 => direct!(64, 8),
            Ty::U32 => direct!(32, 4),
            Ty::U16 => direct!(2, 2),
            Ty::U8 => direct!(1, 1),
            _ => panic!("unhandled scalar layout"),
        }
    }
}

#[macro_export]
macro_rules! PointerSimple {
    ($size:expr, $align:expr) => {
        Layout::PointerSimple(SimpleLayout {
            size: $size,
            align: $align,
        })
    };
}

#[macro_export]
macro_rules! direct {
    ($size:expr, $align:expr) => {
        SimpleLayout {
            size: $size,
            align: $align,
        }
    };
}

#[macro_export]
macro_rules! simple {
    ($size:expr, $align:expr) => {
        Layout::Simple(SimpleLayout {
            size: $size,
            align: $align,
        })
    };
}

#[macro_export]
macro_rules! container {
    ($size:expr, $align:expr, $offsets:expr) => {
        Layout::Container(ContainerLayout {
            size: $size,
            align: $align,
            offsets: $offsets,
        })
    };
}

pub struct SimpleLayout {
    pub size: u16,
    pub align: u8,
}

pub struct ContainerLayout {
    pub layout: SimpleLayout,
    pub offsets: Vec<(String, u32)>,
}
