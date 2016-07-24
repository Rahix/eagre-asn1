use std::io::{self, Write, Read};
use byteorder::{WriteBytesExt, ReadBytesExt};

/// Encode DER length bytes
pub fn der_encode_length_bytes(length: usize, w: &mut Write) -> io::Result<()> {
    if length < 0x80 {
        try!(w.write_u8(length as u8));
    } else {
        let mut length_bytes = 0;
        let mut l2 = length;
        while l2 > 0 {
            length_bytes += 1;
            l2 >>= 8
        }
        try!(w.write_u8(0x80 | length_bytes));
        for i in (0..length_bytes).rev() {
            try!(w.write_u8((length >> i * 8) as u8 & 0xFF));
        }
    }
    Ok(())
}

/// Decode DER length bytes
///
/// Result it `(bytes_read, length)`
///
pub fn der_decode_length_bytes(r: &mut Read) -> io::Result<(usize, usize)> {
    let first_byte = try!(r.read_u8());
    let mut bytes_read = 1;
    if (first_byte & 0x80) != 0 {
        // Long form
        let length_length = first_byte & 0x7F;
        if (length_length as u64 * 8) > (usize::max_value() as f64).log2() as u64 { // Afl found
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "too big size"));
        }
        let mut length: usize = 0;
        for i in (0..length_length).rev() {
            let byte = try!(r.read_u8()) as usize;
            bytes_read += 1;
            println!("i: {}", i);
            length |= byte << i * 8;
        }
        Ok((bytes_read, length))
    } else {
        Ok((bytes_read, first_byte as usize))
    }
}
