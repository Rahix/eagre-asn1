// Proof of Concept
#[macro_use]
extern crate eagre_asn1 as asn1;

use asn1::der::DER;

#[derive(Debug, PartialEq)]
struct Foo {
    pub a: i32,
    pub b: bool,
}

impl DER for Foo {
    fn der_universal_tag() -> asn1::der::UniversalTag {
        asn1::der::UniversalTag::Sequence
    }

    fn der_content() -> asn1::der::ContentType {
        asn1::der::ContentType::Constructed
    }

    fn der_encode_content(&self, w: &mut dyn ::std::io::Write) -> ::std::io::Result<()> {
        self.a
            .der_intermediate()?
            .encode_explicit(12, asn1::der::Class::Application, w)?;
        self.b
            .der_intermediate()?
            .encode_explicit(8, asn1::der::Class::ContextSpecific, w)?;
        Ok(())
    }

    fn der_decode_content(r: &mut dyn ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
        let a: i32 =
            asn1::der::DER::der_from_intermediate(asn1::der::Intermediate::decode_explicit(r)?.2)?;
        let b: bool =
            asn1::der::DER::der_from_intermediate(asn1::der::Intermediate::decode_explicit(r)?.2)?;
        Ok(Foo { a: a, b: b })
    }
}

fn main() {
    let foo = Foo { a: 12, b: true };
    assert_eq!(foo, Foo::der_from_bytes(foo.der_bytes().unwrap()).unwrap());
}
