use crate::der::*;
use std::io::{self, Read, Write};

/// Asn1 Any Type
///
/// Any encoded "Any" type will not be visible in the encoded bytes.
/// It is a helper type, which should be used in case the actual type is not known yet.
///
/// # Example
///
/// ```
/// # use eagre_asn1::types::*;
/// # use eagre_asn1::der::DER;
///
/// let any = Any::new("I am a random string".to_string()).unwrap();
/// let encoded = any.der_bytes().unwrap();
/// // Send to far away planet
/// let decoded = Any::der_from_bytes(encoded).unwrap();
/// assert_eq!("I am a random string", &decoded.resolve::<String>().unwrap());
/// ```
#[derive(Debug)]
pub struct Any {
    i: Intermediate,
}

impl Any {
    /// Create a new Any object from a inner value
    pub fn new<T: DER>(val: T) -> io::Result<Any> {
        Ok(Any {
            i: val.der_intermediate()?,
        })
    }

    /// Resolve the inner value of an Any object
    pub fn resolve<T: DER>(&self) -> io::Result<T> {
        <T>::der_from_intermediate(self.i.clone())
    }
}

impl DER for Any {
    fn der_universal_tag() -> UniversalTag {
        unimplemented!() // Any is a hidden type and does not have a Universal Tag
    }

    fn der_content() -> ContentType {
        unimplemented!() // Same as universal tag
    }

    fn der_encode_content(&self, _: &mut dyn Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_decode_content(_: &mut dyn Read, _: usize) -> io::Result<Self> {
        unimplemented!()
    }

    fn der_intermediate(&self) -> io::Result<Intermediate> {
        Ok(self.i.clone())
    }

    fn der_from_intermediate(i: Intermediate) -> io::Result<Self> {
        Ok(Any { i: i })
    }
}

#[test]
fn serialize_any() {
    let val = Any::new(31415).unwrap();
    let decoded = Any::der_from_bytes(val.der_bytes().unwrap()).unwrap();
    assert_eq!(31415, decoded.resolve().unwrap());
}
