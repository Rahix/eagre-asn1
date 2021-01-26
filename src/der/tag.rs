use super::*;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

/// Encode DER tag bytes
pub fn der_encode_tag_bytes(
    tag: u32,
    class: Class,
    content: ContentType,
    w: &mut dyn Write,
) -> io::Result<()> {
    let first_byte =
        (class as u8) << 6 | (content as u8) << 5 | if tag < 0x1F { tag as u8 } else { 0x1F };
    w.write_u8(first_byte)?;
    if tag > 0x1E {
        let mut bytes = 0;
        let mut tag2 = tag;
        while tag2 > 0 {
            bytes += 1;
            tag2 >>= 7
        }
        for i in (0..bytes).rev() {
            w.write_u8(((tag >> i * 7) & 0x7F) as u8 | if i != 0 { 0x80 } else { 0x00 })?;
        }
    }
    Ok(())
}

/// Decode DER tag bytes
///
/// Result is `(bytes_read, tag, class, content_type)`
//                                          Bytes Read, Tag, Class, ContentType
pub fn der_decode_tag_bytes(r: &mut dyn Read) -> io::Result<(usize, u32, Class, ContentType)> {
    let first_byte = r.read_u8()?;
    let mut bytes_read = 1;
    let content = match first_byte >> 5 & 1 {
        0 => ContentType::Primitive,
        1 => ContentType::Constructed,
        _ => unreachable!(),
    };
    let class = match first_byte >> 6 & 3 {
        0 => Class::Universal,
        1 => Class::Application,
        2 => Class::ContextSpecific,
        3 => Class::Private,
        _ => unreachable!(),
    };
    let mut tag: u32 = 0;
    if (first_byte as u32 & 31) != 31 {
        tag = first_byte as u32 & 31;
    } else {
        let mut bytes = Vec::new();
        loop {
            let byte = r.read_u8()?;
            bytes_read += 1;
            bytes.push(byte);
            if byte & 0x80 == 0 {
                break;
            }
        }
        if (7 * bytes.len()) > (u32::max_value() as f64).log2() as usize {
            // Afl found
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "too many tag bytes",
            ));
        }
        for i in 0..bytes.len() {
            let byte = bytes.get(i).unwrap().clone() as u32;
            tag |= (byte & 0x7f) << 7 * (bytes.len() - i - 1);
        }
    }
    Ok((bytes_read, tag, class, content))
}
