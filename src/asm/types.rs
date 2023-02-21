use std::{rc::Rc, str::FromStr};
use std::collections::HashMap;

use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    asm::byte_vec::{ByteVec, ByteVecImpl},
    error::KapiError,
};
use crate::error::KapiResult;

lazy_static! {
    static ref PRIMITIVE_TYPE_2_DESC: HashMap<&'static str, &'static str> = HashMap::from([
        ("boolean", "Z"),
        ("char", "C"),
        ("byte", "B"),
        ("short", "S"),
        ("int", "I"),
        ("long", "J"),
        ("float", "F"),
        ("double", "D")
    ]);
}

/// Transform canonical name into internal name
///
/// Example:<br/>
/// `int` -> `I`<br/>
/// `java.lang.String` -> `Ljava/lang/String;`<br/>
/// `int[]` -> `[I`
pub(crate) fn canonical_to_internal<S>(canonical: S) -> String
    where
        S: Into<String>,
{
    canonical_to_descriptor(canonical).replace(".", "/")
}

pub(crate) fn canonical_to_descriptor<S>(canonical: S) -> String
    where
        S: Into<String>,
{
    // array type preprocess
    let canonical_name = canonical.into();
    let dim = canonical_name.matches("[]").count();
    let mut internal_name_builder =
        String::with_capacity(canonical_name.graphemes(true).count() - dim);

    internal_name_builder.push_str(&String::from("[").repeat(dim));

    for (type_name, internal_name) in PRIMITIVE_TYPE_2_DESC.iter() {
        if canonical_name.starts_with(type_name) {
            // primitive type
            internal_name_builder.push_str(internal_name);

            return internal_name_builder;
        }
    }

    // object type
    if dim > 0 {
        internal_name_builder.push('L');
        internal_name_builder.push_str(&canonical_name.trim_end_matches("[]"));
        internal_name_builder.push(';');
    } else {
        internal_name_builder.push_str(&canonical_name.trim_end_matches("[]"));
    }

    internal_name_builder
}

pub const VOID: u8 = 0;

/// The sort of the {@code boolean} type. See {@link #getSort}.
pub const BOOLEAN: u8 = 1;

/// The sort of the {@code char} type. See {@link #getSort}.
pub const CHAR: u8 = 2;

/// The sort of the {@code byte} type. See {@link #getSort}.
pub const BYTE: u8 = 3;

/// The sort of the {@code short} type. See {@link #getSort}.
pub const SHORT: u8 = 4;

/// The sort of the {@code int} type. See {@link #getSort}.
pub const INT: u8 = 5;

/// The sort of the {@code float} type. See {@link #getSort}.
pub const FLOAT: u8 = 6;

/// The sort of the {@code long} type. See {@link #getSort}.
pub const LONG: u8 = 7;

/// The sort of the {@code double} type. See {@link #getSort}.
pub const DOUBLE: u8 = 8;

/// The sort of array reference types. See {@link #getSort}.
pub const ARRAY: u8 = 9;

/// The sort of object reference types. See {@link #getSort}.
pub const OBJECT: u8 = 10;

/// The sort of method types. See {@link #getSort}.
pub const METHOD: u8 = 11;

/// The (private) sort of object reference types represented with an internal name.
const INTERNAL: u8 = 12;

/// The descriptors of the primitive types.
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

/// A type path step that steps into the element type of an array type. See {@link #getStep}.
pub const ARRAY_ELEMENT: u8 = 0;

/// A type path step that steps into the nested type of a class type. See {@link #getStep}.
pub const INNER_TYPE: u8 = 1;

/// A type path step that steps into the bound of a wildcard type. See {@link #getStep}.
pub const WILDCARD_BOUND: u8 = 2;

