use std::io::{self, Write, Read};
use byteorder::{WriteBytesExt, BigEndian, ReadBytesExt};

use super::*;

pub trait DER : Sized {
    fn der_universal_tag() -> UniversalTag;
    fn der_content() -> ContentType;
    fn der_encode_content(&self, w: &mut Write) -> io::Result<()>;
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self>;
    fn der_intermediate(&self) -> io::Result<Intermediate> {
        let mut buf = vec!();
        try!(self.der_encode_content(&mut buf));
        Ok(Intermediate::new(Class::Universal, Self::der_content(), Self::der_universal_tag() as u32)
            .with_content(buf))
    }
    fn der_encode(&self, w: &mut Write) -> io::Result<()> {
        try!(try!(self.der_intermediate()).encode(w));
        Ok(())
    }
    fn der_bytes(&self) -> io::Result<Vec<u8>> {
        let mut stream = Vec::new();
        try!(self.der_encode(&mut stream));
        Ok(stream)
    }
    fn der_from_intermediate(i: Intermediate) -> io::Result<Self> {
        let length = i.content.len();
        let mut stream = io::Cursor::new(i.content);
        Self::der_decode_content(&mut stream, length)
    }
    fn der_decode(r: &mut Read) -> io::Result<Self> {
        let i = try!(Intermediate::decode(r));
        Self::der_from_intermediate(i)
    }
    fn der_from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        let mut stream = io::Cursor::new(bytes);
        Self::der_decode(&mut stream)
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
            if i > 3 { // i32 can only handle 4 bytes
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Trying to decode too big integer"));
            }
            if fb & 0x80 == 0x80 {
                value = value & !(0xff << i * 8);
            }
//          |                                     |
//          V TODO: Something is not working here V
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
