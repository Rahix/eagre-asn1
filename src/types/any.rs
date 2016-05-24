use std::io::{self, Write, Read};
use der::*;

pub struct Any {
    class: Class,
    content_type: ContentType,
    tag: u32,
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
}
