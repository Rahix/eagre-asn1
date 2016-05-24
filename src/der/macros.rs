#[macro_export]
macro_rules! der_sequence {
    ($struct_name:ident : $($field_name:ident : $field_type:ty),+) => {
        impl $crate::der::DER for $struct_name {
            fn der_universal_tag() -> $crate::der::UniversalTag {
                $crate::der::UniversalTag::Sequence
            }

            fn der_content() -> $crate::der::ContentType {
                $crate::der::ContentType::Constructed
            }

            fn der_encode_content(&self, w: &mut ::std::io::Write) -> ::std::io::Result<()> {
                use $crate::der::DER;
                $(
                    try!(self.$field_name.der_encode(w));
                )+
                Ok(())
            }

            fn der_decode_content(r: &mut ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
                $(
                    let $field_name : $field_type = try!($crate::der::DER::der_decode(r));
                )+
                Ok($struct_name {
                    $(
                        $field_name: $field_name,
                    )+
                })
            }
        }
    };
    ($struct_name:ident : $($field_name:ident : $field_type:ty),+,) => {
        der_sequence!($struct_name: $($field_name: $field_type),+);
    };
}

#[macro_export]
macro_rules! der_enumerated {
    ($enum_name:ident, $($enum_variant:ident, $enum_discr:expr),+) => {
        impl $crate::der::DER for $enum_name {
            fn der_universal_tag() -> $crate::der::UniversalTag {
                $crate::der::UniversalTag::Enumerated
            }

            fn der_content() -> $crate::der::ContentType {
                $crate::der::ContentType::Primitive
            }

            fn der_encode_content(&self, w: &mut ::std::io::Write) -> ::std::io::Result<()> {
                use $crate::der::DER;
                try!(match self {
                    $(&$enum_name::$enum_variant => $enum_discr,)+
                }.der_encode(w));
                Ok(())
            }

            fn der_decode_content(r: &mut ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
                use $crate::der::DER;
                use std::io;
                let val = try!(i32::der_decode(r));
                match val {
                    $($enum_discr => Ok($enum_name::$enum_variant),)+
                    _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "unknown enum variant"))
                }
            }
        }
    };
    ($enum_name:ident, $($enum_val:ident),+,) => {
        der_enumerated!($enum_name, $($enum_val),+);
    };
}
