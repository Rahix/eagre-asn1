#[macro_use]
extern crate eagre_asn1 as asn1;

use asn1::der::DEREncodeable;

fn main() {
    //let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    let mut stream = ::std::fs::File::create(::std::path::Path::new("test.ber")).unwrap();
    0xDEADBEEF.der_encode(&mut stream).unwrap();
    //let bytes = stream.into_inner();
    /*for byte in bytes {
        println!("{:x}", byte);
    }*/
}
