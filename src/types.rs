use std::{rc::Rc, str::FromStr};

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    byte_vec::{ByteVec, ByteVecImpl},
    error::KapiError,
};

pub const VOID: u8 = 0;

/** The sort of the {@code boolean} type. See {@link #getSort}. */
pub const BOOLEAN: u8 = 1;

/** The sort of the {@code char} type. See {@link #getSort}. */
pub const CHAR: u8 = 2;

/** The sort of the {@code byte} type. See {@link #getSort}. */
pub const BYTE: u8 = 3;

/** The sort of the {@code short} type. See {@link #getSort}. */
pub const SHORT: u8 = 4;

/** The sort of the {@code int} type. See {@link #getSort}. */
pub const INT: u8 = 5;

/** The sort of the {@code float} type. See {@link #getSort}. */
pub const FLOAT: u8 = 6;

/** The sort of the {@code long} type. See {@link #getSort}. */
pub const LONG: u8 = 7;

/** The sort of the {@code double} type. See {@link #getSort}. */
pub const DOUBLE: u8 = 8;

/** The sort of array reference types. See {@link #getSort}. */
pub const ARRAY: u8 = 9;

/** The sort of object reference types. See {@link #getSort}. */
pub const OBJECT: u8 = 10;

/** The sort of method types. See {@link #getSort}. */
pub const METHOD: u8 = 11;

/** The (private) sort of object reference types represented with an internal name. */
const INTERNAL: u8 = 12;

/** The descriptors of the primitive types. */
const PRIMITIVE_DESCRIPTORS: &'static str = "VZCBSIFJD";

pub struct Type {
    sort: u8,
    value_buffer: String,
    value_begin: usize,
    value_end: usize,
}

impl Type {
    pub const fn new(sort: u8, value_buffer: String, value_begin: usize, value_end: usize) -> Self {
        Self {
            sort,
            value_buffer,
            value_begin,
            value_end,
        }
    }
}

/** A type path step that steps into the element type of an array type. See {@link #getStep}. */
pub const ARRAY_ELEMENT: u8 = 0;

/** A type path step that steps into the nested type of a class type. See {@link #getStep}. */
pub const INNER_TYPE: u8 = 1;

/** A type path step that steps into the bound of a wildcard type. See {@link #getStep}. */
pub const WILDCARD_BOUND: u8 = 2;

/** A type path step that steps into a type argument of a generic type. See {@link #getStep}. */
pub const TYPE_ARGUMENT: u8 = 3;

pub struct TypePath {
    type_path_container: Rc<Vec<u8>>,
    type_path_offset: usize,
}

impl TypePath {
    pub(crate) const fn new(type_path_container: Rc<Vec<u8>>, type_path_offset: usize) -> Self {
        Self {
            type_path_container,
            type_path_offset,
        }
    }

    pub fn put<BV>(type_path: Option<&TypePath>, output: &mut BV)
    where
        BV: ByteVec,
    {
        if let Some(type_path) = type_path {
            let len = (type_path.type_path_container[type_path.type_path_offset] * 2 + 1) as usize;
            output.put_u8s(
                &type_path.type_path_container
                    [type_path.type_path_offset..type_path.type_path_offset + len],
            );
        } else {
            output.put_u8(0);
        }
    }

    pub fn len(&self) -> usize {
        self.type_path_container[self.type_path_offset] as usize
    }

    pub fn get_step(&self, index: usize) -> u8 {
        self.type_path_container[self.type_path_offset + index * 2 + 1]
    }

    pub fn get_step_argument(&self, index: usize) -> u8 {
        self.type_path_container[self.type_path_offset + index * 2 + 2]
    }
}

impl FromStr for TypePath {
    type Err = KapiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(KapiError::Utf8Error(String::from(
                "Type string must not be empty",
            )));
        }

        let type_path_len = s.graphemes(true).count();
        let mut chars = s.chars();
        let mut output: ByteVecImpl = Vec::with_capacity(type_path_len).into();

        output.put_u8(0);

        while let Some(c) = chars.next() {
            match c {
                '[' => output.put_u8s(&[ARRAY_ELEMENT, 0]),
                '.' => output.put_u8s(&[INNER_TYPE, 0]),
                '*' => output.put_u8s(&[WILDCARD_BOUND, 0]),
                _ if c.is_ascii_digit() => {
                    let mut type_arg = c as u8 - 48;

                    while let Some(c) = chars.next() {
                        match c {
                            _ if c.is_ascii_digit() => type_arg = type_arg * 10 + c as u8 - 48,
                            ';' => break,
                            _ => {
                                return Err(KapiError::ArgError(format!(
                                    "Illegal type argument character {}",
                                    c
                                )))
                            }
                        }
                    }

                    output.put_u8s(&[TYPE_ARGUMENT, type_arg]);
                }
                _ => {
                    return Err(KapiError::ArgError(format!(
                        "Illegal type argument character {}",
                        c
                    )))
                }
            }
        }

        output[0] = (output.len() / 2) as u8;
        Ok(TypePath::new(Rc::new(output.into()), 0))
    }
}

impl ToString for TypePath {
    fn to_string(&self) -> String {
        let len = self.len();
        let mut result = String::with_capacity(len);

        for i in 0..len {
            match self.get_step(i) {
                ARRAY_ELEMENT => result.push('['),
                INNER_TYPE => result.push('.'),
                WILDCARD_BOUND => result.push('*'),
                TYPE_ARGUMENT => {
                    result.push_str(&self.get_step_argument(i).to_string());
                    result.push(';');
                }
                _ => assert!(false),
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::types::{self, TypePath};

    #[test]
    fn test_type_path_len() {
        assert_eq!(5, TypePath::from_str("[.[*0").unwrap().len());
        assert_eq!(5, TypePath::from_str("[*0;*[").unwrap().len());
        assert_eq!(1, TypePath::from_str("10;").unwrap().len());
        assert_eq!(2, TypePath::from_str("1;0;").unwrap().len());
    }

    #[test]
    fn test_type_path_get_step() {
        let type_path = TypePath::from_str("[.[*7").unwrap();

        assert_eq!(types::ARRAY_ELEMENT, type_path.get_step(0));
        assert_eq!(types::INNER_TYPE, type_path.get_step(1));
        assert_eq!(types::WILDCARD_BOUND, type_path.get_step(3));
        assert_eq!(types::TYPE_ARGUMENT, type_path.get_step(4));
        assert_eq!(7, type_path.get_step_argument(4));
    }

    #[test]
    fn test_type_path_from_str_and_to_str() {
        assert!(TypePath::from_str("").is_err());
        assert_eq!("[.[*0;", TypePath::from_str("[.[*0").unwrap().to_string());
        assert_eq!("[*0;*[", TypePath::from_str("[*0;*[").unwrap().to_string());
        assert_eq!("10;", TypePath::from_str("10;").unwrap().to_string());
        assert_eq!("1;0;", TypePath::from_str("1;0;").unwrap().to_string());
        assert!(TypePath::from_str("-").is_err());
        assert!(TypePath::from_str("=").is_err());
        assert!(TypePath::from_str("1-").is_err());
    }
}
