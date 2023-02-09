use crate::error::KapiError;

pub trait ByteVec: FromIterator<u8> + From<Vec<u8>> {
    fn len(&self) -> usize;

    fn put_u8(&mut self, u8: u8);
    fn put_u8s(&mut self, u8s: &[u8]);

    fn put_byte(&mut self, byte: i8);
    fn put_bytes(&mut self, bytes: &[i8]);

    fn put_short(&mut self, short: i16);
    fn put_shorts(&mut self, shorts: &[i16]);

    fn put_int(&mut self, int: i32);
    fn put_ints(&mut self, ints: &[i32]);

    fn put_utf8<S>(&mut self, string: S) -> Result<(), KapiError>
    where
        S: Into<String>;
}

pub(crate) type ByteVecImpl = Vec<u8>;

impl ByteVec for ByteVecImpl {
    fn len(&self) -> usize {
        self.len()
    }

    fn put_u8(&mut self, u8: u8) {
        self.push(u8);
    }

    fn put_u8s(&mut self, u8s: &[u8]) {
        self.extend_from_slice(u8s);
    }

    fn put_byte(&mut self, byte: i8) {
        self.put_u8s(&byte.to_ne_bytes());
    }

    fn put_bytes(&mut self, bytes: &[i8]) {
        self.append(&mut bytes.iter().flat_map(|&b| b.to_ne_bytes()).collect());
    }

    fn put_short(&mut self, short: i16) {
        self.put_u8s(&short.to_ne_bytes());
    }

    fn put_shorts(&mut self, shorts: &[i16]) {
        self.append(&mut shorts.iter().flat_map(|&s| s.to_ne_bytes()).collect());
    }

    fn put_int(&mut self, int: i32) {
        self.put_u8s(&int.to_ne_bytes());
    }

    fn put_ints(&mut self, ints: &[i32]) {
        self.append(&mut ints.iter().flat_map(|&i| i.to_ne_bytes()).collect());
    }

    fn put_utf8<S>(&mut self, string: S) -> Result<(), KapiError>
    where
        S: Into<String>,
    {
        let s = string.into();
        let len = s.chars().map(|c| c.len_utf8()).sum::<usize>();

        if len > 65535 {
            return Err(KapiError::Utf8Error(String::from("UTF8 string too large")));
        }

        self.put_u8s(&(s.len() as u16).to_ne_bytes()); // put length of string (bytes len)
        self.put_u8s(s.as_bytes()); // put actual byte content

        Ok(())
    }
}
