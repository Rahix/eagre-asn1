/// The Intermediate Type
pub mod intermediate;
/// Length encoding/decoding
pub mod length;
/// Tag encoding/decoding
pub mod tag;

/// DER Trait
pub mod der;
#[doc(hidden)]
#[macro_use]
pub mod macros;

#[cfg(test)]
mod test;

pub use self::der::DER;
pub use self::intermediate::Intermediate;
pub use self::length::*;
pub use self::tag::*;

/// DER Universal Tag Values
#[derive(Debug, Copy, Clone)]
pub enum UniversalTag {
    /// End of Content
    EOC = 0,
    /// Boolean
    Boolean = 1,
    /// Integer
    Integer = 2,
    /// BitString
    BitString = 3,
    /// OctetString
    OctetString = 4,
    /// Null
    Null = 5,
    /// ObjectIdentifier
    ObjectIdentifier = 6,
    /// ObjectDescriptor
    ObjectDescriptor = 7,
    /// External
    External = 8,
    /// Real
    Real = 9,
    /// Enumerated
    Enumerated = 10,
    /// EmbeddedPDV
    EmbeddedPDV = 11,
    /// UTF8String
    UTF8String = 12,
    /// RelativeOID
    RelativeOID = 13,
    /// Reserved, Unused
    Reserved01 = 14,
    /// Reserved, Unused
    Reserved02 = 15,
    /// Sequence
    Sequence = 16,
    /// Set
    Set = 17,
    /// NumericString
    NumericString = 18,
    /// PrintableString
    PrintableString = 19,
    /// T61String
    T61String = 20,
    /// VideotexString
    VideotexString = 21,
    /// IA5String
    IA5String = 22,
    /// UTCTime
    UTCTime = 23,
    /// GeneralizedTime
    GeneralizedTime = 24,
    /// GraphicString
    GraphicString = 25,
    /// VisibleString
    VisibleString = 26,
    /// GeneralString
    GeneralString = 27,
    /// UniversalString
    UniversalString = 28,
    /// CharacterString
    CharacterString = 29,
    /// BMPString
    BMPString = 30,
}

/// DER Class Values
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Class {
    /// Universal
    Universal = 0,
    /// Application
    Application = 1,
    /// ContextSpecific
    ContextSpecific = 2,
    /// Private
    Private = 3,
}

/// DER ContentType Values
#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    /// Primitive
    Primitive = 0,
    /// Constructed
    Constructed = 1,
}
