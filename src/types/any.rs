use std::io::{self, Write, Read};
use der::*;


struct Peek<T> {
    inner: T,
    buffer: Vec<u8>,
    peeking: bool,
}

impl<T: Read> Peek<T> {
    pub fn new(reader: T) -> Peek<T> {
        Peek {
            inner: reader,
            buffer: Vec::new(),
            peeking: true,
        }
    }

    pub fn stop(&mut self) {
        self.peeking = false;
    }
}

impl<T: Read> Read for Peek<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.peeking {
            let n = try!(self.inner.read(buf));
            self.buffer.extend_from_slice(&buf);
            Ok(n)
        } else {
            let mut i = 0;
            while i < buf.len() {
                if self.buffer.len() != 0 {
                    buf[i] = self.buffer.remove(0);
                } else {
                    let mut byte = [0; 1];
                    try!(self.inner.read_exact(&mut byte));
                    buf[i] = byte[0];
                }
                i += 1;
            }
            Ok(i)
        }
    }
}

#[test]
fn peek_reader() {
    let stream = io::Cursor::new(vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    let mut peek = Peek::new(stream);
    let mut buffer: [u8; 3] = [0; 3];
    peek.read(&mut buffer).unwrap();
    assert_eq!(buffer, [0, 1, 2]);
    peek.stop();
    let mut buffer: [u8; 9] = [0; 9];
    peek.read(&mut buffer).unwrap();
    assert_eq!(buffer, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

}

pub struct Any(Vec<u8>);

impl DER for Any {
    fn der_universal_tag() -> UniversalTag {
        unimplemented!()
    }

    fn der_content() -> ContentType {
        unimplemented!()
    }

    fn der_encode(&self, w: &mut Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_encode_content(&self, _: &mut Write) -> io::Result<()> {
        unimplemented!()
    }

    fn der_decode(r: &mut Read) -> io::Result<Self> {
        unimplemented!()
    }

    fn der_decode_content(_: &mut Read, length: usize) -> io::Result<Self> {
        unimplemented!()
    }
}

/*#[test]
fn serialize_any() {
    let buf = 42.der_bytes().unwrap();
    let any = Any(buf);
    let mut stream = io::Cursor::new(any.der_bytes().unwrap());
    let Any(vec) = Any::der_decode(&mut stream).unwrap();
    let value = i32::der_decode(&mut io::Cursor::new(vec)).unwrap();
    assert_eq!(value, 42);
}*/
