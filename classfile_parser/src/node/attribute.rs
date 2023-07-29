use serde::{Deserialize, Serialize};

use crate::node;
use crate::node::access_flag::{ModuleAccessFlag, NestedClassAccessFlag, ParameterAccessFlag};
use crate::node::attribute::annotation::{
    Annotation, ElementValue, ParameterAnnotation, TypeAnnotation,
};
use crate::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::node::constant::{Class, Constant, ConstantPool, MethodHandle, NameAndType, Utf8};
use crate::node::error::{NodeResError, NodeResResult};
use crate::node::opcode::Instruction;
use crate::parse::{class_signature, field_signature, method_signature};

pub mod annotation;
pub mod module;

pub(crate) const CONSTANT_VALUE: &'static str = "ConstantValue";
pub(crate) const CODE: &'static str = "Code";
pub(crate) const STACK_MAP_TABLE: &'static str = "StackMapTable";
pub(crate) const EXCEPTIONS: &'static str = "Exceptions";
pub(crate) const INNER_CLASSES: &'static str = "InnerClasses";
pub(crate) const ENCLOSING_METHOD: &'static str = "EnclosingMethod";
pub(crate) const SYNTHETIC: &'static str = "Synthetic";
pub(crate) const SIGNATURE: &'static str = "Signature";
pub(crate) const SOURCE_FILE: &'static str = "SourceFile";
pub(crate) const SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
pub(crate) const LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
pub(crate) const LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
pub(crate) const LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
pub(crate) const DEPRECATED: &'static str = "Deprecated";
pub(crate) const RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
pub(crate) const RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
pub(crate) const RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeVisibleParameterAnnotations";
pub(crate) const RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeInvisibleParameterAnnotations";
pub(crate) const RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
pub(crate) const RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str =
    "RuntimeInvisibleTypeAnnotations";
pub(crate) const ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
pub(crate) const BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
pub(crate) const METHOD_PARAMETERS: &'static str = "MethodParameters";
pub(crate) const MODULE: &'static str = "Module";
pub(crate) const MODULE_PACKAGES: &'static str = "ModulePackages";
pub(crate) const MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
pub(crate) const NEST_HOST: &'static str = "NestHost";
pub(crate) const NEST_MEMBERS: &'static str = "NestMembers";
pub(crate) const PERMITTED_SUBCLASSES: &'static str = "PermittedSubclasses";
pub(crate) const RECORD: &'static str = "Record";

/// Represents an attribute info.
///
/// See [4.7 Attributes](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=371).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_len: u32,
    pub info: Vec<u8>,
    pub attribute: Option<Attribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code),
    StackMapTable(StackMapTable),
    Exceptions(Exceptions),
    InnerClasses(InnerClasses),
    EnclosingMethod(EnclosingMethod),
    Synthetic,
    Signature(Signature),
    SourceFile(SourceFile),
    SourceDebugExtension(SourceDebugExtension),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    LocalVariableTypeTable(LocalVariableTypeTable),
    Deprecate,
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations),
    AnnotationDefault(AnnotationDefault),
    BootstrapMethods(BootstrapMethods),
    MethodParameters(MethodParameters),
    Module(Module),
    ModulePackages(ModulePackages),
    ModuleMainClass(ModuleMainClass),
    NestHost(NestHost),
    NestMembers(NestMembers),
    Record(Record),
    PermittedSubclasses(PermittedSubclasses),
}

