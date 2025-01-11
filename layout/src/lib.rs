use types::*;

pub struct LayoutBuilder {}

pub enum Layout {
    Simple(SimpleLayout),
    Container(ContainerLayout),
}

pub trait GetSimpleLayout {
    fn get_layout(&self, layout_builder: &mut LayoutBuilder) -> SimpleLayout;
}

pub trait GetContainerLayout {
    fn get_layout(&self, layout_builder: &mut LayoutBuilder) -> ContainerLayout;
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {}
    }
    //    pub fn get_layout(&self, typetree: &TypeTree) -> Layout {
    //        match val {
    //            Ty::Bool => simple!(1, 1),
    //            Ty::Char => simple!(1, 1),
    //            Ty::String => simple!(64, 8),
    //            Ty::I64 => simple!(64, 8),
    //            Ty::U64 => simple!(64, 8),
    //            Ty::U32 => simple!(32, 4),
    //            Ty::I32 => simple!(32, 4),
    //            Ty::U8 => simple!(1, 1),
    //            Ty::Const(x) => self.get_layout(*x),
    //            Ty::Mut(x) => self.get_layout(*x),
    //            Ty::MutBorrow(_) => simple!(64, 8),
    //            Ty::ReadBorrow(_) => simple!(64, 8),
    //            Ty::Frame(_) => simple!(64, 8),
    //            Ty::Struct(x) => simple!(64, 8),
    //            Ty::Array(_) => simple!(64, 8),
    //            Ty::Tag(_) => simple!(64, 8),
    //            Ty::Error => simple!(64, 8),
    //            Ty::Void => simple!(0, 0),
    //            Ty::Undefined => simple!(0, 0),
    //            Ty::Unknown => simple!(0, 0),
    //            Ty::Never => simple!(0, 0),
    //            _ => panic!("unhandled simple layout"),
    //        }
    //    }
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
    pub size: u32,
    pub align: u8,
    pub offsets: Vec<(String, u32)>,
}
