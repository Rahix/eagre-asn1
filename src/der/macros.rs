#[macro_export]
macro_rules! der_sequence {
    ($struct_name:ident, $($field_name:ident , $field_type:ty),+) => {
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

            fn der_decode_content(r: &mut ::std::io::Read, length: usize) -> ::std::io::Result<Self> {
                use $crate::der::DER;
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
    ($struct_name:ident, $($field_name:ident , $field_type:ty),+,) => {
        der_sequence!($struct_name, $($field_name, $field_type),+);
    };
}
