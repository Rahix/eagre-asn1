use std::io::Write;

/// Deprecated!
pub trait XEREncodeable {
    /// Encode content
    fn xer_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()>;
    /// Tag name
    fn xer_name(&self) -> String;
    /// Encode full tag
    fn xer_encode<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        try!(stream.write(b"<"));
        try!(stream.write(self.xer_name().as_bytes()));
        try!(stream.write(b">"));
        try!(self.xer_encode_content(stream));
        try!(stream.write(b"</"));
        try!(stream.write(self.xer_name().as_bytes()));
        try!(stream.write(b">"));
        Ok(())
    }
}

impl XEREncodeable for bool {
    fn xer_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        try!(stream.write(match self {
            &true => "True".as_bytes(),
            &false => "False".as_bytes(),
        }));
        Ok(())
    }

    fn xer_name(&self) -> String {
        "Boolean".to_string()
    }
}

impl XEREncodeable for String {
    fn xer_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        try!(stream.write(self.as_bytes()));
        Ok(())
    }

    fn xer_name(&self) -> String {
        "String".to_string()
    }
}

impl XEREncodeable for i32 {
    fn xer_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        try!(stream.write(format!("{}", self).as_bytes()));
        Ok(())
    }

    fn xer_name(&self) -> String {
        "Integer".to_string()
    }
}

impl<T: XEREncodeable> XEREncodeable for Vec<T> {
    fn xer_encode_content<W: Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
        for item in self.iter() {
            try!(item.xer_encode(stream));
        }
        Ok(())
    }

    fn xer_name(&self) -> String {
        "Array".to_string()
    }
}

/// Deprecated!
#[macro_export]
macro_rules! implement_xer {
    ($struct_name:ident, $($field_name:ident),+) => {
        impl $crate::xer::XEREncodeable for $struct_name {
            fn xer_encode_content<W: ::std::io::Write>(&self, stream: &mut W) -> ::std::io::Result<()> {
                $(
                    try!(stream.write(b"<"));
                    try!(stream.write(stringify!($field_name).to_string().as_bytes()));
                    try!(stream.write(b">"));
                    try!(self.$field_name.xer_encode_content(stream));
                    try!(stream.write(b"</"));
                    try!(stream.write(stringify!($field_name).to_string().as_bytes()));
                    try!(stream.write(b">"));
                )+
                Ok(())
            }

            fn xer_name(&self) -> String {
                stringify!($struct_name).to_string()
            }
        }
    }
}
