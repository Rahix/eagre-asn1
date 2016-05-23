extern crate byteorder;

#[macro_use]
pub mod xer;

#[macro_use]
pub mod der;

pub mod types;

pub mod peek;

pub use peek::Peek;

#[macro_export]
macro_rules! debug_xer {
    ($struct_name:ident) => {
        impl ::std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use $crate::xer::XEREncodeable;
                let mut stream = Vec::<u8>::new();
                self.xer_encode(&mut stream).unwrap();
                write!(f, "{}", ::std::string::String::from_utf8(stream).unwrap())
            }
        }
    }
}
