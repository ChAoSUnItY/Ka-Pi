use std::collections::BTreeMap;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

use crate::asm::node::attribute::{Attribute, AttributeInfo, BootstrapMethod, BootstrapMethods};
use crate::asm::node::opcode::RefKind;
use crate::error::{KapiError, KapiResult};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstantPool {
    len: u16,
    entries: BTreeMap<u16, Constant>,
}

impl ConstantPool {
    pub(crate) fn add(&mut self, constant: Constant) {
        let is_2 = matches!(constant, Constant::Long { .. } | Constant::Double { .. });

        self.entries.insert(self.len, constant);

        if is_2 {
            self.len += 2;
        } else {
            self.len += 1;
        }
    }

    pub fn get(&self, index: u16) -> Option<&Constant> {
        self.entries.get(&index)
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self {
            len: 1,
            entries: BTreeMap::default(),
        }
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    TryFromPrimitive,
    IntoStaticStr,
)]
pub enum ConstantTag {
    /** The tag value of CONSTANT_Class_info JVMS structures. */
    Class = 7,
    /** The tag value of CONSTANT_Fieldref_info JVMS structures. */
    FieldRef = 9,
    /** The tag value of CONSTANT_Methodref_info JVMS structures. */
    MethodRef = 10,
    /** The tag value of CONSTANT_InterfaceMethodref_info JVMS structures. */
    InterfaceMethodRef = 11,
    /** The tag value of CONSTANT_String_info JVMS structures. */
    String = 8,
    /** The tag value of CONSTANT_Integer_info JVMS structures. */
    Integer = 3,
    /** The tag value of CONSTANT_Float_info JVMS structures. */
    Float = 4,
    /** The tag value of CONSTANT_Long_info JVMS structures. */
    Long = 5,
    /** The tag value of CONSTANT_Double_info JVMS structures. */
    Double = 6,
    /** The tag value of CONSTANT_NameAndType_info JVMS structures. */
    NameAndType = 12,
    /** The tag value of CONSTANT_Utf8_info JVMS structures. */
    Utf8 = 1,
    /** The tag value of CONSTANT_MethodHandle_info JVMS structures. */
    MethodHandle = 15,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    MethodType = 16,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    Dynamic = 17,
    /** The tag value of CONSTANT_Dynamic_info JVMS structures. */
    InvokeDynamic = 18,
    /** The tag value of CONSTANT_InvokeDynamic_info JVMS structures. */
    Module = 19,
    /** The tag value of CONSTANT_Module_info JVMS structures. */
    Package = 20,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, IntoStaticStr)]
pub enum Constant {
    Class(Class),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    String(String),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    NameAndType(NameAndType),
    Utf8(Utf8),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    Dynamic(Dynamic),
    InvokeDynamic(InvokeDynamic),
    Module(Module),
    Package(Package),
}

