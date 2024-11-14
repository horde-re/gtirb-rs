use crate::byte_interval::ByteInterval;

use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Section {
    pub uuid: Uuid,
    pub name: String,
    pub byte_intervals: Vec<ByteInterval>,
    pub section_flags: Vec<SectionFlag>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum SectionFlag {
    Undefined,
    Readable,
    Writable,
    Executable,
    Loaded,
    Initialized,
    ThreadLocal,
}
