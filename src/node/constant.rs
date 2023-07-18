use std::collections::BTreeMap;
use std::ops::Deref;

use num_enum::TryFromPrimitive;
use paste::paste;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

use crate::error::{KapiError, KapiResult};
use crate::node::attribute::{Attribute, AttributeInfo, BootstrapMethod, BootstrapMethods};
use crate::node::Node;
use crate::visitor::constant::ConstantVisitor;
use crate::visitor::Visitable;

macro_rules! const_getter {
    ($(#[$attr:meta])* => $variant_name: ident, $variant: ty) => {
        paste! {
            $(#[$attr])*
            pub fn [< get_ $variant_name >] (&self, index: u16) -> Option<&$variant> {
                if let Some(Node(
                    _,
                    ConstantInfo {
                        tag: _,
                        constant: Node(_, Constant:: $variant(constant)),
                    },
                )) = self.get(index)
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
    entries: BTreeMap<u16, Node<ConstantInfo>>,
}

impl ConstantPool {
    pub fn len(&self) -> u16 {
        self.len
    }

    pub(crate) fn add(&mut self, cp_info: Node<ConstantInfo>) {
        let occupies_2_slots = cp_info.occupies_2_slots();

        self.entries.insert(self.len, cp_info);

        if occupies_2_slots {
            self.len += 2;
        } else {
            self.len += 1;
        }
    }

    /// Get [ConstantInfo] node reference from constant pool based on given index.
    ///
    /// ### Index
    ///
    /// There are two scenarios which will return a [None]:
    /// 1. The given index is out of bound, which is 0 < index <= self.len().
    /// 2. The given index is located at an placeholder constant, which is followed after [Constant::Long]
    ///    and [Constant::Double].
    pub fn get(&self, index: u16) -> Option<&Node<ConstantInfo>> {
        self.entries.get(&index)
    }

    pub fn get_constant(&self, index: u16) -> Option<&Constant> {
        self.get(index).map(|node| &*node.constant)
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

impl Default for ConstantPool {
    fn default() -> Self {
        Self {
            len: 1,
            entries: BTreeMap::default(),
        }
    }
}

impl Deref for ConstantPool {
    type Target = BTreeMap<u16, Node<ConstantInfo>>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl From<Vec<Node<ConstantInfo>>> for ConstantPool {
    fn from(value: Vec<Node<ConstantInfo>>) -> Self {
        let mut constant_pool = ConstantPool::default();

        for item in value {
            constant_pool.add(item);
        }

        constant_pool
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConstantInfo {
    pub tag: Node<ConstantTag>,
    pub constant: Node<Constant>,
}

impl ConstantInfo {
    pub fn occupies_2_slots(&self) -> bool {
        matches!(*self.tag, ConstantTag::Long | ConstantTag::Double)
    }
}

/// Represents JVM constant tags.
///
/// See [Table 4.4-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=94).
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

/// Represents JVM constants.
///
/// See [4.4 The Constant Pool](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=93).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, IntoStaticStr)]
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
}

impl<CV> Visitable<CV> for Constant
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        match self {
            Constant::Utf8(utf8) => utf8.visit(visitor),
            Constant::Integer(integer) => integer.visit(visitor),
            Constant::Float(float) => float.visit(visitor),
            Constant::Long(long) => long.visit(visitor),
            Constant::Double(double) => double.visit(visitor),
            Constant::Class(class) => class.visit(visitor),
            Constant::String(string) => string.visit(visitor),
            Constant::FieldRef(field_ref) => field_ref.visit(visitor),
            Constant::MethodRef(method_ref) => method_ref.visit(visitor),
            Constant::InterfaceMethodRef(interface_method_ref) => {
                interface_method_ref.visit(visitor)
            }
            Constant::NameAndType(name_and_type) => name_and_type.visit(visitor),
            Constant::MethodHandle(method_handle) => method_handle.visit(visitor),
            Constant::MethodType(method_type) => method_type.visit(visitor),
            Constant::Dynamic(dynamic) => dynamic.visit(visitor),
            Constant::InvokeDynamic(invoke_dynamic) => invoke_dynamic.visit(visitor),
            Constant::Module(module) => module.visit(visitor),
            Constant::Package(package) => package.visit(visitor),
        }
    }
}

/// Represents constant UTF8.
///
/// See [4.4.7 The CONSTANT_Utf8_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=102).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Utf8 {
    pub length: Node<u16>,
    pub bytes: Node<Vec<u8>>,
}

impl Utf8 {
    /// Converts bytes into string.
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

impl<CV> Visitable<CV> for Utf8
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_utf8(self);
    }
}

/// Represents constant Integer.
///
/// See [4.4.4 The CONSTANT_Integer_info and CONSTANT_Float_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Integer {
    pub bytes: Node<[u8; 4]>,
}

impl Integer {
    /// Converts bytes into i32.
    pub fn as_i32(&self) -> i32 {
        i32::from_be_bytes(*self.bytes)
    }
}

impl<CV> Visitable<CV> for Integer
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_integer(self);
    }
}

/// Represents constant Float.
///
/// See [4.4.4 The CONSTANT_Integer_info and CONSTANT_Float_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Float {
    pub bytes: Node<[u8; 4]>,
}

impl Float {
    /// Converts bytes into f32.
    pub fn as_f32(&self) -> f32 {
        f32::from_be_bytes(*self.bytes)
    }
}

impl<CV> Visitable<CV> for Float
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_float(self);
    }
}

