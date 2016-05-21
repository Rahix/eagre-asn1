use std::io::{self, Write, Read};
use byteorder::{WriteBytesExt, BigEndian, ReadBytesExt};

use super::*;

pub trait DER : Sized {
    fn der_universal_tag() -> UniversalTag;
    fn der_content() -> ContentType;
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()>;
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self>;
    fn der_encode(&self, w: &mut Write) -> io::Result<()> {
        try!(der_encode_tag_bytes(Self::der_universal_tag() as u32, Class::Universal, Self::der_content(), w));
        let mut content = Vec::<u8>::new();
        try!(self.der_encode_content(&mut content));
        try!(der_encode_length_bytes(content.len(), w));
        try!(w.write(&content));
        Ok(())
    }
    fn der_decode(r: &mut Read) -> io::Result<Self> {
        let (_, tag, class, _) = try!(der_decode_tag_bytes(r));
        if (class == Class::Universal) && (tag != Self::der_universal_tag() as u32) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "trying to decode other type"));
        }
        let (_, length) = try!(der_decode_length_bytes(r));
        Ok(try!(Self::der_decode_content(r, length)))
    }
    fn der_bytes(&self) -> io::Result<Vec<u8>> {
        let mut stream = Vec::new();
        try!(self.der_encode(&mut stream));
        Ok(stream)
    }
}

impl DER for bool {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Boolean
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        match self {
            &true => try!(w.write_u8(0xFF)),
            &false => try!(w.write_u8(0x00)),
        }
        Ok(())
    }

    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        if length != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "boolean value longer that 1 octet"));
        }
        Ok(match try!(r.read_u8()) {
            0x00 => false,
            _ => true,
        })
    }
}

impl DER for i32 {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Integer
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        let mut bytes = Vec::new();
        try!(bytes.write_i32::<BigEndian>(self.clone()));
        let i = 0;
        loop {
            if bytes[i] == 0 && i != (bytes.len() - 1) && (bytes[i+1] == 0 || bytes[i+1] & 0x80 == 0) {
                bytes.remove(i);
            } else if bytes[i] == 0xff && i != (bytes.len() - 1) && (bytes[i+1] == 0xff || bytes[i+1] & 0x80 == 0x80) {
                bytes.remove(i);
            } else {
                break;
            }
        }
        try!(w.write(&bytes));
        Ok(())
    }

    #[allow(overflowing_literals)]
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = Vec::new();
        try!(encoded.read_to_end(&mut buffer));
        let mut value = 0;
        let mut i = buffer.len();
        let fb = buffer[0];
        if fb & 0x80 == 0x80 {
            value = 0xffffffff;
        }
        for byte in buffer {
            i -= 1;
            if fb & 0x80 == 0x80 {
                value = value & !(0xff << i*8);
            }
            value = value | (byte as i32) << i * 8;
        }
        if fb & 0x80 == 0x80 {
            value = -(!value + 1);
        }
        Ok(value)
    }
}

impl DER for String {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::UTF8String
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write(self.as_bytes()));
        Ok(())
    }

    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = String::new();
        try!(encoded.read_to_string(&mut buffer));
        Ok(buffer)
    }
}

impl<'a> DER for &'a str {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::UTF8String
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write(self.as_bytes()));
        Ok(())
    }

    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = String::new();
        try!(encoded.read_to_string(&mut buffer));
        Ok("not_implemented_yet")
    }
}

impl<T: DER> DER for Vec<T> {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Sequence
    }

    fn der_content() -> ContentType {
        ContentType::Constructed
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        for item in self.iter() {
            try!(item.der_encode(w));
        }
        Ok(())
    }

    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut vector = Vec::new();
        while encoded.limit() > 0 {
            vector.push(try!(T::der_decode(&mut encoded)));
        }
        Ok(vector)
    }
}

impl DER for Vec<u8> {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::OctetString
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
        try!(w.write(self));
        Ok(())
    }

    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut buffer = Vec::new();
        try!(r.take(length as u64).read_to_end(&mut buffer));
        Ok(buffer)
    }
}
