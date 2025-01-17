use std::cmp;

use types::*;

pub struct LayoutBuilder {}

#[derive(Debug, Clone)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

pub struct StructLayout {
    pub layout: Layout,
    pub offsets: Vec<(String, usize)>,
}

pub enum Container {
    Simple(Layout),
    Struct(StructLayout),
}

impl Container {
    pub fn get_layout(&self) -> Layout {
        match self {
            Container::Simple(x) => x,
            Container::Struct(x) => x.layout,
        }
    }
}

impl Layout {
    pub fn extend(&self, next: Self) -> (Self, usize) {
        let new_align = cmp::max(self.align, next.align);
        let offset = self.size_rounded_up_to_custom_align(next.align);

        let new_size = offset + next.size;

        return (
            Layout {
                size: new_size,
                align: new_align,
            },
            offset,
        );
    }
    pub fn size_rounded_up_to_custom_align(&self, align: usize) -> usize {
        let align_m1 = align - 1;
        let size_rounded_up = (self.size + align_m1) & !align_m1;
        return size_rounded_up;
    }
    pub fn pad_to_align(&self) -> Layout {
        let new_size = self.size_rounded_up_to_custom_align(self.align);

        return Layout {
            size: new_size,
            align: self.align,
        };
    }
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {}
    }
    pub fn type_layout(&self, ty: &Ty) -> Container {
        match ty {
            Ty::I8 | Ty::U8 | Ty::Bool | Ty::Char => simple!(1, 1),
            Ty::I16 | Ty::U16 => simple!(2, 2),
            Ty::I32 | Ty::U32 | Ty::F32 => simple!(4, 4),
            Ty::I64 | Ty::U64 | Ty::F64 => simple!(8, 8),
            Ty::I128 | Ty::U128 | Ty::F128 | Ty::D128 => simple!(16, 16),
            Ty::Void | Ty::Never => simple!(1, 0),
            Ty::String => simple!(1, 1),
            Ty::Array(inner) => self.type_layout(inner),
            Ty::Struct(fields) => Container::Struct(self.struct_layout(&fields.1)),
            _ => panic!("unhandled_type_layout"), // Default alignment and size for other types
        }
    }
    pub fn struct_layout(&self, fields: &[(String, Ty)]) -> StructLayout {
        let mut field_layouts = vec![];
        for (name, field_ty) in fields {
            let layout = self.type_layout(field_ty).get_layout();
            field_layouts.push((name.clone(), layout));
        }
        return repr_c(&field_layouts);
    }
}

pub fn repr_c(fields: &[(String, Layout)]) -> StructLayout {
    let mut offsets = Vec::new();
    let mut layout = Layout { size: 0, align: 1 };
    for field in fields {
        let (new_layout, offset) = layout.extend(field.1.clone());
        layout = new_layout;
        offsets.push((field.0.clone(), offset));
    }
    return StructLayout {
        layout: layout.pad_to_align(),
        offsets,
    };
}

#[macro_export]
macro_rules! pointer_simple {
    ($size:expr, $align:expr) => {
        Layout::PointerSimple(SimpleLayout {
            size: $size,
            align: $align,
        })
    };
}

#[macro_export]
macro_rules! simple {
    ($size:expr, $align:expr) => {
        Container::Simple(Layout {
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
