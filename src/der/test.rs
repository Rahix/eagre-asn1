#[macro_use]
use super::*;

#[test]
fn decode_tag_bytes() {
    for i in 0..32000 {
        let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
        der_encode_tag_bytes(i, Class::Private, ContentType::Constructed, &mut stream);
        stream.set_position(0);
        let (_, tg, _, _) = der_decode_tag_bytes(&mut stream).unwrap();
        assert_eq!(tg, i);
    }
}

#[test]
fn decode_invalid_tag_0xff() {
    let mut stream = ::std::io::Cursor::new(vec!(0xff));
    if let Ok(_) = der_decode_tag_bytes(&mut stream) {
        panic!("This is illegal!");
    }
}

#[test]
fn encode_length_short() {
    for i in 0..128 {
        let mut stream = Vec::<u8>::new();
        der_encode_length_bytes(i, &mut stream).unwrap();
        assert_eq!(stream.get(0).unwrap().clone(), i as u8);
    }
}

#[test]
fn encode_length_long() {
    for i in 128..256 {
        let mut stream = Vec::<u8>::new();
        der_encode_length_bytes(i, &mut stream).unwrap();
        assert_eq!(stream.get(0).unwrap().clone(), 0b10000001);
        assert_eq!(stream.get(1).unwrap().clone(), i as u8);
    }
}

#[test]
fn encode_length_long_long_no_crash() {
    let mut stream = Vec::<u8>::new();
    der_encode_length_bytes(::std::usize::MAX, &mut stream).unwrap();
}

#[test]
fn decode_length() {
    for i in 0..32000 {
        let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
        der_encode_length_bytes(i as usize, &mut stream).unwrap();
        stream.set_position(0);
        let (_, res) = der_decode_length_bytes(&mut stream).unwrap();
        assert_eq!(i, res);
    }
}

#[test]
fn serialize_bool() {
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    true.der_encode(&mut stream).unwrap();
    stream.set_position(0);
    assert_eq!(true, bool::der_decode(&mut stream).unwrap());
    stream = ::std::io::Cursor::new(Vec::<u8>::new());
    false.der_encode(&mut stream).unwrap();
    stream.set_position(0);
    assert_eq!(false, bool::der_decode(&mut stream).unwrap());
}

#[test]
fn serialize_string() {
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    "ThisIsATestWithUtf8: ∅ ".to_string().der_encode(&mut stream).unwrap();
    stream.set_position(0);
    assert_eq!("ThisIsATestWithUtf8: ∅ ".to_string(), String::der_decode(&mut stream).unwrap());
}

#[test]
fn serialize_sequence_of() {
    let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
    let vec = vec!("I", "am", "the", "master!");
    vec.der_encode(&mut stream).unwrap();
    stream.set_position(0);
    let ret = Vec::<String>::der_decode(&mut stream).unwrap();
    for i in 0..vec.len() {
        assert_eq!(vec.get(i).unwrap().to_string(), ret.get(i).unwrap().clone());
    }
}
