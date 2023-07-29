use std::cell::RefCell;
use std::ops::Deref;

use paste::paste;
use serde::{Deserialize, Serialize};

use crate::node::attribute::{Attribute, AttributeInfo, BootstrapMethod, BootstrapMethods};
use crate::node::error::{NodeResError, NodeResResult};
use crate::parse::ParseError;

macro_rules! const_getter {
    ($(#[$attr:meta])* => $variant_name: ident, $variant: ty) => {
        paste! {
            $(#[$attr])*
            pub fn [< get_ $variant_name >] (&self, index: u16) -> Option<&$variant> {
                if let Some(Some(Constant::$variant(constant))) = self.get(index as usize - 1)
                {
                    Some(constant)
                } else {
                    None
                }
            }
        }
    };
}

/// Represents a class constant pool.
///
/// See [4.4 The Constant Pool](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=93).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConstantPool {
    len: u16,
    entries: Vec<Option<Constant>>,
}

impl ConstantPool {
    pub(crate) fn with_capacity(capacity: u16) -> Self {
        Self {
            len: 1,
            entries: Vec::with_capacity(capacity as usize),
        }
    }

    pub fn len(&self) -> u16 {
        self.len
    }

    pub(crate) fn add(&mut self, constant: Constant) {
        let occupies_2_slots = constant.occupies_2_slots();

        self.entries.push(Some(constant));

        if occupies_2_slots {
            self.len += 2;

            self.entries.push(None);
        } else {
            self.len += 1;
        }
    }

    pub fn get_constant(&self, index: u16) -> Option<&Constant> {
        self.get(index as usize - 1).map(Option::as_ref).flatten()
    }

    const_getter!(
        /// Gets [Utf8] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => utf8, Utf8
    );
    const_getter!(
        /// Gets [Integer] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => integer, Integer
    );
    const_getter!(
        /// Gets [Float] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => float, Float
    );
    const_getter!(
        /// Gets [Long] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => long, Long
    );
    const_getter!(
        /// Gets [Double] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => double, Double
    );
    const_getter!(
        /// Gets [Class] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => class, Class
    );
    const_getter!(
        /// Gets [String] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => string, String
    );
    const_getter!(
        /// Gets [FieldRef] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => field_ref, FieldRef
    );
    const_getter!(
        /// Gets [MethodRef] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => method_ref, MethodRef
    );
    const_getter!(
        /// Gets [InterfaceMethodRef] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => interface_method_ref, InterfaceMethodRef
    );
    const_getter!(
        /// Gets [NameAndType] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => name_and_type, NameAndType
    );
    const_getter!(
        /// Gets [MethodHandle] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => method_handle, MethodHandle
    );
    const_getter!(
        /// Gets [MethodType] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => method_type, MethodType
    );
    const_getter!(
        /// Gets [Dynamic] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => dynamic, Dynamic
    );
    const_getter!(
        /// Gets [InvokeDynamic] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => invoke_dynamic, InvokeDynamic
    );
    const_getter!(
        /// Gets [InvokeDynamic] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => module, Module
    );
    const_getter!(
        /// Gets [Package] constant reference from constant pool based on given index, returns
        /// [None] if the constant does not exist or match.
        => package, Package
    );
}

impl Deref for ConstantPool {
    type Target = Vec<Option<Constant>>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

/// Represents JVM constant tags.
///
/// See [Table 4.4-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=94).
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl TryFrom<u8> for ConstantTag {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ConstantTag::*;

        let tag = match value {
            1 => Utf8,
            3 => Integer,
            4 => Float,
            5 => Long,
            6 => Double,
            7 => Class,
            8 => String,
            9 => FieldRef,
            10 => MethodRef,
            11 => InterfaceMethodRef,
            12 => NameAndType,
            15 => MethodHandle,
            16 => MethodType,
            17 => Dynamic,
            18 => InvokeDynamic,
            19 => Module,
            20 => Package,
            _ => {
                return Err(ParseError::MatchOutOfBoundUsize(
                    "constant tag",
                    vec!["1", "3..=12", "15..=20"],
                    value as usize,
                ))
            }
        };

        Ok(tag)
    }
}

impl Into<&'static str> for ConstantTag {
    fn into(self) -> &'static str {
        match self {
            ConstantTag::Utf8 => "Utf8",
            ConstantTag::Integer => "Integer",
            ConstantTag::Float => "Float",
            ConstantTag::Long => "Long",
            ConstantTag::Double => "Double",
            ConstantTag::Class => "Class",
            ConstantTag::String => "String",
            ConstantTag::FieldRef => "FieldRef",
            ConstantTag::MethodRef => "MethodRef",
            ConstantTag::InterfaceMethodRef => "InterfaceMethodRef",
            ConstantTag::NameAndType => "NameAndType",
            ConstantTag::MethodHandle => "MethodHandle",
            ConstantTag::MethodType => "MethodType",
            ConstantTag::Dynamic => "Dynamic",
            ConstantTag::InvokeDynamic => "InvokeDynamic",
            ConstantTag::Module => "Module",
            ConstantTag::Package => "Package",
        }
    }
}

