use std::collections::{BTreeMap, HashMap};

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

use crate::asm::node::attribute::{Attribute, AttributeInfo, BootstrapMethod, BootstrapMethods};
use crate::asm::node::ConstantRearrangeable;
use crate::error::{KapiError, KapiResult};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for ConstantPool {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        let mut remapped_entries = BTreeMap::new();

        for (&from, &to) in rearrangements {
            if let Some(entry) = self.get(from) {
                remapped_entries.insert(to, entry.clone());
            } else {
                return Err(KapiError::StateError(format!("Unable to remap constant entry from #{from} to #{to}: Constant #{from} does not exists")));
            }
        }

        self.entries = remapped_entries;

        Ok(())
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
    Hash,
    Serialize,
    Deserialize,
    TryFromPrimitive,
    IntoStaticStr,
)]
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

#[derive(
    Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, IntoStaticStr,
)]
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
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Integer {
    pub bytes: [u8; 4],
}

impl Integer {
    pub fn as_i32(&self) -> i32 {
        i32::from_be_bytes(self.bytes)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Float {
    pub bytes: [u8; 4],
}

impl Float {
    pub fn as_f32(&self) -> f32 {
        f32::from_be_bytes(self.bytes)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for Class {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for String {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.string_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

//noinspection DuplicatedCode
impl ConstantRearrangeable for FieldRef {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.class_index, rearrangements);
        Self::rearrange_index(&mut self.name_and_type_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

//noinspection DuplicatedCode
impl ConstantRearrangeable for MethodRef {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.class_index, rearrangements);
        Self::rearrange_index(&mut self.name_and_type_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

//noinspection DuplicatedCode
impl ConstantRearrangeable for InterfaceMethodRef {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.class_index, rearrangements);
        Self::rearrange_index(&mut self.name_and_type_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for MethodHandle {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.reference_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for MethodType {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.descriptor_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for Dynamic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_and_type_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for InvokeDynamic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_and_type_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for Module {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for Package {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);

        Ok(())
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
    Hash,
    Serialize,
    Deserialize,
    TryFromPrimitive,
)]
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
