eagre-asn1
==========

eagre-asn1 is an asn1 library for [Rust](https://www.rust-lang.org/).

It makes heavy use of macros to make the interface easy to use.

Currently only DER and a very small bit of xer is supported.

## Example ##
Say you have the following asn1 structure:  
```
User ::= SEQUENCE {
	username         UTF8String,
	passwordHash     [CONTEXT 12] IMPLICIT OctetString,
	age              [APPLICATION 1] EXPLICIT Integer,
	admin            Boolean
}
```
In Rust it would look like this:  
```rust
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
```rust
use eagre_asn1::der::DER;

let some_user = User { ... };
let encoded = some_user.der_bytes().unwrap();
// Send to far away planet
let decoded = User::der_from_bytes(encoded).unwrap();
assert_eq!(some_user, decoded);
```

## Implemented Types ##

[x] Any `types::Any`  
[ ] BitString `types::BitString`  
[ ] BMPString `types::BMPString`  
[x] Boolean `bool`  
[x] CharacterString `types::CharacterString`  
[x] Choice `enum`  
[ ] Date `types::Date`  
[ ] DateTime `types::DateTime`  
[ ] Duration `std::time::Duration`  
[ ] EmbeddedPDV `types::EmbeddedPDV`  
[x] Enumeration `enum`  
[ ] External  
[x] GeneralString `types::GeneralString`  
[x] GraphicString `types::GraphicString`  
[x] IA5String `types::IA5String`  
[ ] InstanceOf  
[x] Integer `i32`  
[ ] IRI  
[x] Null `types::Null`  
[x] NumericString `types::NumericString`  
[ ] ObjectClassField  
[ ] ObjectIdentifier  
[x] OctetString `Vec<u8>`  
[ ] PrintableString `types::PrintableString`  
[ ] Real `f32`  
[ ] RelativeIRI  
[ ] RelativeOID  
[x] Sequence `struct`  
[x] Sequence Of `Vec<T>`  
[ ] Set `struct`  
[ ] Set Of `types::SetOf`  
[x] T61String `types::T61String`  
[ ] Time `types::Time`  
[ ] TimeOfDay `types::TimeOfDay`  
[x] UniversalString `types::UniversalString`  
[x] UTF8String `String` or `&str`  
[x] VideotexString `types::VideotexString`  
[x] VisibleString `types::VisibleString`  

## License ##
eagre-asn1 is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
