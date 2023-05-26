#[derive(Clone, Hash)]
/// Flags group `riscv64`.
pub struct Flags {
    bytes: [u8; 4],
}
impl Flags {
    /// Create flags riscv64 settings group.
    #[allow(unused_variables)]
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("riscv64");
        let mut riscv64 = Self { bytes: [0; 4] };
        debug_assert_eq!(bvec.len(), 4);
        riscv64.bytes[0..4].copy_from_slice(&bvec);
        riscv64
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.bytes[0..4]);
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
    /// has extension M?
    pub fn has_m(&self) -> bool {
        self.numbered_predicate(0)
    }
    /// has extension A?
    pub fn has_a(&self) -> bool {
        self.numbered_predicate(1)
    }
    /// has extension F?
    pub fn has_f(&self) -> bool {
        self.numbered_predicate(2)
    }
    /// has extension D?
    pub fn has_d(&self) -> bool {
        self.numbered_predicate(3)
    }
    /// has extension V?
    pub fn has_v(&self) -> bool {
        self.numbered_predicate(4)
    }
    /// has extension C?
    pub fn has_c(&self) -> bool {
        self.numbered_predicate(5)
    }
    /// has extension zbkb?
    /// Zbkb: Bit-manipulation for Cryptography
    pub fn has_zbkb(&self) -> bool {
        self.numbered_predicate(6)
    }
    /// has extension zba?
    /// Zba: Address Generation
    pub fn has_zba(&self) -> bool {
        self.numbered_predicate(7)
    }
    /// has extension zbb?
    /// Zbb: Basic bit-manipulation
    pub fn has_zbb(&self) -> bool {
        self.numbered_predicate(8)
    }
    /// has extension zbc?
    /// Zbc: Carry-less multiplication
    pub fn has_zbc(&self) -> bool {
        self.numbered_predicate(9)
    }
    /// has extension zbs?
    /// Zbs: Single-bit instructions
    pub fn has_zbs(&self) -> bool {
        self.numbered_predicate(10)
    }
    /// has extension zicsr?
    /// Zicsr: Control and Status Register (CSR) Instructions
    pub fn has_zicsr(&self) -> bool {
        self.numbered_predicate(11)
    }
    /// has extension zifencei?
    /// Zifencei: Instruction-Fetch Fence
    pub fn has_zifencei(&self) -> bool {
        self.numbered_predicate(12)
    }
    /// has extension Zvl32b?
    /// Zvl32b: Vector register has a minimum of 32 bits
    pub fn has_zvl32b(&self) -> bool {
        self.numbered_predicate(13)
    }
    /// has extension Zvl64b?
    /// Zvl64b: Vector register has a minimum of 64 bits
    pub fn has_zvl64b(&self) -> bool {
        self.numbered_predicate(14)
    }
    /// has extension Zvl128b?
    /// Zvl128b: Vector register has a minimum of 128 bits
    pub fn has_zvl128b(&self) -> bool {
        self.numbered_predicate(15)
    }
    /// has extension Zvl256b?
    /// Zvl256b: Vector register has a minimum of 256 bits
    pub fn has_zvl256b(&self) -> bool {
        self.numbered_predicate(16)
    }
    /// has extension Zvl512b?
    /// Zvl512b: Vector register has a minimum of 512 bits
    pub fn has_zvl512b(&self) -> bool {
        self.numbered_predicate(17)
    }
    /// has extension Zvl1024b?
    /// Zvl1024b: Vector register has a minimum of 1024 bits
    pub fn has_zvl1024b(&self) -> bool {
        self.numbered_predicate(18)
    }
    /// has extension Zvl2048b?
    /// Zvl2048b: Vector register has a minimum of 2048 bits
    pub fn has_zvl2048b(&self) -> bool {
        self.numbered_predicate(19)
    }
    /// has extension Zvl4096b?
    /// Zvl4096b: Vector register has a minimum of 4096 bits
    pub fn has_zvl4096b(&self) -> bool {
        self.numbered_predicate(20)
    }
    /// has extension Zvl8192b?
    /// Zvl8192b: Vector register has a minimum of 8192 bits
    pub fn has_zvl8192b(&self) -> bool {
        self.numbered_predicate(21)
    }
    /// has extension Zvl16384b?
    /// Zvl16384b: Vector register has a minimum of 16384 bits
    pub fn has_zvl16384b(&self) -> bool {
        self.numbered_predicate(22)
    }
    /// has extension Zvl32768b?
    /// Zvl32768b: Vector register has a minimum of 32768 bits
    pub fn has_zvl32768b(&self) -> bool {
        self.numbered_predicate(23)
    }
    /// has extension Zvl65536b?
    /// Zvl65536b: Vector register has a minimum of 65536 bits
    pub fn has_zvl65536b(&self) -> bool {
        self.numbered_predicate(24)
    }
}
static DESCRIPTORS: [detail::Descriptor; 37] = [
    detail::Descriptor {
        name: "has_m",
        description: "has extension M?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_a",
        description: "has extension A?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_f",
        description: "has extension F?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_d",
        description: "has extension D?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_v",
        description: "has extension V?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_c",
        description: "has extension C?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_zbkb",
        description: "has extension zbkb?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "has_zba",
        description: "has extension zba?",
        offset: 0,
        detail: detail::Detail::Bool { bit: 7 },
    },
    detail::Descriptor {
        name: "has_zbb",
        description: "has extension zbb?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_zbc",
        description: "has extension zbc?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_zbs",
        description: "has extension zbs?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_zicsr",
        description: "has extension zicsr?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_zifencei",
        description: "has extension zifencei?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_zvl32b",
        description: "has extension Zvl32b?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_zvl64b",
        description: "has extension Zvl64b?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "has_zvl128b",
        description: "has extension Zvl128b?",
        offset: 1,
        detail: detail::Detail::Bool { bit: 7 },
    },
    detail::Descriptor {
        name: "has_zvl256b",
        description: "has extension Zvl256b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_zvl512b",
        description: "has extension Zvl512b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_zvl1024b",
        description: "has extension Zvl1024b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_zvl2048b",
        description: "has extension Zvl2048b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_zvl4096b",
        description: "has extension Zvl4096b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_zvl8192b",
        description: "has extension Zvl8192b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_zvl16384b",
        description: "has extension Zvl16384b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "has_zvl32768b",
        description: "has extension Zvl32768b?",
        offset: 2,
        detail: detail::Detail::Bool { bit: 7 },
    },
    detail::Descriptor {
        name: "has_zvl65536b",
        description: "has extension Zvl65536b?",
        offset: 3,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "zvl32b",
        description: "Has a vector register size of at least 32 bits",
        offset: 0,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl64b",
        description: "Has a vector register size of at least 64 bits",
        offset: 4,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl128b",
        description: "Has a vector register size of at least 128 bits",
        offset: 8,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl256b",
        description: "Has a vector register size of at least 256 bits",
        offset: 12,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl512b",
        description: "Has a vector register size of at least 512 bits",
        offset: 16,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl1024b",
        description: "Has a vector register size of at least 1024 bits",
        offset: 20,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl2048b",
        description: "Has a vector register size of at least 2048 bits",
        offset: 24,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl4096b",
        description: "Has a vector register size of at least 4096 bits",
        offset: 28,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl8192b",
        description: "Has a vector register size of at least 8192 bits",
        offset: 32,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl16384b",
        description: "Has a vector register size of at least 16384 bits",
        offset: 36,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl32768b",
        description: "Has a vector register size of at least 32768 bits",
        offset: 40,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "zvl65536b",
        description: "Has a vector register size of at least 65536 bits",
        offset: 44,
        detail: detail::Detail::Preset,
    },
];
static ENUMERATORS: [&str; 0] = [
];
static HASH_TABLE: [u16; 64] = [
    19,
    13,
    17,
    0xffff,
    34,
    36,
    23,
    0xffff,
    8,
    9,
    0xffff,
    7,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    27,
    28,
    21,
    11,
    29,
    0xffff,
    0xffff,
    4,
    12,
    20,
    0xffff,
    0xffff,
    6,
    15,
    0xffff,
    25,
    0,
    16,
    5,
    24,
    1,
    26,
    0xffff,
    2,
    35,
    3,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    0xffff,
    30,
    18,
    22,
    32,
    0xffff,
    10,
    0xffff,
    0xffff,
    33,
    31,
    14,
    0xffff,
];
static PRESETS: [(u8, u8); 48] = [
    // zvl32b: has_zvl32b
    (0b00000000, 0b00000000),
    (0b00100000, 0b00100000),
    (0b00000000, 0b00000000),
    (0b00000000, 0b00000000),
    // zvl64b: has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b01100000, 0b01100000),
    (0b00000000, 0b00000000),
    (0b00000000, 0b00000000),
    // zvl128b: has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00000000, 0b00000000),
    (0b00000000, 0b00000000),
    // zvl256b: has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00000001, 0b00000001),
    (0b00000000, 0b00000000),
    // zvl512b: has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00000011, 0b00000011),
    (0b00000000, 0b00000000),
    // zvl1024b: has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00000111, 0b00000111),
    (0b00000000, 0b00000000),
    // zvl2048b: has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00001111, 0b00001111),
    (0b00000000, 0b00000000),
    // zvl4096b: has_zvl4096b, has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00011111, 0b00011111),
    (0b00000000, 0b00000000),
    // zvl8192b: has_zvl8192b, has_zvl4096b, has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b00111111, 0b00111111),
    (0b00000000, 0b00000000),
    // zvl16384b: has_zvl16384b, has_zvl8192b, has_zvl4096b, has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b01111111, 0b01111111),
    (0b00000000, 0b00000000),
    // zvl32768b: has_zvl32768b, has_zvl16384b, has_zvl8192b, has_zvl4096b, has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b11111111, 0b11111111),
    (0b00000000, 0b00000000),
    // zvl65536b: has_zvl65536b, has_zvl32768b, has_zvl16384b, has_zvl8192b, has_zvl4096b, has_zvl2048b, has_zvl1024b, has_zvl512b, has_zvl256b, has_zvl128b, has_zvl64b, has_zvl32b
    (0b00000000, 0b00000000),
    (0b11100000, 0b11100000),
    (0b11111111, 0b11111111),
    (0b00000001, 0b00000001),
];
static TEMPLATE: detail::Template = detail::Template {
    name: "riscv64",
    descriptors: &DESCRIPTORS,
    enumerators: &ENUMERATORS,
    hash_table: &HASH_TABLE,
    defaults: &[0x00, 0x00, 0x00, 0x00],
    presets: &PRESETS,
};
/// Create a `settings::Builder` for the riscv64 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE)
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[riscv64]")?;
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
