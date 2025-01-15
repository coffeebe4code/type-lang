use std::cmp;

use types::*;

pub struct LayoutBuilder {}

pub struct Layout {
    pub size: usize,
    pub align: usize,
    pub offsets: Option<Vec<(String, u32)>>,
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {}
    }
    pub fn type_layout(&self, ty: &Ty) -> Layout {
        match ty {
            Ty::I8 | Ty::U8 | Ty::Bool => simple!(1, 1),
            Ty::I16 | Ty::U16 => simple!(2, 2),
            Ty::I32 | Ty::U32 | Ty::F32 => simple!(4, 4),
            Ty::I64 | Ty::U64 | Ty::F64 => simple!(8, 8),
            Ty::I128 | Ty::U128 | Ty::F128 | Ty::D128 => simple!(16, 16),
            Ty::Void | Ty::Never => simple!(1, 0),
            Ty::String => simple!(8, 8),
            Ty::Array(inner) => {
                let layout = self.type_layout(inner);
            }
            _ => panic!("unhandled_type_layout"), // Default alignment and size for other types
        }
    }
    pub fn struct_layout(&self, fields: &[(String, Ty)]) -> Layout {
        let mut offset = 0;
        let mut max_alignment = 1;
        let mut new_order = vec![];

        for (val, field_ty) in fields {
            let layout = self.type_layout(field_ty);
            let alignment = layout.align;
            let size = layout.size;
            max_alignment = cmp::max(max_alignment, alignment);

            if offset % alignment != 0 {
                offset += alignment - (offset % alignment);
            }

            offset += size;
        }

        // Align the total size to the max alignment of the struct
        if offset % max_alignment != 0 {
            offset += max_alignment - (offset % max_alignment);
        }

        Layout {
            align: max_alignment,
            size: 0,
            offsets: new_order,
        }
    }
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
        Layout {
            size: $size,
            align: $align,
            offsets: None,
        }
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