/// Represents JVM constants.
///
/// See [4.4 The Constant Pool](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=93).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Utf8(Utf8),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    Class(Class),
    String(String),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndType(NameAndType),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    Dynamic(Dynamic),
    InvokeDynamic(InvokeDynamic),
    Module(Module),
    Package(Package),
}

impl Constant {
    /// Get corresponding [ConstantTag] for current constant.
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Constant::Utf8(..) => ConstantTag::Utf8,
            Constant::Integer(..) => ConstantTag::Integer,
            Constant::Float(..) => ConstantTag::Float,
            Constant::Long(..) => ConstantTag::Long,
            Constant::Double(..) => ConstantTag::Double,
            Constant::Class(..) => ConstantTag::Class,
            Constant::String(..) => ConstantTag::String,
            Constant::FieldRef(..) => ConstantTag::FieldRef,
            Constant::MethodRef(..) => ConstantTag::MethodRef,
            Constant::InterfaceMethodRef(..) => ConstantTag::InterfaceMethodRef,
            Constant::NameAndType(..) => ConstantTag::NameAndType,
            Constant::MethodHandle(..) => ConstantTag::MethodHandle,
            Constant::MethodType(..) => ConstantTag::MethodType,
            Constant::Dynamic(..) => ConstantTag::Dynamic,
            Constant::InvokeDynamic(..) => ConstantTag::InvokeDynamic,
            Constant::Module(..) => ConstantTag::Module,
            Constant::Package(..) => ConstantTag::Package,
        }
    }

    pub const fn occupies_2_slots(&self) -> bool {
        match self {
            Constant::Long(_) | Constant::Double(_) => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &'static str {
        self.tag().into()
    }
}

/// Represents constant UTF8.
///
/// See [4.4.7 The CONSTANT_Utf8_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=102).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Utf8 {
    pub length: u16,
    pub bytes: Vec<u8>,
    #[serde(skip)]
    pub string: RefCell<Option<std::string::String>>,
}

impl Utf8 {
    /// Converts bytes into string.
    pub fn string(&self) -> NodeResResult<std::string::String> {
        let string_ref = self.string.borrow_mut();

        if let Some(string) = string_ref.as_ref() {
            Ok(string.clone())
        } else {
            cesu8::from_java_cesu8(&self.bytes[..])
                .map_err(|_| NodeResError::StringParseFail(self.bytes.clone().into_boxed_slice()))
                .map(|string| string.to_string())
        }
    }
}

/// Represents constant Integer.
///
/// See [4.4.4 The CONSTANT_Integer_info and CONSTANT_Float_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Integer {
    pub bytes: [u8; 4],
}

impl Integer {
    /// Converts bytes into i32.
    pub fn as_i32(&self) -> i32 {
        i32::from_be_bytes(self.bytes)
    }
}

