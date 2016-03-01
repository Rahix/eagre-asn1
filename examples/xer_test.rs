#[macro_use]
extern crate eagre_asn1 as asn1;

use asn1::xer::XEREncodeable;

struct User {
    pub name: String,
    pub id: i32,
}

implement_xer!(User, name, id);

fn main() {
    let foo = User {
        name: "Rahix".to_string(),
        id: 12,
    };
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    foo.xer_encode(&mut stream).unwrap();
    println!("{}", String::from_utf8(stream.into_inner()).unwrap());
}
