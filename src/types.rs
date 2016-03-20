#![allow(dead_code)]

macro_rules! string_convert {
    ($struct_name:ident) => {
        impl ::std::convert::Into<String> for $struct_name {
            fn into(self) -> String {
                self.s
            }
        }

        impl ::std::convert::From<String> for $struct_name {
            fn from(s: String) -> $struct_name {
                $struct_name {
                    s: s,
                }
            }
        }
    }
}

pub struct NumericString { s: String }
string_convert!(NumericString);
pub struct PrintableString { s: String }
string_convert!(PrintableString);
pub struct T61String { s: String }
string_convert!(T61String);
pub struct VideotexString { s: String }
string_convert!(VideotexString);
pub struct IA5String { s: String }
string_convert!(IA5String);
pub struct GraphicString { s: String }
string_convert!(GraphicString);
pub struct VisibleString { s: String }
string_convert!(VisibleString);
pub struct GeneralString { s: String }
string_convert!(GeneralString);
pub struct UniversalString { s: String }
string_convert!(UniversalString);
pub struct CharacterString { s: String }
string_convert!(CharacterString);

pub struct UTCTime;
pub struct GeneralizedTime;
