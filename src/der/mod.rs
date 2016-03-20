pub mod tag;
pub mod length;
#[cfg(test)]
mod test;

pub mod der;
#[macro_use]
pub mod macros;

pub use self::tag::*;
pub use self::length::*;
pub use self::der::DER;

#[derive(Debug)]
pub enum UniversalTag {
    Eoc = 0,
    Boolean = 1,
    Integer = 2,
    BitString = 3,
    OctetString = 4,
    Null = 5,
    ObjectIdentifier = 6,
    ObjectDescriptor = 7,
    External = 8,
    Real = 9,
    Enumerated = 10,
    EmbeddedPDV = 11,
    UTF8String = 12,
    RelativeOID = 13,
    // Reserved = 14,
    // Reserved = 15,
    Sequence = 16,
    Set = 17,
    NumericString = 18,
    PrintableString = 19,
    T61String = 20,
    VideotexString = 21,
    IA5String = 22,
    UTCTime = 23,
    GeneralizedTime = 24,
    GraphicString = 25,
    VisibleString = 26,
    GeneralString = 27,
    UniversalString = 28,
    CharacterString = 29,
    BMPString = 30,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Class {
    Universal = 0,
    Application = 1,
    ContextSpecific = 2,
    Private = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Primitive = 0,
    Constructed = 1,
}
