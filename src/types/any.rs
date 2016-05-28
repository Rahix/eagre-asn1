use std::io::{self, Write, Read};
use der::*;

pub struct Any {
    i: Intermediate,
}

impl Any {
    pub fn new<T: DER>(val: T) -> io::Result<Any> {
        Ok(Any {
            i: try!(val.der_intermediate()),
        })
    }

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

    fn der_encode_content(&self, _: &mut Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_decode_content(_: &mut Read, _: usize) -> io::Result<Self> {
        unimplemented!()
    }

    fn der_intermediate(&self) -> io::Result<Intermediate> {
        Ok(self.i.clone())
    }

    fn der_from_intermediate(i: Intermediate) -> io::Result<Self> {
        Ok(Any {
            i: i,
        })
    }
}

#[test]
fn serialize_any() {
    let val = Any::new(31415).unwrap();
    let decoded = Any::der_from_bytes(val.der_bytes().unwrap()).unwrap();
    assert_eq!(31415, decoded.resolve().unwrap());
}
