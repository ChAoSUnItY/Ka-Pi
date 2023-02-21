use crate::error::{KapiError, KapiResult};

pub trait ByteVec: FromIterator<u8> + From<Vec<u8>> {
    fn len(&self) -> usize;

    fn put_u8(&mut self, u8: u8);
    fn put_u8s(&mut self, u8s: &[u8]);

    fn put<N, const SIZE: usize>(&mut self, num: N)
    where
        N: Copy + ByteConv<SIZE>,
    {
        self.put_u8s(&num.ne_bytes())
    }

    fn put_utf8<S>(&mut self, string: S) -> KapiResult<()>
    where
        S: Into<String>;
}

pub(crate) type ByteVecImpl = Vec<u8>;

impl ByteVec for ByteVecImpl {
    fn len(&self) -> usize {
        self.len()
    }

    fn put_u8(&mut self, u8: u8) {
        self.push(u8)
    }

    fn put_u8s(&mut self, u8s: &[u8]) {
        self.extend_from_slice(u8s)
    }

    fn put_utf8<S>(&mut self, string: S) -> KapiResult<()>
    where
        S: Into<String>,
    {
        let s = string.into();
        let len = s.chars().map(char::len_utf8).sum::<usize>();

        if len > 65535 {
            return Err(KapiError::Utf8Error("UTF8 string too large"));
        }
        
        self.put(s.len() as u16); // put length of string (bytes len)
        self.put_u8s(s.as_bytes()); // put actual byte content

        Ok(())
    }
}

macro_rules! impl_byteconv {
    ($size:literal, $id:ident) => {
        impl ByteConv<$size> for $id {
            fn ne_bytes(&self) -> [u8; $size] {
                self.to_ne_bytes()
            }
        }
    };
}

/// A common trait to allow to-bytes-operations for numeric datas such as i32, f32, or u32.
pub trait ByteConv<const SIZE: usize> {
    fn ne_bytes(&self) -> [u8; SIZE];
}

impl_byteconv!(1, i8);
impl_byteconv!(2, i16);
impl_byteconv!(4, i32);
impl_byteconv!(8, i64);
impl_byteconv!(16, i128);
impl_byteconv!(8, isize);
impl_byteconv!(1, u8);
impl_byteconv!(2, u16);
impl_byteconv!(4, u32);
impl_byteconv!(8, u64);
impl_byteconv!(16, u128);
impl_byteconv!(8, usize);
