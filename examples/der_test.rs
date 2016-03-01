#[macro_use]
extern crate eagre_asn1 as asn1;

fn main() {
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    asn1::der::der_tag_bytes(0xDEAD, asn1::der::Class::ContextSpecific, asn1::der::PC::Primitive, &mut stream).unwrap();
    //asn1::der::der_length_bytes(435, &mut stream).unwrap();
    let bytes = stream.into_inner();
    for byte in bytes {
        println!("{:b}", byte);
    }
}
