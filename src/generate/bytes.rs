use crate::error::{KapiError, KapiResult};
use crate::generate::symbol::SymbolTable;

pub trait ByteVec: FromIterator<u8> + From<Vec<u8>> {
    fn len(&self) -> usize;

    fn put_u8(&mut self, u8: u8);
    fn put_u8s(&mut self, u8s: &[u8]);

    fn put_ne<N, const SIZE: usize>(&mut self, num: N)
    where
        N: Copy + ByteConv<SIZE>,
    {
        self.put_u8s(&num.ne_bytes())
    }

    fn put_le<N, const SIZE: usize>(&mut self, num: N)
    where
        N: Copy + ByteConv<SIZE>,
    {
        self.put_u8s(&num.le_bytes())
    }

    fn put_be<N, const SIZE: usize>(&mut self, num: N)
    where
        N: Copy + ByteConv<SIZE>,
    {
        self.put_u8s(&num.be_bytes())
    }

    fn put_utf8(&mut self, string: &str) -> KapiResult<()>;
}

pub(crate) type ByteVecImpl = Vec<u8>;

impl ByteVec for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }

    fn put_u8(&mut self, u8: u8) {
        self.push(u8)
    }

    fn put_u8s(&mut self, u8s: &[u8]) {
        self.extend_from_slice(u8s)
    }

    fn put_utf8(&mut self, string: &str) -> KapiResult<()> {
        let len = string.chars().map(char::len_utf8).sum::<usize>();
        let len_pos = self.len();
        let mut actual_byte_len = 0usize;

        if len > 65535 {
            return Err(KapiError::Utf8Error("UTF8 string too large"));
        }

        self.put_u8s(&[0u8; 2]); // Preserve len with placeholder

        for char in string.chars() {
            let char_val = char as u32;

            match char_val {
                0x0001..=0x007F => {
                    self.put_u8(char as u8);
                    actual_byte_len += 1;
                }
                0x0000 | 0x0080..=0x07FF => {
                    self.put_u8s(&[
                        (0xC0 | char_val >> 6 & 0x1F) as u8,
                        (0x80 | char_val & 0x3F) as u8,
                    ]);
                    actual_byte_len += 2;
                }
                _ => {
                    self.put_u8s(&[
                        (0xE0 | char_val >> 12 & 0xF) as u8,
                        (0x80 | char_val >> 6 & 0x3F) as u8,
                        (0x80 | char_val & 0x3F) as u8,
                    ]);
                    actual_byte_len += 3;
                }
            }
        }

        if actual_byte_len > 65535 {
            return Err(KapiError::Utf8Error("UTF8 string too large"));
        }

        self[len_pos..=len_pos + 1].swap_with_slice(&mut (actual_byte_len as u16).to_be_bytes()); // Replace placeholder with actual len's bits

        Ok(())
    }
}

macro_rules! impl_byteconv {
    ($size:literal, $id:ident) => {
        impl ByteConv<$size> for $id {
            fn ne_bytes(&self) -> [u8; $size] {
                self.to_ne_bytes()
            }

            fn le_bytes(&self) -> [u8; $size] {
                self.to_le_bytes()
            }

            fn be_bytes(&self) -> [u8; $size] {
                self.to_be_bytes()
            }
        }
    };
}

/// A common trait to allow to-bytes-operations for numeric datas such as i32, f32, or u32.
pub trait ByteConv<const SIZE: usize> {
    fn ne_bytes(&self) -> [u8; SIZE];
    fn le_bytes(&self) -> [u8; SIZE];
    fn be_bytes(&self) -> [u8; SIZE];
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

pub(crate) trait ByteVecGen {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) -> KapiResult<()>;
}

#[cfg(test)]
mod test {
    use crate::error::KapiResult;
    #[allow(arithmetic_overflow)]
    use crate::generate::bytes::{ByteVec, ByteVecImpl};

    #[test]
    fn test_byte_vec_impl_ascii_string() -> KapiResult<()> {
        let mut byte_vec = ByteVecImpl::new();

        byte_vec.put_utf8("ABC")?;

        assert_eq!(&byte_vec[..], [0, 3, b'A', b'B', b'C']);

        Ok(())
    }

    #[test]
    fn test_byte_vec_impl_ascii_string_too_large() {
        let mut byte_vec = ByteVecImpl::new();

        for size in [65535, 65536] {
            let char_buf = "A".repeat(size);
            let result = byte_vec.put_utf8(&char_buf);

            if size > 65535 {
                assert!(result.is_err())
            } else {
                assert!(result.is_ok())
            }
        }
    }

    #[test]
    fn test_byte_vec_impl_unicode_string() -> KapiResult<()> {
        let mut byte_vec = ByteVecImpl::new();

        byte_vec.put_utf8(&String::from_utf16_lossy(&[
            b'a' as u16,
            0x0000,
            0x0080,
            0x0800,
        ]))?;

        assert_eq!(
            &byte_vec[..],
            [
                0,
                8,
                b'a',
                64u8.wrapping_neg(),
                128u8.wrapping_neg(),
                62u8.wrapping_neg(),
                128u8.wrapping_neg(),
                32u8.wrapping_neg(),
                96u8.wrapping_neg(),
                128u8.wrapping_neg()
            ]
        );

        Ok(())
    }

    #[test]
    fn test_byte_vec_impl_unicode_string_too_large() {
        let mut byte_vec = ByteVecImpl::new();

        let char_buf = String::from('\u{7FF}').repeat(32768);
        let result = byte_vec.put_utf8(&char_buf);

        assert!(result.is_err());
    }
}
