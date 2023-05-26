#[derive(Clone, Hash)]
/// Flags group `s390x`.
pub struct Flags {
    bytes: [u8; 1],
}
impl Flags {
    /// Create flags s390x settings group.
    #[allow(unused_variables)]
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("s390x");
        let mut s390x = Self { bytes: [0; 1] };
        debug_assert_eq!(bvec.len(), 1);
        s390x.bytes[0..1].copy_from_slice(&bvec);
        s390x
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> {
        let mut bytes = [0; 1];
        bytes.copy_from_slice(&self.bytes[0..1]);
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None,
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)),
                _ => None
            };
            Some(Value{ name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] })
        })
    }
}
/// User-defined settings.
#[allow(dead_code)]
impl Flags {
    /// Get a view of the boolean predicates.
    pub fn predicate_view(&self) -> crate::settings::PredicateView {
        crate::settings::PredicateView::new(&self.bytes[0..])
    }
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0
    }
    /// Has Miscellaneous-Instruction-Extensions Facility 2 support.
    pub fn has_mie2(&self) -> bool {
        self.numbered_predicate(0)
    }
    /// Has Vector-Enhancements Facility 2 support.
    pub fn has_vxrs_ext2(&self) -> bool {
        self.numbered_predicate(1)
    }
}
static DESCRIPTORS: [detail::Descriptor; 4] = [
    detail::Descriptor {
        name: "has_mie2",
        description: "Has Miscellaneous-Instruction-Extensions Facility 2 support.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_vxrs_ext2",
        description: "Has Vector-Enhancements Facility 2 support.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "arch13",
        description: "Thirteenth Edition of the z/Architecture.",
        offset: 0,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "z15",
        description: "IBM z15 processor.",
        offset: 1,
        detail: detail::Detail::Preset,
    },
];
static ENUMERATORS: [&str; 0] = [
];
static HASH_TABLE: [u16; 8] = [
    0,
    0xffff,
    1,
    0xffff,
    3,
    2,
    0xffff,
    0xffff,
];
static PRESETS: [(u8, u8); 2] = [
    // arch13: has_mie2, has_vxrs_ext2
    (0b00000011, 0b00000011),
    // z15: has_mie2, has_vxrs_ext2
    (0b00000011, 0b00000011),
];
static TEMPLATE: detail::Template = detail::Template {
    name: "s390x",
    descriptors: &DESCRIPTORS,
    enumerators: &ENUMERATORS,
    hash_table: &HASH_TABLE,
    defaults: &[0x00],
    presets: &PRESETS,
};
/// Create a `settings::Builder` for the s390x settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE)
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[s390x]")?;
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?;
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
