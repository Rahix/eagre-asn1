use std::io::{self, Write, Read};

use byteorder::{WriteBytesExt, BigEndian};

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

#[derive(Clone, Copy)]
pub enum Class {
    Universal = 0,
    Application = 1,
    ContextSpecific = 2,
    Private = 3,
}

#[derive(Clone, Copy)]
pub enum ContentType {
    Primitive = 0,
    Constructed = 1,
}

pub fn der_encode_tag_bytes(tag: u32, class: Class, content: ContentType, w: &mut Write) -> io::Result<()> {
    let first_byte = (class as u8) << 6 | (content as u8) << 5 | if tag < 0x1F {tag as u8} else {0x1F};
    try!(w.write_u8(first_byte));
    if tag > 0x1E {
        let mut bytes = 0;
        let mut tag2 = tag;
        while tag2 > 0 {bytes+=1;tag2>>=7}
        for i in (0..bytes).rev() {
            try!(w.write_u8(((tag >> i*7) & 0x7F) as u8 | if i != 0 {0x80} else {0x00}));
        }
    }
    Ok(())
}

pub fn der_encode_length_bytes(length: usize, w: &mut Write) -> io::Result<()> {
    if length < 0x80 {
        try!(w.write_u8(length as u8));
    } else {
        let mut length_bytes = 0;
        let mut l2 = length;
        while l2 > 0 {length_bytes+=1;l2>>=8}
        try!(w.write_u8(0x80 | length_bytes));
        for i in (0..length_bytes).rev() {
            try!(w.write_u8((length >> i*8) as u8 & 0xFF));
        }
    }
    Ok(())
}

pub trait DEREncodeable {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()>;
    fn der_universal_tag() -> UniversalTag;
    fn der_content() -> ContentType;
    fn der_encode(&self, w: &mut Write) -> io::Result<()> {
        try!(der_encode_tag_bytes(Self::der_universal_tag() as u32, Class::Universal, Self::der_content(), w));
        let mut content = ::std::io::Cursor::new(Vec::<u8>::new());
        try!(self.der_encode_content(&mut content));
        let content = content.into_inner();
        try!(der_encode_length_bytes(content.len(), w));
        try!(w.write(&content));
        Ok(())
    }
}

impl DEREncodeable for bool {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        match self {
            &true => try!(w.write_u8(0xFF)),
            &false => try!(w.write_u8(0x00)),
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

impl DEREncodeable for i32 {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write_i32::<BigEndian>(self.clone()));
        Ok(())
    }

    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Integer
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }
}

impl DEREncodeable for String {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write(self.as_bytes()));
        Ok(())
    }

    fn der_universal_tag() -> UniversalTag {
        UniversalTag::UTF8String
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }
}

impl<'a> DEREncodeable for &'a str {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write(self.as_bytes()));
        Ok(())
    }

    fn der_universal_tag() -> UniversalTag {
        UniversalTag::UTF8String
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }
}

impl<T: DEREncodeable> DEREncodeable for Vec<T> {
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        for item in self.iter() {
            try!(item.der_encode(w));
        }
        Ok(())
    }

    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Sequence
    }

    fn der_content() -> ContentType {
        ContentType::Constructed
    }
}
