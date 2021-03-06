use crate::der::*;
use std::io::{self, Read, Write};

/// Asn1 Null Type
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Null;

impl DER for Null {
    fn der_universal_tag() -> UniversalTag {
        UniversalTag::Null
    }

    fn der_content() -> ContentType {
        ContentType::Primitive
    }

    fn der_encode_content(&self, _: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }

    fn der_decode_content(_: &mut dyn Read, length: usize) -> io::Result<Self> {
        if length != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Null Type with size bigger than zero",
            ));
        }
        Ok(Null)
    }
}

#[test]
fn encode_null() {
    let mut stream = Vec::new();
    Null.der_encode(&mut stream).unwrap();
    assert_eq!(&0x05, stream.get(0).unwrap());
    assert_eq!(&0x00, stream.get(1).unwrap());
}
