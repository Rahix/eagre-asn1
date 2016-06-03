/// Macro to create sequence implementation for a struct
///
/// The macro is basically repeating the structure of the struct, but it includes additional
/// information about what tag should be used.
///
/// * `EXPLICIT TAG <CLASS> <TAG>;` is used for explicit tagging
/// * `IMPLICIT TAG <CLASS> <TAG>;` is used for implicit tagging
/// * `NOTAG` is used if no tagging is required
///
/// `<CLASS>` is replaced by one of
///
/// * `UNIVERSAL` for 00
/// * `APPLICATION` for 01
/// * `CONTEXT` for 10
/// * `PRIVATE` for 11
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate eagre_asn1;
/// # use eagre_asn1::der::DER;
///
/// # #[derive(Debug, PartialEq)]
/// struct SomeStruct {
///     pub foo: String,
///     pub bar: i32,
/// }
///
/// der_sequence! {
///     SomeStruct:
///         foo: EXPLICIT TAG APPLICATION 42; TYPE String,
///         bar: NOTAG TYPE i32,
/// }
///
/// # fn main() {
/// let data = SomeStruct {
///     foo: "I am a random String".to_string(),
///     bar: 42,
/// };
///
/// let encoded = data.der_bytes().unwrap();
/// // Send to far away planet
/// let decoded = SomeStruct::der_from_bytes(encoded).unwrap();
/// assert_eq!(data, decoded);
/// # }
/// ```
///
/// # Implementation Details
/// Because I am not the best with `macro_rules!` the only way I got this macro working is using
/// `ident`s which are converted to strings and matched against other strings. Improvements are
/// welcome :)
#[macro_export]
macro_rules! der_sequence {
    ($struct_name:ident : $($field_name:ident : $tagtype:ident $(TAG $tagclass:ident $tagval:expr ;)* TYPE $field_type:ty),+) => {
        impl $crate::der::DER for $struct_name {
            fn der_universal_tag() -> $crate::der::UniversalTag {
                $crate::der::UniversalTag::Sequence
            }

            fn der_content() -> $crate::der::ContentType {
                $crate::der::ContentType::Constructed
            }

            fn der_encode_content(&self, w: &mut ::std::io::Write) -> ::std::io::Result<()> {
                use $crate::der::DER;
                $({
                    let i = try!(self.$field_name.der_intermediate());
                    match stringify!($tagtype) {
                        "NOTAG" => try!(i.encode(w)),
                        $("EXPLICIT" => try!(i.encode_explicit($tagval, match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                            }, w)),
                          "IMPLICIT" => try!(i.encode_implicit($tagval, match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                            }, w)),
                            )*
                        _ => unreachable!(),
                    }
                })+
                Ok(())
            }

            fn der_decode_content(r: &mut ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
                $(
                    let i = match stringify!($tagtype) {
                        "NOTAG" => try!($crate::der::Intermediate::decode(r)),
                        "EXPLICIT" => try!($crate::der::Intermediate::decode_explicit(r)).2,
                        "IMPLICIT" => try!($crate::der::Intermediate::decode_implicit(<$field_type>::der_universal_tag() as u32, $crate::der::Class::Universal, r)).2,
                        _ => unreachable!(),
                    };
                    let $field_name : $field_type = try!($crate::der::DER::der_from_intermediate(i));
                )+
                Ok($struct_name {
                    $(
                        $field_name: $field_name,
                    )+
                })
            }
        }
    };
    ($struct_name:ident : $($field_name:ident : $tagtype:ident $(TAG $tagclass:ident $tagval:expr ;)* TYPE $field_type:ty),+,) => {
        der_sequence!($struct_name: $($field_name: $tagtype $(TAG $tagclass $tagval;)* TYPE $field_type),+);
    };
}

