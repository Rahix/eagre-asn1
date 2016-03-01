use std::io::Write;

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

#[derive(PartialEq, Clone)]
pub enum Class {
    Universal = 0,
    Application = 1,
    ContextSpecific = 2,
    Private = 3,
}

pub enum ContentType {
    Primitive = 0,
    Constructed = 1,
}

pub fn der_tag_bytes<W: Write>(tag: u32, class: Class, pc: PC, stream: &mut W) -> ::std::io::Result<()> {
/*    let mut first_octet = ((class.clone() as u8) << 6) + ((pc as u8) << 5);
    if (class == Class::Universal) || (tag < 0x1F) {
        first_octet = first_octet + tag as u8;
        try!(stream.write(&[first_octet]));
    } else {
        first_octet = first_octet + 0x1F;
        try!(stream.write(&[first_octet]));
        let mut tag = tag;
        while tag > 0 {
            let mut octet = ((tag & 0x7F) + 0x80) as u8;
            tag = tag >> 7;
            if tag <= 0 {
                octet = octet - 0x80;
            }
            try!(stream.write(&[octet]));
        }
    }
    Ok(())*/
}

pub fn der_length_bytes<W: Write>(length: u32, stream: &mut W) -> ::std::io::Result<()> {
/*    if length < 0x80 {
        try!(stream.write(&[length as u8]));
    } else {
        // Long form
        let mut bytes = vec!();
        let mut length = length;
        while length > 0 {
            bytes.push((length & 0xFF) as u8);
            length = length >> 8;
        }
        bytes.reverse();
        try!(stream.write(&[0x80+bytes.len() as u8]));
        try!(stream.write(&bytes));
    }
    Ok(())*/
}

pub trait DEREncodeable {
    fn der_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()>;
    fn der_universal_tag() -> UniversalTag;
    fn der_content() -> ContentType;
    //fn der_encode<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()>;
}

impl DEREncodeable for bool {
    fn der_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        match self {
            &true => {try!(stream.write(&[0xFF as u8]));},
            &false => {try!(stream.write(&[0x00 as u8]));},
        }
        Ok(())
    }

    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Boolean
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }
}