impl Constant {
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Constant::Class(..) => ConstantTag::Class,
            Constant::FieldRef(..) => ConstantTag::FieldRef,
            Constant::MethodRef(..) => ConstantTag::MethodRef,
            Constant::InterfaceMethodRef(..) => ConstantTag::InterfaceMethodRef,
            Constant::String(..) => ConstantTag::String,
            Constant::Integer(..) => ConstantTag::Integer,
            Constant::Float(..) => ConstantTag::Float,
            Constant::Long(..) => ConstantTag::Long,
            Constant::Double(..) => ConstantTag::Double,
            Constant::NameAndType(..) => ConstantTag::NameAndType,
            Constant::Utf8(..) => ConstantTag::Utf8,
            Constant::MethodHandle(..) => ConstantTag::MethodHandle,
            Constant::MethodType(..) => ConstantTag::MethodType,
            Constant::Dynamic(..) => ConstantTag::Dynamic,
            Constant::InvokeDynamic(..) => ConstantTag::InvokeDynamic,
            Constant::Module(..) => ConstantTag::Module,
            Constant::Package(..) => ConstantTag::Package,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Class {
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.name_index) {
            Some(constant)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FieldRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl FieldRef {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.class_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) =
            constant_pool.get(self.name_and_type_index)
        {
            Some(name_and_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl MethodRef {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.class_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) =
            constant_pool.get(self.name_and_type_index)
        {
            Some(name_and_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InterfaceMethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl InterfaceMethodRef {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.class_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) =
            constant_pool.get(self.name_and_type_index)
        {
            Some(name_and_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct String {
    pub string_index: u16,
}

impl String {
    pub fn string<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.string_index) {
            Some(constant)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Integer {
    pub bytes: [u8; 4],
}

impl Integer {
    pub fn as_i32(&self) -> i32 {
        i32::from_be_bytes(self.bytes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Float {
    pub bytes: [u8; 4],
}

impl Float {
    pub fn as_f32(&self) -> f32 {
        f32::from_be_bytes(self.bytes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Long {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

impl Long {
    pub fn as_i64(&self) -> i64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.high_bytes);
        bytes[4..].copy_from_slice(&self.low_bytes);

        i64::from_be_bytes(bytes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Double {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

impl Double {
    pub fn as_f64(&self) -> f64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.high_bytes);
        bytes[4..].copy_from_slice(&self.low_bytes);

        f64::from_be_bytes(bytes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NameAndType {
    pub name_index: u16,
    pub type_index: u16,
}

//noinspection DuplicatedCode
impl NameAndType {
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.name_index) {
            Some(constant)
        } else {
            None
        }
    }

    pub fn typ<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.type_index) {
            Some(constant)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Utf8 {
    pub length: u16,
    pub bytes: Vec<u8>,
}

impl Utf8 {
    pub fn string(&self) -> KapiResult<std::string::String> {
        cesu8::from_java_cesu8(&self.bytes[..])
            .map_err(|err| {
                KapiError::ClassParseError(format!(
                    "Unable to convert bytes to string, reason: {}",
                    err.to_string()
                ))
            })
            .map(|string| string.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_index: u16,
}

impl MethodHandle {
    pub fn reference_kind(&self) -> KapiResult<RefKind> {
        RefKind::try_from(self.reference_kind).map_err(|err| {
            KapiError::ClassParseError(format!(
                "Reference kind {} does not match any kinds described in specification, reason: {}",
                err.number,
                err.to_string()
            ))
        })
    }

    pub fn reference_constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> KapiResult<Option<&'constant_pool Constant>> {
        if let Some(constant) = constant_pool.get(self.reference_index) {
            match self.reference_kind()? {
                RefKind::GetField | RefKind::GetStatic | RefKind::PutField | RefKind::PutStatic => {
                    if let Constant::FieldRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant FieldRef at #{} but got {}",
                            self.reference_index,
                            Into::<&'static str>::into(constant)
                        )))
                    }
                }
                RefKind::InvokeVirtual | RefKind::NewInvokeSpecial => {
                    if let Constant::MethodRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant MethodRef at #{} but got {}",
                            self.reference_index,
                            Into::<&'static str>::into(constant)
                        )))
                    }
                }
                RefKind::InvokeStatic | RefKind::InvokeSpecial => {
                    if matches!(
                        constant,
                        Constant::MethodRef(_) | Constant::InterfaceMethodRef(_)
                    ) {
                        Ok(Some(constant))
                    } else {
                        Err(KapiError::ClassParseError(format!(
                            "Expected referenced either constant MethodRef or constant InterfaceMethodRef at #{} but got {}",
                            self.reference_index,
                            Into::<&'static str>::into(constant)
                        )))
                    }
                }
                RefKind::InvokeInterface => {
                    if let Constant::InterfaceMethodRef(_) = constant {
                        Ok(Some(constant))
                    } else {
                        Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant InterfaceMethodRef at #{} but got {}",
                            self.reference_index,
                            Into::<&'static str>::into(constant)
                        )))
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodType {
    pub descriptor_index: u16,
}

impl MethodType {
    pub fn descriptor<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.descriptor_index) {
            Some(constant)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl Dynamic {
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

    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) =
            constant_pool.get(self.name_and_type_index)
        {
            Some(name_and_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
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

    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) =
            constant_pool.get(self.name_and_type_index)
        {
            Some(name_and_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Module {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Module {
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.name_index) {
            Some(constant)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Package {
    pub name_index: u16,
}

//noinspection DuplicatedCode
impl Package {
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(constant)) = constant_pool.get(self.name_index) {
            Some(constant)
        } else {
            None
        }
    }
}