/// Macro to create choice implementation for enum
///
/// Used like `der_sequence!` except that every variant has to have it's unique tag
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate eagre_asn1;
/// # use eagre_asn1::der::DER;
/// # use eagre_asn1::types::Null;
///
/// # #[derive(Debug, PartialEq)]
/// enum Command {
///     Forward(i32),
///     Rotate(i32),
///     Start(Null),
///     Stop(Null),
/// }
///
/// der_choice!{
///     Command: // Don't forget to make every variant have it's own tag
///         Forward: IMPLICIT TAG CONTEXT 1; TYPE i32,
///         Rotate:  IMPLICIT TAG CONTEXT 2; TYPE i32,
///         Start:   IMPLICIT TAG CONTEXT 3; TYPE Null,
///         Stop:    IMPLICIT TAG CONTEXT 4; TYPE Null,
/// }
///
/// # fn main() {
/// for cmd in vec!(Command::Start(Null), Command::Forward(12), 
///                 Command::Rotate(2), Command::Stop(Null)) {
///     let encoded = cmd.der_bytes().unwrap();
///     // Send to far away planet
///     let decoded = Command::der_from_bytes(encoded).unwrap();
///     assert_eq!(cmd, decoded);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! der_choice {
    ($choice_name:ident : $($variant_name:ident : $tagtype:ident $(TAG $tagclass:ident $tagval:expr ;)* TYPE $variant_type:ty),+) => {
        impl $crate::der::DER for $choice_name {
            fn der_universal_tag() -> $crate::der::UniversalTag {
                $crate::der::UniversalTag::EOC
            }

            fn der_content() -> $crate::der::ContentType {
                $crate::der::ContentType::Constructed
            }

            fn der_encode_content(&self, w: &mut ::std::io::Write) -> ::std::io::Result<()> {
                use $crate::der::DER;
                match self {
                    $(&$choice_name::$variant_name(ref val) => {
                        let i = try!(val.der_intermediate());
                        match stringify!($tagtype) {
                            "NOTAG" => try!(i.encode(w)),
                            $("EXPLICIT" => try!(i.encode_explicit($tagval, match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                                }, w)),
                            "IMPLICIT" => try!(i.encode_implicit($tagval, match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                                }, w)),)*
                            _ => unreachable!(),
                    }
                    },)+
                }
                Ok(())
            }

            fn der_decode_content(r: &mut ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
                use $crate::der::DER;
                let i = try!($crate::der::Intermediate::decode(r));
                $(
                    match stringify!($tagtype) {
                        "NOTAG" => if i.tag == <$variant_type>::der_universal_tag() as u32 && i.class == $crate::der::Class::Universal {
                            return Ok($choice_name::$variant_name(try!(<$variant_type>::der_from_intermediate(i))));
                        },
                        $("EXPLICIT" => if i.tag == $tagval && i.class == match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                                                                          } {
                            return Ok($choice_name::$variant_name(try!(<$variant_type>::der_from_bytes(i.content))));
                        },
                        "IMPLICIT" => if i.tag == $tagval && i.class == match stringify!($tagclass) {
                                                                              "UNIVERSAL" => $crate::der::Class::Universal,
                                                                              "APPLICATION" => $crate::der::Class::Application,
                                                                              "CONTEXT" => $crate::der::Class::ContextSpecific,
                                                                              "PRIVATE" => $crate::der::Class::Private,
                                                                              _ => unreachable!(),
                                                                          } {
                            let mut i = i;
                            i.tag = <$variant_type>::der_universal_tag() as u32;
                            i.class = $crate::der::Class::Universal;
                            return Ok($choice_name::$variant_name(try!(<$variant_type>::der_from_intermediate(i))));
                        },)*
                        _ => unreachable!(),
                    }
                )+
                Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, "Was not able to decode choice option"))
            }
        }
    };
    ($choice_name:ident : $($variant_name:ident : $tagtype:ident $(TAG $tagclass:ident $tagval:expr ;)* TYPE $variant_type:ty),+,) => {
        der_choice!($choice_name: $($variant_name: $tagtype $(TAG $tagclass $tagval;)* TYPE $variant_type),+);
    };
}

/// Macro to create enumeration implementation for enum
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate eagre_asn1;
/// # use eagre_asn1::der::DER;
///
/// # #[derive(Debug, PartialEq)]
/// enum EventType {
///     Keyboard = 12,
///     Mouse = 10,
///     Update = 1,
///     Timer = 42,
/// }
///
/// der_enumerated!(EventType, Keyboard, Mouse, Update, Timer);
///
/// # fn main() {
/// let event = EventType::Timer;
/// let encoded = event.der_bytes().unwrap();
/// // Send to far away planet
/// let decoded = EventType::der_from_bytes(encoded).unwrap();
/// assert_eq!(event, decoded);
/// # }
/// ```
#[macro_export]
macro_rules! der_enumerated {
    ($enum_name:ident, $($enum_variant:ident),+) => {
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
                    $(&$enum_name::$enum_variant => $enum_name::$enum_variant as i32,)+
                }.der_encode(w));
                Ok(())
            }

            fn der_decode_content(r: &mut ::std::io::Read, _: usize) -> ::std::io::Result<Self> {
                use $crate::der::DER;
                use std::io;
                let val = try!(i32::der_decode(r));
                let mut result = Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown enum variant"));
                $(
                    if val == $enum_name::$enum_variant as i32 {
                        result = Ok($enum_name::$enum_variant);
                    }
                )+
                result
            }
        }
    };
    ($enum_name:ident, $($enum_val:ident),+,) => {
        der_enumerated!($enum_name, $($enum_val),+);
    };
}