/// A type path step that steps into a type argument of a generic type. See {@link #getStep}.
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

    fn from_str(s: &str) -> KapiResult<Self> {
        if s.is_empty() {
            return Err(KapiError::Utf8Error(
                "Type string must not be empty",
            ));
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
                                )));
                            }
                        }
                    }

                    output.put_u8s(&[TYPE_ARGUMENT, type_arg]);
                }
                _ => {
                    return Err(KapiError::ArgError(format!(
                        "Illegal type argument character {}",
                        c
                    )));
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

/// The sort of type references that target a type parameter of a generic class. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const CLASS_TYPE_PARAMETER: i32 = 0x00;

/// The sort of type references that target a type parameter of a generic method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_TYPE_PARAMETER: i32 = 0x01;

/// The sort of type references that target the super class of a class or one of the interfaces it
/// implements. See [`TypeRef::sort`](TypeRef#method.sort).
pub const CLASS_EXTENDS: i32 = 0x10;

/// The sort of type references that target a bound of a type parameter of a generic class. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const CLASS_TYPE_PARAMETER_BOUND: i32 = 0x11;

/// The sort of type references that target a bound of a type parameter of a generic method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_TYPE_PARAMETER_BOUND: i32 = 0x12;

/// The sort of type references that target the type of a field. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const FIELD: i32 = 0x13;

/// The sort of type references that target the return type of a method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_RETURN: i32 = 0x14;

/// The sort of type references that target the receiver type of a method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_RECEIVER: i32 = 0x15;

/// The sort of type references that target the type of a formal parameter of a method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_FORMAL_PARAMETER: i32 = 0x16;

/// The sort of type references that target the type of an exception declared in the throws clause
/// of a method. See [`TypeRef::sort`](TypeRef#method.sort).
pub const THROWS: i32 = 0x17;

/// The sort of type references that target the type of a local variable in a method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const LOCAL_VARIABLE: i32 = 0x40;

/// The sort of type references that target the type of a resource variable in a method. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const RESOURCE_VARIABLE: i32 = 0x41;

/// The sort of type references that target the type of the exception of a 'catch' clause in a
/// method. See [`TypeRef::sort`](TypeRef#method.sort).
pub const EXCEPTION_PARAMETER: i32 = 0x42;

/// The sort of type references that target the type declared in an 'instanceof' instruction. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const INSTANCEOF: i32 = 0x43;

/// The sort of type references that target the type of the object created by a 'new' instruction.
/// See [`TypeRef::sort`](TypeRef#method.sort).
pub const NEW: i32 = 0x44;

/// The sort of type references that target the receiver type of a constructor reference. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const CONSTRUCTOR_REFERENCE: i32 = 0x45;

/// The sort of type references that target the receiver type of a method reference. See
/// [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_REFERENCE: i32 = 0x46;

/// The sort of type references that target the type declared in an explicit or implicit cast
/// instruction. See [`TypeRef::sort`](TypeRef#method.sort).
pub const CAST: i32 = 0x47;

/// The sort of type references that target a type parameter of a generic constructor in a
/// constructor call. See [`TypeRef::sort`](TypeRef#method.sort).
pub const CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT: i32 = 0x48;

/// The sort of type references that target a type parameter of a generic method in a method call.
/// See [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_INVOCATION_TYPE_ARGUMENT: i32 = 0x49;

/// The sort of type references that target a type parameter of a generic constructor in a
/// constructor reference. See [`TypeRef::sort`](TypeRef#method.sort).
pub const CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT: i32 = 0x4A;

/// The sort of type references that target a type parameter of a generic method in a method
/// reference. See [`TypeRef::sort`](TypeRef#method.sort).
pub const METHOD_REFERENCE_TYPE_ARGUMENT: i32 = 0x4B;

/// A reference to a type appearing in a class, field or method declaration, or on an instruction.
/// Such a reference designates the part of the class where the referenced type is appearing (e.g. an
/// 'extends', 'implements' or 'throws' clause, a 'new' instruction, a 'catch' clause, a type cast, a
/// local variable declaration, etc).
///
/// **author** Eric Bruneton
pub struct TypeRef {
    /// The target_type and target_info structures - as defined in the Java Virtual Machine
    /// Specification (JVMS) - corresponding to this type reference. target_type uses one byte, and all
    /// the target_info union fields use up to 3 bytes (except localvar_target, handled with the
    /// specific method [`MethodVisitor::visit_local_variable_annotation`]
    /// (crate::MethodVisitor#method.visit_local_variable_annotation)). Thus, both structures can
    /// be stored in an int.
    ///
    /// <p>This int field stores target_type (called the TypeRef 'sort' in the public API of this
    /// class) in its most significant byte, followed by the target_info fields. Depending on
    /// target_type, 1, 2 or even 3 least significant bytes of this field are unused. target_info
    /// fields which reference bytecode offsets are set to 0 (these offsets are ignored in ClassReader,
    /// and recomputed in MethodWriter).
    ///
    /// ### See <a href="https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.7.20">JVMS 4.7.20</a>
    /// ### See <a href="https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.7.20.1">JVMS 4.7.20.1</a>
    target_type_and_info: i32,
}

impl TypeRef {
    pub const fn new(type_ref: i32) -> Self {
        Self {
            target_type_and_info: type_ref,
        }
    }

    pub const fn type_ref(sort: i32) -> Self {
        Self::new(sort << 24)
    }

    pub const fn type_param_ref(sort: i32, param_index: i32) -> Self {
        Self::new((sort << 24) | (param_index << 16))
    }

    pub const fn type_param_bound_ref(sort: i32, param_index: i32, bound_index: i32) -> Self {
        Self::new((sort << 24) | (param_index << 16) | (bound_index << 8))
    }

    pub const fn super_type_ref(itf_index: i32) -> Self {
        Self::new((CLASS_EXTENDS << 24) | ((itf_index & 0xFFFF) << 8))
    }

    pub const fn formal_param_ref(param_index: i32) -> Self {
        Self::new((METHOD_FORMAL_PARAMETER << 24) | (param_index << 16))
    }

    pub const fn exception_ref(exception_index: i32) -> Self {
        Self::new((THROWS << 24) | (exception_index << 8))
    }

    pub const fn try_catch_ref(try_catch_block_index: i32) -> Self {
        Self::new((EXCEPTION_PARAMETER << 24) | (try_catch_block_index << 8))
    }

    pub const fn type_arg_ref(sort: i32, arg_index: i32) -> Self {
        Self::new((sort << 24) | arg_index)
    }

    pub(crate) fn put_target<BV>(target_type_and_info: i32, output: &mut BV) -> KapiResult<()>
        where
            BV: ByteVec,
    {
        match target_type_and_info {
            CLASS_TYPE_PARAMETER | METHOD_TYPE_PARAMETER | METHOD_FORMAL_PARAMETER => {
                output.put_short((target_type_and_info >> 16) as i16)
            }
            FIELD | METHOD_RETURN | METHOD_RECEIVER => {
                output.put_short((target_type_and_info >> 16) as i16)
            }
            CAST
            | CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT
            | METHOD_INVOCATION_TYPE_ARGUMENT
            | CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT
            | METHOD_REFERENCE_TYPE_ARGUMENT => output.put_short(target_type_and_info as i16),
            CLASS_EXTENDS
            | CLASS_TYPE_PARAMETER_BOUND
            | METHOD_TYPE_PARAMETER_BOUND
            | THROWS
            | EXCEPTION_PARAMETER
            | INSTANCEOF
            | NEW
            | CONSTRUCTOR_REFERENCE
            | METHOD_REFERENCE => {
                output.put_byte((target_type_and_info >> 24) as i8);
                output.put_short(((target_type_and_info & 0xFFFF00) >> 8) as i16);
            }
            _ => {
                return Err(KapiError::ArgError(String::from(
                    "Illegal type reference target",
                )));
            }
        }

        Ok(())
    }

    pub fn sort(&self) -> i32 {
        self.target_type_and_info >> 24
    }

    pub fn type_param_index(&self) -> i32 {
        (self.target_type_and_info & 0x00FF0000) >> 16
    }

    pub fn type_param_bound_index(&self) -> i32 {
        (self.target_type_and_info & 0x0000FF00) >> 8
    }

    pub fn super_type_index(&self) -> i32 {
        ((self.target_type_and_info & 0x00FFFF00) >> 8) as i16 as i32
    }

    pub fn formal_param_index(&self) -> i32 {
        (self.target_type_and_info & 0x00FF0000) >> 16
    }

    pub fn exception_index(&self) -> i32 {
        (self.target_type_and_info & 0x00FFFF00) >> 8
    }

    pub fn try_catch_block_index(&self) -> i32 {
        (self.target_type_and_info & 0x00FFFF00) >> 8
    }

    pub fn type_arg_index(&self) -> i32 {
        self.target_type_and_info & 0xFF
    }

    pub fn value(&self) -> i32 {
        self.target_type_and_info
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::asm::types::{self, canonical_to_internal, THROWS, TypePath};

    use super::{
        CAST, CLASS_EXTENDS, CLASS_TYPE_PARAMETER, EXCEPTION_PARAMETER, FIELD, METHOD_FORMAL_PARAMETER,
        TypeRef,
    };

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

    #[test]
    fn test_new_type_ref() {
        let type_ref = TypeRef::type_ref(FIELD);

        assert_eq!(FIELD, type_ref.sort());
        assert_eq!(FIELD << 24, type_ref.value());
    }

    #[test]
    fn test_new_type_param_ref() {
        let type_param_ref = TypeRef::type_param_ref(CLASS_TYPE_PARAMETER, 3);

        assert_eq!(CLASS_TYPE_PARAMETER, type_param_ref.sort());
        assert_eq!(3, type_param_ref.type_param_index());
    }

    #[test]
    fn test_new_type_param_bound_ref() {
        let type_param_bound_ref = TypeRef::type_param_bound_ref(CLASS_TYPE_PARAMETER, 3, 7);

        assert_eq!(CLASS_TYPE_PARAMETER, type_param_bound_ref.sort());
        assert_eq!(3, type_param_bound_ref.type_param_index());
        assert_eq!(7, type_param_bound_ref.type_param_bound_index());
    }

    #[test]
    fn test_new_super_type_ref() {
        let super_type_ref = TypeRef::super_type_ref(-1);

        assert_eq!(CLASS_EXTENDS, super_type_ref.sort());
        assert_eq!(-1, super_type_ref.super_type_index());
    }

    #[test]
    fn test_new_formal_type_ref() {
        let formal_type_ref = TypeRef::formal_param_ref(3);

        assert_eq!(METHOD_FORMAL_PARAMETER, formal_type_ref.sort());
        assert_eq!(3, formal_type_ref.formal_param_index());
    }

    #[test]
    fn test_new_exception_ref() {
        let exception_ref = TypeRef::exception_ref(3);

        assert_eq!(THROWS, exception_ref.sort());
        assert_eq!(3, exception_ref.exception_index());
    }

    #[test]
    fn test_try_catch_ref() {
        let try_catch_ref = TypeRef::try_catch_ref(3);

        assert_eq!(EXCEPTION_PARAMETER, try_catch_ref.sort());
        assert_eq!(3, try_catch_ref.try_catch_block_index());
    }

    #[test]
    fn test_type_arg_ref() {
        let type_arg_ref = TypeRef::type_arg_ref(CAST, 3);

        assert_eq!(CAST, type_arg_ref.sort());
        assert_eq!(3, type_arg_ref.type_arg_index());
    }

    #[test]
    fn test_canonical_name_to_internal_name() {
        let primitive_internal_name = canonical_to_internal("int");
        let primitive_array_internal_name = canonical_to_internal("int[]");
        let internal_name = canonical_to_internal("java.lang.String");
        let internal_name_with_array = canonical_to_internal("java.lang.String[]");

        assert_eq!("I", primitive_internal_name);
        assert_eq!("[I", primitive_array_internal_name);
        assert_eq!("java/lang/String", internal_name);
        assert_eq!("[Ljava/lang/String;", internal_name_with_array);
    }
}