/// Represents constant Float.
///
/// See [4.4.4 The CONSTANT_Integer_info and CONSTANT_Float_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Float {
    pub bytes: [u8; 4],
}

impl Float {
    /// Converts bytes into f32.
    pub fn as_f32(&self) -> f32 {
        f32::from_be_bytes(self.bytes)
    }
}

/// Represents constant Long.
///
/// See [4.4.5 The CONSTANT_Long_info and CONSTANT_Double_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=100).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Long {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

impl Long {
    /// Converts bytes into i64.
    pub fn as_i64(&self) -> i64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.high_bytes);
        bytes[4..].copy_from_slice(&self.low_bytes);

        i64::from_be_bytes(bytes)
    }
}

/// Represents constant Double.
///
/// See [4.4.5 The CONSTANT_Long_info and CONSTANT_Double_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=100).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Double {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

impl Double {
    /// Converts bytes into f64.
    pub fn as_f64(&self) -> f64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.high_bytes);
        bytes[4..].copy_from_slice(&self.low_bytes);

        f64::from_be_bytes(bytes)
    }
}

/// Represents constant Class.
///
/// See [4.4.1 The CONSTANT_Class_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=96).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Class {
    /// Get name of class.
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }
}

/// Represents constant String.
///
/// See [4.4.3 The CONSTANT_String_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct String {
    pub string_index: u16,
}

impl String {
    /// Gets the string of [string](String).
    pub fn string<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.string_index)
    }
}

/// Represents constant FieldRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl FieldRef {
    /// Gets owner class of field.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.class_index)
    }

    /// Gets [NameAndType] of field.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.name_and_type_index)
    }
}

/// Represents constant MethodRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl MethodRef {
    /// Gets owner class of method.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.class_index)
    }

    /// Gets [NameAndType] of method.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.name_and_type_index)
    }
}

/// Represents constant InterfaceMethodRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl InterfaceMethodRef {
    /// Gets owner class of interface method.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.class_index)
    }

    /// Gets [NameAndType] of interface method.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.name_and_type_index)
    }
}

/// Represents constant NameAndType.
///
/// See [4.4.6 The CONSTANT_NameAndType_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=101).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NameAndType {
    pub name_index: u16,
    pub type_index: u16,
}

//noinspection DuplicatedCode
impl NameAndType {
    /// Gets the name of [NameAndType].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }

    /// Gets the type of [NameAndType].
    pub fn typ<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.type_index)
    }
}

/// Represents constant MethodHandle.
///
/// See [4.4.8 The CONSTANT_MethodHandle_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=104).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_index: u16,
}

impl MethodHandle {
    /// Gets [RefKind] of MethodHandle.
    pub fn reference_kind(&self) -> NodeResResult<RefKind> {
        RefKind::try_from(self.reference_kind).map_err(|_| {
            NodeResError::MatchOutOfBound(
                "reference kind",
                vec!["1..=9"],
                self.reference_kind as usize,
            )
        })
    }

    /// Gets the referenced constant of MethodHandle.
    pub fn reference_constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> NodeResResult<Option<&'constant_pool Constant>> {
        if let Some(constant) = constant_pool.get_constant(self.reference_index) {
            match self.reference_kind()? {
                RefKind::GetField | RefKind::GetStatic | RefKind::PutField | RefKind::PutStatic => {
                    if let Constant::FieldRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(NodeResError::MismatchReferenceConstant(
                            "FieldRef",
                            self.reference_index,
                            constant.name(),
                        ))
                    }
                }
                RefKind::InvokeVirtual | RefKind::NewInvokeSpecial => {
                    if let Constant::MethodRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(NodeResError::MismatchReferenceConstant(
                            "MethodRef",
                            self.reference_index,
                            constant.name(),
                        ))
                    }
                }
                RefKind::InvokeStatic | RefKind::InvokeSpecial => {
                    if matches!(
                        constant,
                        Constant::MethodRef(_) | Constant::InterfaceMethodRef(_)
                    ) {
                        Ok(Some(constant))
                    } else {
                        Err(NodeResError::MismatchReferenceConstant(
                            "MethodRef or constant InterfaceMethodRef",
                            self.reference_index,
                            constant.name(),
                        ))
                    }
                }
                RefKind::InvokeInterface => {
                    if let Constant::InterfaceMethodRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(NodeResError::MismatchReferenceConstant(
                            "InterfaceMethodRef",
                            self.reference_index,
                            constant.name(),
                        ))
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}

/// Represents constant MethodType.
///
/// See [4.4.9 The CONSTANT_MethodType_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodType {
    pub descriptor_index: u16,
}

