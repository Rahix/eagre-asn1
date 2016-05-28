#[macro_use]
extern crate eagre_asn1 as asn1;
extern crate byteorder;

#[derive(Debug)]
struct User {
    pub name: String,
    pub male: bool,
}

der_sequence!{
    User:
    name: NOTAG TYPE String,
    male: NOTAG TYPE bool,
}

fn main() {
    use asn1::der::DER;
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    let mut file = ::std::fs::File::create(::std::path::Path::new("test.ber")).unwrap();
    let data = User {
        name: "Rahix".to_string(),
        male: true,
    };

    data.der_encode(&mut stream).unwrap();
    data.der_encode(&mut file).unwrap();
    stream.set_position(0);
    let new_data = User::der_decode(&mut stream).unwrap();
    println!("{:?}", new_data);
}
