use der::*;

pub struct Sequence {
    inner: Vec<(String, Box<DER>)>,
}
