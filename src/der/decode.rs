use std::io::{self, Read};
use byteorder::{ReadBytesExt, BigEndian};

use super::*;

pub trait DERDecodeable : Sized {
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self>;
    fn der_decode(r: &mut Read) -> io::Result<Self> {
        let (tag_bytes, tag, class, content) = try!(der_decode_tag_bytes(r));
        let (length_bytes, length) = try!(der_decode_length_bytes(r));
        Ok(try!(Self::der_decode_content(r, length)))
    }
}

impl DERDecodeable for bool {
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        if length != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "boolean value not correctly encoded"));
        }
        Ok(match try!(r.read_u8()) {
            0x00 => false,
            _ => true,
        })
    }
}

impl DERDecodeable for String {
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut buffer = Vec::new();
        for _ in 0..length {
            buffer.push(try!(r.read_u8()));
        }
        Ok(try!(String::from_utf8(buffer).or(Err(io::Error::new(io::ErrorKind::InvalidData, "none utf8 data in utf8 string")))))
    }
}

impl<T: DERDecodeable> DERDecodeable for Vec<T> {
    fn der_decode_content(r: &mut Read, length: usize) -> io::Result<Self> {
        let mut stream = r.take(length as u64);
        let mut vector = Vec::new();
        while stream.limit() > 0 {
            vector.push(try!(T::der_decode(&mut stream)));
        }
        Ok(vector)
    }
}

#[macro_export]
macro_rules! der_decode_sequence {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),+) => {
        impl $crate::der::DERDecodeable for $struct_name {
            fn der_decode_content(r: &mut ::std::io::Read, length: usize) -> ::std::io::Result<$struct_name> {
                use $crate::der::DERDecodeable;
                //let mut stream = r.take(length as u64);
                Ok($struct_name {
                    $($field_name: try!($field_type ::der_decode(r)),
                )+})
            }
        }
    }
}
