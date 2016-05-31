use std::io::{self, Write, Read};

use super::*;

/// Intermediate Type
///
/// Intermediate type necessary for tagging, etc.
#[derive(Clone, Debug)]
pub struct Intermediate {
    /// Class of this encoded object
    pub class: Class,
    /// Content Type of this encoded object
    pub content_type: ContentType,
    /// Tag of this encoded object
    pub tag: u32,
    /// Content octets
    pub content: Vec<u8>,
}

impl Intermediate {
    /// Create new Intermediate with empty content octets
    pub fn new(class: Class, ct: ContentType, tag: u32) -> Intermediate {
        Intermediate {
            class: class,
            content_type: ct,
            tag: tag,
            content: vec!(),
        }
    }

    /// Add a content like using a builder
    ///
    /// ```
    /// # use eagre_asn1::der::*;
    /// let i = Intermediate::new(Class::Application,
    ///                           ContentType::Primitive,
    ///                           2).with_content(vec!(0x00));
    /// ```
    pub fn with_content(mut self, content: Vec<u8>) -> Intermediate {
        self.content = content;
        self
    }

    /// Set content octets
    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    /// Encode this Intermediate
    pub fn encode(&self, w: &mut Write) -> io::Result<()> {
        try!(der_encode_tag_bytes(self.tag, self.class, self.content_type, w));
        try!(der_encode_length_bytes(self.content.len(), w));
        try!(w.write(&self.content));
        Ok(())
    }

    /// Encode this Intermediate using explicit tagging
    pub fn encode_explicit(&self, tag: u32, class: Class, w: &mut Write) -> io::Result<()> {
        try!(der_encode_tag_bytes(tag, class, ContentType::Constructed, w));
        let mut stream = io::Cursor::new(vec!());
        try!(self.encode(&mut stream));
        let data = stream.into_inner();
        try!(der_encode_length_bytes(data.len(), w));
        try!(w.write(&data));
        Ok(())
    }

    /// Encode this Intermediate using implicit tagging
    pub fn encode_implicit(&self, tag: u32, class: Class, w: &mut Write) -> io::Result<()> {
        try!(der_encode_tag_bytes(tag, class, self.content_type, w));
        try!(der_encode_length_bytes(self.content.len(), w));
        try!(w.write(&self.content));
        Ok(())
    }

    /// Decode an Intermediate
    pub fn decode(r: &mut Read) -> io::Result<Intermediate> {
        let (_, tag, class, content_type) = try!(der_decode_tag_bytes(r));
        let (_, length) = try!(der_decode_length_bytes(r));
        let mut enc = r.take(length as u64);
        let mut buf = vec!();
        try!(enc.read_to_end(&mut buf));
        Ok(Intermediate {
            class: class,
            content_type: content_type,
            tag: tag,
            content: buf,
        })
    }

    /// Decode an Intermediate using explicit tagging
    pub fn decode_explicit(r: &mut Read) -> io::Result<(u32, Class, Intermediate)> {
        let (_, tag, class, _) = try!(der_decode_tag_bytes(r));
        let (_, _) = try!(der_decode_length_bytes(r));
        Ok((tag, class, try!(Intermediate::decode(r))))
    }

    /// Decode an Intermediate using implicit tagging
    pub fn decode_implicit(tag: u32, class: Class, r: &mut Read) -> io::Result<(u32, Class, Intermediate)> {
        let (_, tag_impl, class_impl, content_type) = try!(der_decode_tag_bytes(r));
        let (_, length) = try!(der_decode_length_bytes(r));
        let mut enc = r.take(length as u64);
        let mut buf = vec!();
        try!(enc.read_to_end(&mut buf));
        Ok((tag_impl, class_impl, Intermediate {
            class: class,
            content_type: content_type,
            tag: tag,
            content: buf,
        }))
    }
}

#[test]
fn test_explicit_tagging() {
    let i = 1234.der_intermediate().unwrap();
    let mut data = io::Cursor::new(vec!());
    i.encode_explicit(42, Class::Private, &mut data).unwrap();
    data.set_position(0);
    let (tag, class, intermediate) = Intermediate::decode_explicit(&mut data).unwrap();
    assert_eq!(tag, 42);
    assert_eq!(class, Class::Private);
    assert_eq!(1234, i32::der_from_intermediate(intermediate).unwrap());
}
