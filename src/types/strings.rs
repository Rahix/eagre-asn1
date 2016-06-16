use der::{self, DER};
use std::io::{self, Read, Write};

// Macro for lazy people like me
macro_rules! string_type {
    ($name:ident) => {
        /// Asn1 String Type
        ///
        /// Currently restricted character sets are not enforced, so it is
        /// the callers job to check wether string contents are legal for
        /// the specific string type
        pub struct $name(String);

        impl From<String> for $name {
            fn from(s: String) -> $name {
                $name(s)
            }
        }

        impl From<$name> for String {
            fn from(s: $name) -> String {
                s.0
            }
        }

        impl DER for $name {
            fn der_universal_tag() -> der::UniversalTag {
                der::UniversalTag::$name
            }

            fn der_content() -> der::ContentType {
                der::ContentType::Primitive
            }

            fn der_encode_content(&self, w: &mut Write) -> io::Result<()> {
                try!(w.write(self.0.as_bytes()));
                Ok(())
            }

            fn der_decode_content(r: &mut Read, length: usize) -> io::Result<$name> {
                let mut encoded = r.take(length as u64);
                let mut buffer = String::new();
                try!(encoded.read_to_string(&mut buffer));
                Ok($name(buffer))
            }
        }
    }
}

string_type!(NumericString);
string_type!(PrintableString);
string_type!(T61String);
string_type!(VideotexString);
string_type!(IA5String);
string_type!(GraphicString);
string_type!(VisibleString);
string_type!(GeneralString);
string_type!(UniversalString);
string_type!(CharacterString);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        use der::DER;
        let bytes = IA5String::from("FooBar123".to_string()).der_bytes().unwrap();
        assert_eq!("FooBar123",
                   &String::from(IA5String::der_from_bytes(bytes).unwrap()));
    }
}