/// Represents constant Long.
///
/// See [4.4.5 The CONSTANT_Long_info and CONSTANT_Double_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=100).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Long {
    pub high_bytes: Node<[u8; 4]>,
    pub low_bytes: Node<[u8; 4]>,
}

impl Long {
    /// Converts bytes into i64.
    pub fn as_i64(&self) -> i64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&*self.high_bytes);
        bytes[4..].copy_from_slice(&*self.low_bytes);

        i64::from_be_bytes(bytes)
    }
}

impl<CV> Visitable<CV> for Long
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_long(self);
    }
}

/// Represents constant Double.
///
/// See [4.4.5 The CONSTANT_Long_info and CONSTANT_Double_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=100).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Double {
    pub high_bytes: Node<[u8; 4]>,
    pub low_bytes: Node<[u8; 4]>,
}

impl Double {
    /// Converts bytes into f64.
    pub fn as_f64(&self) -> f64 {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&*self.high_bytes);
        bytes[4..].copy_from_slice(&*self.low_bytes);

        f64::from_be_bytes(bytes)
    }
}

impl<CV> Visitable<CV> for Double
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_double(self);
    }
}

/// Represents constant Class.
///
/// See [4.4 The CONSTANT_Class_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=96).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub name_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Class {
    /// Get name of class.
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }
}

impl<CV> Visitable<CV> for Class
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_class(self);
    }
}

/// Represents constant String.
///
/// See [4.4.3 The CONSTANT_String_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=98).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct String {
    pub string_index: Node<u16>,
}

impl String {
    /// Gets the string of [string](String).
    pub fn string<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.string_index)
    }
}

impl<CV> Visitable<CV> for String
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_string(self);
    }
}

/// Represents constant FieldRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldRef {
    pub class_index: Node<u16>,
    pub name_and_type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl FieldRef {
    /// Gets owner class of field.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.class_index)
    }

    /// Gets [NameAndType] of field.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.name_and_type_index)
    }
}

impl<CV> Visitable<CV> for FieldRef
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_field_ref(self);
    }
}

/// Represents constant MethodRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodRef {
    pub class_index: Node<u16>,
    pub name_and_type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl MethodRef {
    /// Gets owner class of method.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.class_index)
    }

    /// Gets [NameAndType] of method.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.class_index)
    }
}

impl<CV> Visitable<CV> for MethodRef
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_method_ref(self);
    }
}

/// Represents constant InterfaceMethodRef.
///
/// See [4.4.2 The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=97).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethodRef {
    pub class_index: Node<u16>,
    pub name_and_type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl InterfaceMethodRef {
    /// Gets owner class of interface method.
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.class_index)
    }

    /// Gets [NameAndType] of interface method.
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.name_and_type_index)
    }
}

impl<CV> Visitable<CV> for InterfaceMethodRef
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_interface_method_ref(self);
    }
}

/// Represents constant NameAndType.
///
/// See [4.4.6 The CONSTANT_NameAndType_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=101).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NameAndType {
    pub name_index: Node<u16>,
    pub type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl NameAndType {
    /// Gets the name of [NameAndType].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }

    /// Gets the type of [NameAndType].
    pub fn typ<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.type_index)
    }
}

impl<CV> Visitable<CV> for NameAndType
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_name_and_type(self);
    }
}

/// Represents constant MethodHandle.
///
/// See [4.4.8 The CONSTANT_MethodHandle_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=104).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodHandle {
    pub reference_kind: Node<RefKind>,
    pub reference_index: Node<u16>,
}

impl MethodHandle {
    /// Gets [RefKind] of MethodHandle.
    pub fn reference_kind(&self) -> RefKind {
        *self.reference_kind
    }

