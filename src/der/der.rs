use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use super::*;

/// The base trait for DER
///
/// Every type implementing this trait may be serialized
///
/// Note that only `der_intermediate()` and `der_from_intermediate()` need a meaningful
/// implementation. `der_encode_content()`, `der_decode_content()`, `der_universal_tag()` and
/// `der_content()` are only used in the standard implementation of the intermediate functions.
///
/// # Example Implementation #
/// ```
/// # use eagre_asn1::der::*;
/// # use std::io::{self, Read, Write};
///
/// # struct Null;
/// impl DER for Null {
///    fn der_universal_tag() -> UniversalTag {
///        UniversalTag::Null
///    }
///
///    fn der_content() -> ContentType {
///        ContentType::Primitive
///    }
///
///    fn der_encode_content(&self, _: &mut dyn Write) -> io::Result<()> {
///        Ok(())
///    }
///
///    fn der_decode_content(_: &mut dyn Read, length: usize) -> io::Result<Self> {
///        if length != 0 {
///            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Null Type with size bigger than zero"));
///        }
///        Ok(Null)
///    }
/// }
/// ```
pub trait DER: Sized {
    /// Return universal tag of this type
    fn der_universal_tag() -> UniversalTag;
    /// Return content type of this type
    fn der_content() -> ContentType;
    /// Encode the content octets
    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()>;
    /// Decode the content octets
    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self>;
    /// Create Intermediate from this object
    fn der_intermediate(&self) -> io::Result<Intermediate> {
        let mut buf = vec![];
        self.der_encode_content(&mut buf)?;
        Ok(Intermediate::new(
            Class::Universal,
            Self::der_content(),
            Self::der_universal_tag() as u32,
        )
        .with_content(buf))
    }
    /// Fully encode into stream ( tag bytes + length bytes + content bytes )
    fn der_encode(&self, w: &mut dyn Write) -> io::Result<()> {
        self.der_intermediate()?.encode(w)?;
        Ok(())
    }
    /// Return fully encoded bytes (wrapper for der_encode() for easier use)
    fn der_bytes(&self) -> io::Result<Vec<u8>> {
        let mut stream = Vec::new();
        self.der_encode(&mut stream)?;
        Ok(stream)
    }
    /// Create object from Intermediate
    fn der_from_intermediate(i: Intermediate) -> io::Result<Self> {
        let length = i.content.len();
        let mut stream = io::Cursor::new(i.content);
        Self::der_decode_content(&mut stream, length)
    }
    /// Create object from stream
    fn der_decode(r: &mut dyn Read) -> io::Result<Self> {
        let i = Intermediate::decode(r)?;
        Self::der_from_intermediate(i)
    }
    /// Create object from bytes
    fn der_from_bytes(bytes: Vec<u8>) -> io::Result<Self> {
        let mut stream = io::Cursor::new(bytes);
        Self::der_decode(&mut stream)
    }
}

/// FooBar Cool
impl DER for bool {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Boolean
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            &true => w.write_u8(0xFF)?,
            &false => w.write_u8(0x00)?,
        }
        Ok(())
    }

    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        if length != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "boolean value longer that 1 octet",
            ));
        }
        Ok(match r.read_u8()? {
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

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        let mut bytes = Vec::new();
        bytes.write_i32::<BigEndian>(self.clone())?;
        let i = 0;
        loop {
            if bytes[i] == 0
                && i != (bytes.len() - 1)
                && (bytes[i + 1] == 0 || bytes[i + 1] & 0x80 == 0)
            {
                bytes.remove(i);
            } else if bytes[i] == 0xff
                && i != (bytes.len() - 1)
                && (bytes[i + 1] == 0xff || bytes[i + 1] & 0x80 == 0x80)
            {
                bytes.remove(i);
            } else {
                break;
            }
        }
        w.write(&bytes)?;
        Ok(())
    }

    #[allow(overflowing_literals)]
    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = Vec::new();
        encoded.read_to_end(&mut buffer)?;
        let mut value = 0;
        let mut i = buffer.len();
        if i == 0 {
            // Afl found
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Integer with zero content octets",
            ));
        }
        let fb = buffer[0];
        if fb & 0x80 == 0x80 {
            value = 0xffffffff;
        }
        for byte in buffer {
            i -= 1;
            if i > 3 {
                // i32 can only handle 4 bytes
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Trying to decode too big integer",
                ));
            }
            if fb & 0x80 == 0x80 {
                value = value & !(0xff << i * 8);
            }
            //          |                                     |
            //          V TODO: Something is not working here V
            value = value | (byte as i32) << i * 8;
        }
        // Afl found
        /*if fb & 0x80 == 0x80 {
            value = -(!value + 1);
        }*/
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

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write(self.as_bytes())?;
        Ok(())
    }

    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = String::new();
        encoded.read_to_string(&mut buffer)?;
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

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write(self.as_bytes())?;
        Ok(())
    }

    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut buffer = String::new();
        encoded.read_to_string(&mut buffer)?;
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

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        for item in self.iter() {
            item.der_encode(w)?;
        }
        Ok(())
    }

    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        let mut encoded = r.take(length as u64);
        let mut vector = Vec::new();
        while encoded.limit() > 0 {
            vector.push(T::der_decode(&mut encoded)?);
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

    fn der_encode_content(&self, w: &mut dyn Write) -> io::Result<()> {
        w.write(self)?;
        Ok(())
    }

    fn der_decode_content(r: &mut dyn Read, length: usize) -> io::Result<Self> {
        let mut buffer = Vec::new();
        r.take(length as u64).read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