impl Attribute {
    pub const fn name(&self) -> &'static str {
        match self {
            Attribute::ConstantValue(..) => CONSTANT_VALUE,
            Attribute::Code(..) => CODE,
            Attribute::StackMapTable(..) => STACK_MAP_TABLE,
            Attribute::Exceptions(..) => EXCEPTIONS,
            Attribute::InnerClasses(..) => INNER_CLASSES,
            Attribute::EnclosingMethod(..) => ENCLOSING_METHOD,
            Attribute::Synthetic => SYNTHETIC,
            Attribute::Signature(..) => SIGNATURE,
            Attribute::SourceFile(..) => SOURCE_FILE,
            Attribute::SourceDebugExtension(..) => SOURCE_DEBUG_EXTENSION,
            Attribute::LineNumberTable(..) => LINE_NUMBER_TABLE,
            Attribute::LocalVariableTable(..) => LOCAL_VARIABLE_TABLE,
            Attribute::LocalVariableTypeTable(..) => LOCAL_VARIABLE_TYPE_TABLE,
            Attribute::Deprecate => DEPRECATED,
            Attribute::RuntimeVisibleAnnotations(..) => RUNTIME_VISIBLE_ANNOTATIONS,
            Attribute::RuntimeInvisibleAnnotations(..) => RUNTIME_INVISIBLE_ANNOTATIONS,
            Attribute::RuntimeVisibleParameterAnnotations(..) => {
                RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS
            }
            Attribute::RuntimeInvisibleParameterAnnotations(..) => {
                RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS
            }
            Attribute::RuntimeVisibleTypeAnnotations(..) => RUNTIME_VISIBLE_TYPE_ANNOTATIONS,
            Attribute::RuntimeInvisibleTypeAnnotations(..) => RUNTIME_INVISIBLE_TYPE_ANNOTATIONS,
            Attribute::AnnotationDefault(..) => ANNOTATION_DEFAULT,
            Attribute::BootstrapMethods(..) => BOOTSTRAP_METHODS,
            Attribute::MethodParameters(..) => METHOD_PARAMETERS,
            Attribute::Module(..) => MODULE,
            Attribute::ModulePackages(..) => MODULE_PACKAGES,
            Attribute::ModuleMainClass(..) => MODULE_MAIN_CLASS,
            Attribute::NestHost(..) => NEST_HOST,
            Attribute::NestMembers(..) => NEST_MEMBERS,
            Attribute::Record(..) => RECORD,
            Attribute::PermittedSubclasses(..) => PERMITTED_SUBCLASSES,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConstantValue {
    pub constant_value_index: u16,
}

impl ConstantValue {
    pub fn constant<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(self.constant_value_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub instructions: Vec<Instruction>,
    pub exception_table_length: u16,
    pub exception_table: Vec<Exception>,
    pub attributes_length: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StackMapTable {
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrameEntry>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exceptions {
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerClasses {
    pub number_of_classes: u16,
    pub class: Vec<InnerClass>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

impl EnclosingMethod {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.class_index)
    }

    pub fn method_name_and_type<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(self.method_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub signature_index: u16,
}

//noinspection DuplicatedCode
impl Signature {
    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.signature_index)
    }

    /// Parses current signature attribute's string into class signature. Returns [Err] on either
    /// invalid constant reference or signature format.
    pub fn as_class_signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> NodeResResult<node::signature::Signature> {
        let signature_str = self
            .signature(constant_pool)
            .ok_or(NodeResError::UnknownConstantReference(self.signature_index))?;

        class_signature(&signature_str.string()?).map_err(|err| NodeResError::ParseFail(err))
    }

    /// Parses current signature attribute's string into field signature. Returns [Err] on either
    /// invalid constant reference or signature format.
    pub fn as_field_signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> NodeResResult<node::signature::Signature> {
        let signature_str = self
            .signature(constant_pool)
            .ok_or(NodeResError::UnknownConstantReference(self.signature_index))?;

        field_signature(&signature_str.string()?).map_err(|err| NodeResError::ParseFail(err))
    }

    /// Parses current signature attribute's string into method signature. Returns [Err] on either
    /// invalid constant reference or signature format.
    pub fn as_method_signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> NodeResResult<node::signature::Signature> {
        let signature_str = self
            .signature(constant_pool)
            .ok_or(NodeResError::UnknownConstantReference(self.signature_index))?;

        method_signature(&signature_str.string()?).map_err(|err| NodeResError::ParseFail(err))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    pub source_file_index: u16,
}

impl SourceFile {
    pub fn source_file<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.source_file_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceDebugExtension {
    pub debug_extension: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LineNumberTable {
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumber>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableTable {
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariable>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleParameterAnnotations {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleTypeAnnotations {
    pub num_annotations: u16,
    pub type_annotations: Vec<TypeAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub num_annotations: u16,
    pub type_annotations: Vec<TypeAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnnotationDefault {
    pub default_value: ElementValue,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapMethods {
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodParameters {
    pub parameters_count: u8,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub module_name_index: u16,
    pub module_flags: ModuleAccessFlag,
    pub module_version_index: u16,
    pub requires_count: u16,
    pub requires: Vec<Requires>,
    pub exports_count: u16,
    pub exports: Vec<Exports>,
    pub opens_count: u16,
    pub opens: Vec<Opens>,
    pub uses_count: u16,
    pub uses_index: Vec<u16>,
    pub provides_count: u16,
    pub provides: Vec<Provides>,
}

impl Module {
    pub fn module_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.module_name_index)
    }

    pub fn module_version<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.module_version_index)
    }

    pub fn uses<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        self.uses_index
            .get(index)
            .and_then(|uses_index| constant_pool.get_class(*uses_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModulePackages {
    pub package_count: u16,
    pub package_index: Vec<u16>,
}

impl ModulePackages {
    pub fn package<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        self.package_index
            .get(index)
            .and_then(|package_index| constant_pool.get_utf8(*package_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleMainClass {
    pub main_class_index: u16,
}

impl ModuleMainClass {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.main_class_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestHost {
    pub host_class_index: u16,
}

impl NestHost {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.host_class_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestMembers {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

//noinspection DuplicatedCode
impl NestMembers {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        self.classes
            .get(index)
            .and_then(|class_index| constant_pool.get_class(*class_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub components_count: u16,
    pub components: Vec<RecordComponent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PermittedSubclasses {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

//noinspection DuplicatedCode
impl PermittedSubclasses {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        self.classes
            .get(index)
            .and_then(|class_index| constant_pool.get_class(*class_index))
    }
}

// Inner structs

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl Exception {
    pub fn catch_type<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.catch_type)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum StackMapFrameEntry {
    Same {
        frame_type: u8,
    },
    SameLocal1StackItem {
        frame_type: u8,
        stack: VerificationType,
    },
    SameLocal1StackItemExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationType,
    },
    Chop {
        frame_type: u8,
        offset_delta: u16,
    },
    SameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    Append {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },
    Full {
        frame_type: u8,
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationType>,
        number_of_stack_items: u16,
        stack: Vec<VerificationType>,
    },
}

//noinspection SpellCheckingInspection
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object(Object),
    Uninitialized { offset: u16 },
}

//noinspection SpellCheckingInspection

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub(crate) cpool_index: u16,
}

//noinspection DuplicatedCode
impl Object {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.cpool_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: NestedClassAccessFlag,
}

impl InnerClass {
    pub fn inner_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.inner_class_info_index)
    }

    pub fn outer_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(self.outer_class_info_index)
    }

    pub fn inner_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.inner_name_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

//noinspection DuplicatedCode
impl LocalVariable {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.descriptor_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

//noinspection DuplicatedCode
impl LocalVariableType {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }

    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.signature_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    pub fn new(bootstrap_method_ref: u16, boostrap_arguments_indices: Vec<u16>) -> Self {
        Self {
            bootstrap_method_ref,
            num_bootstrap_arguments: boostrap_arguments_indices.len() as u16,
            bootstrap_arguments: boostrap_arguments_indices,
        }
    }

    pub fn bootstrap_method_handle<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodHandle> {
        constant_pool.get_method_handle(self.bootstrap_method_ref)
    }

    pub fn bootstrap_argument<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        self.bootstrap_arguments
            .get(index)
            .and_then(|argument_index| constant_pool.get_constant(*argument_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: ParameterAccessFlag,
}

//noinspection DuplicatedCode
impl MethodParameter {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordComponent {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

//noinspection DuplicatedCode
impl RecordComponent {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.name_index)
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.descriptor_index)
    }
}
