#[derive(Clone, PartialEq)]
pub struct Section {
    pub uuid: Vec<u8>,

    pub name: String,

    pub byte_intervals: Vec<ByteInterval>,

    pub section_flags: Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum SectionFlag {
    SectionUndefined = 0,
    Readable = 1,
    Writable = 2,
    Executable = 3,
    Loaded = 4,
    Initialized = 5,
    ThreadLocal = 6,
}
impl SectionFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SectionUndefined => "Section_Undefined",
            Self::Readable => "Readable",
            Self::Writable => "Writable",
            Self::Executable => "Executable",
            Self::Loaded => "Loaded",
            Self::Initialized => "Initialized",
            Self::ThreadLocal => "ThreadLocal",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Section_Undefined" => Some(Self::SectionUndefined),
            "Readable" => Some(Self::Readable),
            "Writable" => Some(Self::Writable),
            "Executable" => Some(Self::Executable),
            "Loaded" => Some(Self::Loaded),
            "Initialized" => Some(Self::Initialized),
            "ThreadLocal" => Some(Self::ThreadLocal),
            _ => None,
        }
    }
}
