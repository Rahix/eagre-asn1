use std::io::{self, Write};
use byteorder::{WriteBytesExt, BigEndian};

use super::*;

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

#[macro_export]
macro_rules! der_encode_sequence {
    ($struct_name:ident, $($field_name:ident),+) => {
        impl $crate::der::DEREncodeable for $struct_name {
            fn der_encode_content(&self, w: &mut ::std::io::Write) -> ::std::io::Result<()> {
                $(
                    try!(self.$field_name.der_encode(w));
                )+
                Ok(())
            }

            fn der_universal_tag() -> $crate::der::UniversalTag {
                $crate::der::UniversalTag::Sequence
            }

            fn der_content() -> $crate::der::ContentType {
                $crate::der::ContentType::Constructed
            }
        }
    }
}
