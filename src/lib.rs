/*!
eagre-asn1
==========

eagre-asn1 is an asn1 library for [Rust](https://www.rust-lang.org/).

It makes heavy use of macros to make the interface easy to use.

Currently only DER and a very small bit of XER is supported.

## Example ##
Say you have the following asn1 structure:

```text
User ::= SEQUENCE {
	username         UTF8String,
	passwordHash     [CONTEXT 12] IMPLICIT OctetString,
	age              [APPLICATION 1] EXPLICIT Integer,
	admin            Boolean
}
```

In Rust it would look like this:

```ignore
struct User {
	pub username: String,
	pub password_hash: Vec<u8>,
	pub age: i32,
	pub admin: bool,
}

der_sequence!{
	User:
		username:      NOTAG                       TYPE String,
		password_hash: IMPLICIT TAG CONTEXT 12;    TYPE Vec<u8>,
		age:           EXPLICIT TAG APPLICATION 1; TYPE i32,
		admin:         NOTAG                       TYPE bool,
}
```

And serializing is as easy as:

```ignore
use eagre_asn1::der::DER;

let some_user = User { ... };
let encoded = some_user.der_bytes().unwrap();
// Send to far away planet
let decoded = User::der_from_bytes(encoded).unwrap();
assert_eq!(some_user, decoded);
```
 */

#![warn(missing_docs)]
#[doc(hidden)]
extern crate byteorder;

/// **UNFINISHED** XER Implementation
#[macro_use]
pub mod xer;

/// DER Implementation
#[macro_use]
pub mod der;

/// Asn1 Types
pub mod types;

#[doc(hidden)]
pub mod peek;

pub use peek::Peek;

#[doc(hidden)]
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
