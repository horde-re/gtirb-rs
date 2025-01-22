/// Represents an effective address in the program.
///
/// This is a simple wrapper around a u64. It is used to distinguish
/// addresses from other integer values. An address cannot store relative
/// addresses as it cannot contain negative values.
pub type Address = u64;