impl MethodType {
    /// Gets descriptor of the MethodType.
    pub fn descriptor<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.descriptor_index)
    }
}

/// Represents constant Dynamic.
///
/// See [4.4.10 The CONSTANT_Dynamic_info and CONSTANT_InvokeDynamic_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl Dynamic {
    /// Gets the [BootstrapMethod] reference [Dynamic] targeting to.
    pub fn bootstrap_method<'bootstrap_method, 'attributes: 'bootstrap_method>(
        &self,
        attribute_infos: &'attributes Vec<AttributeInfo>,
    ) -> Option<&'bootstrap_method BootstrapMethod> {
        let bootstrap_methods = if let Some(AttributeInfo {
            attribute:
                Some(Attribute::BootstrapMethods(BootstrapMethods {
                    bootstrap_methods, ..
                })),
            ..
        }) = attribute_infos.iter().find(|attribute_info| {
            matches!(
                attribute_info.attribute,
                Some(Attribute::BootstrapMethods { .. })
            )
        }) {
            bootstrap_methods
        } else {
            return None;
        };

        bootstrap_methods.get(self.bootstrap_method_attr_index as usize)
    }

    /// Gets the descriptor of the [Dynamic].
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.name_and_type_index)
    }
}

/// Represents constant InvokeDynamic.
///
/// See [4.4.10 The CONSTANT_Dynamic_info and CONSTANT_InvokeDynamic_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
    /// Gets the [BootstrapMethod] reference [InvokeDynamic] targeting to.
    pub fn bootstrap_method<'bootstrap_method, 'attributes: 'bootstrap_method>(
        &self,
        attribute_infos: &'attributes Vec<AttributeInfo>,
    ) -> Option<&'bootstrap_method BootstrapMethod> {
        let bootstrap_methods = if let Some(AttributeInfo {
            attribute:
                Some(Attribute::BootstrapMethods(BootstrapMethods {
                    bootstrap_methods, ..
                })),
            ..
        }) = attribute_infos.iter().find(|attribute_info| {
            matches!(
                attribute_info.attribute,
                Some(Attribute::BootstrapMethods { .. })
            )
        }) {
            bootstrap_methods
        } else {
            return None;
        };

        bootstrap_methods.get(self.bootstrap_method_attr_index as usize)
    }

    /// Gets the descriptor of the [InvokeDynamic].
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.name_and_type_index)
    }
}

/// Represents constant Module.
///
/// See [4.4.11 The CONSTANT_Module_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=107).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Module {
    /// Gets name of [Module].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }
}

/// Represents constant Package.
///
/// See [4.4.12 The CONSTANT_Package_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=108).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Package {
    /// Gets name of [Package].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }
}

/// Represents reference kind used by [MethodHandle].
///
/// See [Table 5.4.3.5-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=396)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RefKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl TryFrom<u8> for RefKind {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use RefKind::*;

        let ref_kind = match value {
            1 => GetField,
            2 => GetStatic,
            3 => PutField,
            4 => PutStatic,
            5 => InvokeVirtual,
            6 => InvokeStatic,
            7 => InvokeSpecial,
            8 => NewInvokeSpecial,
            9 => InvokeInterface,
            _ => {
                return Err(ParseError::MatchOutOfBoundUsize(
                    "reference kind",
                    vec!["1..=9"],
                    value as usize,
                ))
            }
        };

        Ok(ref_kind)
    }
}