    /// Gets the referenced constant of MethodHandle.
    pub fn reference_constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> KapiResult<Option<&'constant_pool Constant>> {
        if let Some(constant) = constant_pool.get(*self.reference_index) {
            match *self.reference_kind {
                RefKind::GetField | RefKind::GetStatic | RefKind::PutField | RefKind::PutStatic => {
                    match &*constant.constant {
                        constant @ Constant::FieldRef(_) => Ok(Some(constant)),
                        _ => Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant FieldRef at #{} but got {}",
                            *self.reference_index,
                            Into::<&'static str>::into(&*constant.constant)
                        )))
                    }
                }
                RefKind::InvokeVirtual | RefKind::NewInvokeSpecial => {
                    match &*constant.constant {
                        constant @ Constant::MethodRef(_) => Ok(Some(constant)),
                        _ => Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant MethodRef at #{} but got {}",
                            *self.reference_index,
                            Into::<&'static str>::into(&*constant.constant)
                        )))
                    }
                }
                RefKind::InvokeStatic | RefKind::InvokeSpecial => {
                    match &*constant.constant {
                        constant @ Constant::MethodRef(_) => Ok(Some(constant)),
                        constant @ Constant::InterfaceMethodRef(_) => Ok(Some(constant)),
                        _ => Err(KapiError::ClassParseError(format!(
                            "Expected referenced either constant MethodRef or constant InterfaceMethodRef at #{} but got {}",
                            *self.reference_index,
                            Into::<&'static str>::into(&*constant.constant)
                        )))
                    }
                }
                RefKind::InvokeInterface => {
                    match &*constant.constant {
                        constant @ Constant::InterfaceMethodRef(_) => Ok(Some(constant)),
                        _ => Err(KapiError::ClassParseError(format!(
                            "Expected referenced constant InterfaceMethodRef at #{} but got {}",
                            *self.reference_index,
                            Into::<&'static str>::into(&*constant.constant)
                        )))
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl<CV> Visitable<CV> for MethodHandle
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_method_handle(self);
    }
}

/// Represents constant MethodType.
///
/// See [4.4.9 The CONSTANT_MethodType_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodType {
    pub descriptor_index: Node<u16>,
}

impl MethodType {
    /// Gets descriptor of the MethodType.
    pub fn descriptor<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.descriptor_index)
    }
}

impl<CV> Visitable<CV> for MethodType
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_method_type(self);
    }
}

/// Represents constant Dynamic.
///
/// See [4.4.10 The CONSTANT_Dynamic_info and CONSTANT_InvokeDynamic_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: Node<u16>,
    pub name_and_type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Dynamic {
    /// Gets the [BootstrapMethod] reference [Dynamic] targeting to.
    pub fn bootstrap_method<'bootstrap_method, 'attributes: 'bootstrap_method>(
        &self,
        attribute_infos: &'attributes Vec<AttributeInfo>,
    ) -> Option<&'bootstrap_method Node<BootstrapMethod>> {
        let bootstrap_methods = if let Some(AttributeInfo {
            attribute:
                Some(Node(
                    _,
                    Attribute::BootstrapMethods(BootstrapMethods {
                        bootstrap_methods, ..
                    }),
                )),
            ..
        }) = attribute_infos.iter().find(|attribute_info| {
            matches!(
                attribute_info.attribute,
                Some(Node(_, Attribute::BootstrapMethods { .. }))
            )
        }) {
            bootstrap_methods
        } else {
            return None;
        };

        bootstrap_methods.get(*self.bootstrap_method_attr_index as usize)
    }

    /// Gets the descriptor of the [Dynamic].
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.name_and_type_index)
    }
}

impl<CV> Visitable<CV> for Dynamic
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_dynamic(self);
    }
}

/// Represents constant InvokeDynamic.
///
/// See [4.4.10 The CONSTANT_Dynamic_info and CONSTANT_InvokeDynamic_info Structures](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=106).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: Node<u16>,
    pub name_and_type_index: Node<u16>,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
    /// Gets the [BootstrapMethod] reference [InvokeDynamic] targeting to.
    pub fn bootstrap_method<'bootstrap_method, 'attributes: 'bootstrap_method>(
        &self,
        attribute_infos: &'attributes Vec<AttributeInfo>,
    ) -> Option<&'bootstrap_method Node<BootstrapMethod>> {
        let bootstrap_methods = if let Some(AttributeInfo {
            attribute:
                Some(Node(
                    _,
                    Attribute::BootstrapMethods(BootstrapMethods {
                        bootstrap_methods, ..
                    }),
                )),
            ..
        }) = attribute_infos.iter().find(|attribute_info| {
            matches!(
                attribute_info.attribute,
                Some(Node(_, Attribute::BootstrapMethods { .. }))
            )
        }) {
            bootstrap_methods
        } else {
            return None;
        };

        bootstrap_methods.get(*self.bootstrap_method_attr_index as usize)
    }

    /// Gets the descriptor of the [InvokeDynamic].
    pub fn name_and_type<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.name_and_type_index)
    }
}

impl<CV> Visitable<CV> for InvokeDynamic
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_invoke_dynamic(self);
    }
}

/// Represents constant Module.
///
/// See [4.4.11 The CONSTANT_Module_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=107).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Module {
    /// Gets name of [Module].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }
}

impl<CV> Visitable<CV> for Module
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_module(self);
    }
}

/// Represents constant Package.
///
/// See [4.4.12 The CONSTANT_Package_info Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=108).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Package {
    /// Gets name of [Package].
    pub fn name<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }
}

impl<CV> Visitable<CV> for Package
where
    CV: ConstantVisitor,
{
    fn visit(&self, visitor: &mut CV) {
        visitor.visit_package(self);
    }
}

/// Represents reference kind used by [MethodHandle].
///
/// See [Table 5.4.3.5-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=396)
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
