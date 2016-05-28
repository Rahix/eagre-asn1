use super::*;

include!("macros.rs");

#[test]
fn decode_tag_bytes() {
    for i in 0..32000 {
        let mut stream = ::std::io::Cursor::new(Vec::<u8>::new());
        der_encode_tag_bytes(i, Class::Private, ContentType::Constructed, &mut stream).unwrap();
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
    assert_eq!(true, bool::der_from_bytes(true.der_bytes().unwrap()).unwrap());
    assert_eq!(false, bool::der_from_bytes(false.der_bytes().unwrap()).unwrap());
}

#[test]
fn serialize_i32() {
    for i in vec!(::std::i32::MAX, 65535, 8, 1, 0, -1, -8, -65535, -::std::i32::MAX) {
        assert_eq!(i, i32::der_from_bytes(i.der_bytes().unwrap()).unwrap());
    }
}

#[test]
fn i32_no_panic_but_err() {
    let data = vec!(0x02, 0x07, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF); // Very big integer
    if let Ok(_) = i32::der_from_bytes(data) {
        panic!("Decoded too big Integer");
    }
}

#[test]
fn serialize_string() {
    assert_eq!("ThisIsATestWithUtf8: ∅ ".to_string(), 
               String::der_from_bytes(
                   "ThisIsATestWithUtf8: ∅ ".to_string().der_bytes().unwrap()).unwrap());
}

#[test]
fn serialize_sequence_of() {
    let vec = vec!("I", "am", "the", "master!");
    let ret = Vec::<String>::der_from_bytes(vec.der_bytes().unwrap()).unwrap();
    for i in 0..vec.len() {
        assert_eq!(vec.get(i).unwrap().to_string(), ret.get(i).unwrap().clone());
    }
}

#[test]
fn serialize_octet_string() {
    let vec = vec!(1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8);
    let ret = Vec::<u8>::der_from_bytes(vec.der_bytes().unwrap()).unwrap();
    for i in 0..vec.len() {
        assert_eq!(vec.get(i).unwrap().clone(), ret.get(i).unwrap().clone());
    }
}

#[derive(Debug, PartialEq)]
struct TestStruct {
    pub alpha: i32,
    pub beta: bool,
    pub gamma: String,
}

der_sequence!{TestStruct:
    alpha: NOTAG TYPE i32,
    beta: EXPLICIT TAG CONTEXT 42; TYPE bool,
    gamma: IMPLICIT TAG APPLICATION 397; TYPE String
}

#[test]
fn serialize_sequence() {
    //use std::io::Write;
    //use std::fs::File;
    let data = TestStruct {
        alpha: 65535,
        beta: false,
        gamma: "Hello World".to_string(),
    };
    assert_eq!(data, TestStruct::der_from_bytes(data.der_bytes().unwrap()).unwrap());
    //let mut f = File::create("test.ber").unwrap();
    //f.write_all(&data.der_bytes().unwrap()).unwrap();
}

#[derive(Debug, PartialEq)]
enum TestEnum {
    Alpha,
    Beta,
    Gamma,
}

der_enumerated!(TestEnum, Alpha, 5, Beta, 1222, Gamma, 42);

#[test]
fn serialize_enumerated() {
    for val in vec!(TestEnum::Alpha, TestEnum::Beta, TestEnum::Gamma) {
        assert_eq!(val, TestEnum::der_from_bytes(val.der_bytes().unwrap()).unwrap());
    }
}

/*#[derive(Debug, PartialEq)]
enum TestChoice {
    Alpha(i32),
    Beta(String),
    Gamma(bool),
}*/

//der_choice!(TestChoice, Alpha, i32, Beta, String, Gamma, bool);
