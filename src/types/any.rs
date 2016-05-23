use std::io::{self, Write, Read};
use der::*;

pub struct Any(Vec<u8>);

impl DER for Any {
    fn der_universal_tag() -> UniversalTag {
        unimplemented!()
    }

    fn der_content() -> ContentType {
        unimplemented!()
    }

    fn der_encode(&self, w: &mut Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_encode_content(&self, _: &mut Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_decode(r: &mut Read) -> io::Result<Self> {
        unimplemented!()
    }

    fn der_decode_content(_: &mut Read, length: usize) -> io::Result<Self> {
        unimplemented!()
    }
}

/*#[test]
fn serialize_any() {
    let buf = 42.der_bytes().unwrap();
    let any = Any(buf);
    let mut stream = io::Cursor::new(any.der_bytes().unwrap());
    let Any(vec) = Any::der_decode(&mut stream).unwrap();
    let value = i32::der_decode(&mut io::Cursor::new(vec)).unwrap();
    assert_eq!(value, 42);
}*/
