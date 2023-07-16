use serde::{Deserialize, Serialize};

use crate::node::access_flag::{ModuleAccessFlag, NestedClassAccessFlag, ParameterAccessFlag};
use crate::node::attribute::annotation::{
    Annotation, ElementValue, ParameterAnnotation, TypeAnnotation,
};
use crate::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::node::constant::{Class, Constant, ConstantPool, MethodHandle, NameAndType, Utf8};
use crate::node::opcode::Instruction;
use crate::node::{Node, Nodes};

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
    pub attribute_name_index: Node<u16>,
    pub attribute_len: Node<u32>,
    pub info: Node<Vec<u8>>,
    pub attribute: Option<Node<Attribute>>,
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
    pub constant_value_index: Node<u16>,
}

impl ConstantValue {
    pub fn constant<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(*self.constant_value_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Code {
    pub max_stack: Node<u16>,
    pub max_locals: Node<u16>,
    pub code_length: Node<u32>,
    pub code: Node<Vec<u8>>,
    pub instructions: Nodes<Instruction>,
    pub exception_table_length: Node<u16>,
    pub exception_table: Nodes<Exception>,
    pub attributes_length: Node<u16>,
    pub attributes: Nodes<AttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StackMapTable {
    pub number_of_entries: Node<u16>,
    pub entries: Nodes<StackMapFrameEntry>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exceptions {
    pub number_of_exceptions: Node<u16>,
    pub exception_index_table: Nodes<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerClasses {
    pub number_of_classes: Node<u16>,
    pub class: Nodes<InnerClass>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnclosingMethod {
    pub class_index: Node<u16>,
    pub method_index: Node<u16>,
}

impl EnclosingMethod {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.class_index)
    }

    pub fn method_name_and_type<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        constant_pool.get_name_and_type(*self.method_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub signature_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Signature {
    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.signature_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    pub source_file_index: Node<u16>,
}

impl SourceFile {
    pub fn source_file<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.source_file_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceDebugExtension {
    pub debug_extension: Node<Vec<u8>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LineNumberTable {
    pub line_number_table_length: Node<u16>,
    pub line_number_table: Nodes<LineNumber>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableTable {
    pub local_variable_table_length: Node<u16>,
    pub local_variable_table: Nodes<LocalVariable>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table_length: Node<u16>,
    pub local_variable_type_table: Nodes<LocalVariableType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleAnnotations {
    pub num_annotations: Node<u16>,
    pub annotations: Nodes<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleAnnotations {
    pub num_annotations: Node<u16>,
    pub annotations: Nodes<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleParameterAnnotations {
    pub num_parameters: Node<u16>,
    pub parameter_annotations: Nodes<ParameterAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub num_parameters: Node<u16>,
    pub parameter_annotations: Nodes<ParameterAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVisibleTypeAnnotations {
    pub num_annotations: Node<u16>,
    pub type_annotations: Nodes<TypeAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub num_annotations: Node<u16>,
    pub type_annotations: Nodes<TypeAnnotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnnotationDefault {
    pub default_value: Node<ElementValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapMethods {
    pub num_bootstrap_methods: Node<u16>,
    pub bootstrap_methods: Nodes<BootstrapMethod>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodParameters {
    pub parameters_count: Node<u8>,
    pub parameters: Nodes<MethodParameter>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub module_name_index: Node<u16>,
    pub module_flags: Node<Vec<ModuleAccessFlag>>,
    pub module_version_index: Node<u16>,
    pub requires_count: Node<u16>,
    pub requires: Nodes<Requires>,
    pub exports_count: Node<u16>,
    pub exports: Nodes<Exports>,
    pub opens_count: Node<u16>,
    pub opens: Nodes<Opens>,
    pub uses_count: Node<u16>,
    pub uses_index: Nodes<u16>,
    pub provides_count: Node<u16>,
    pub provides: Nodes<Provides>,
}

impl Module {
    pub fn module_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.module_name_index)
    }

    pub fn module_version<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.module_version_index)
    }

    pub fn uses<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        self.uses_index
            .get(index)
            .and_then(|uses_index| constant_pool.get_class(**uses_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModulePackages {
    pub package_count: Node<u16>,
    pub package_index: Nodes<u16>,
}

impl ModulePackages {
    pub fn package<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        self.package_index
            .get(index)
            .and_then(|package_index| constant_pool.get_utf8(**package_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleMainClass {
    pub main_class_index: Node<u16>,
}

impl ModuleMainClass {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.main_class_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestHost {
    pub host_class_index: Node<u16>,
}

impl NestHost {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.host_class_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestMembers {
    pub number_of_classes: Node<u16>,
    pub classes: Nodes<u16>,
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
            .and_then(|class_index| constant_pool.get_class(**class_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub components_count: Node<u16>,
    pub components: Nodes<RecordComponent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PermittedSubclasses {
    pub number_of_classes: Node<u16>,
    pub classes: Nodes<u16>,
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
            .and_then(|class_index| constant_pool.get_class(**class_index))
    }
}

// Inner structs

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exception {
    pub start_pc: Node<u16>,
    pub end_pc: Node<u16>,
    pub handler_pc: Node<u16>,
    pub catch_type: Node<u16>,
}

impl Exception {
    pub fn catch_type<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.catch_type)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum StackMapFrameEntry {
    Same {
        frame_type: Node<u8>,
    },
    SameLocal1StackItem {
        frame_type: Node<u8>,
        stack: Node<VerificationType>,
    },
    SameLocal1StackItemExtended {
        frame_type: Node<u8>,
        offset_delta: Node<u16>,
        stack: Node<VerificationType>,
    },
    Chop {
        frame_type: Node<u8>,
        offset_delta: Node<u16>,
    },
    SameExtended {
        frame_type: Node<u8>,
        offset_delta: Node<u16>,
    },
    Append {
        frame_type: Node<u8>,
        offset_delta: Node<u16>,
        locals: Nodes<VerificationType>,
    },
    Full {
        frame_type: Node<u8>,
        offset_delta: Node<u16>,
        number_of_locals: Node<u16>,
        locals: Nodes<VerificationType>,
        number_of_stack_items: Node<u16>,
        stack: Nodes<VerificationType>,
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
    Uninitialized { offset: Node<u16> },
}

//noinspection SpellCheckingInspection
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub(crate) cpool_index: Node<u16>,
}

//noinspection DuplicatedCode
impl Object {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.cpool_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerClass {
    pub inner_class_info_index: Node<u16>,
    pub outer_class_info_index: Node<u16>,
    pub inner_name_index: Node<u16>,
    pub inner_class_access_flags: Node<Vec<NestedClassAccessFlag>>,
}

impl InnerClass {
    pub fn inner_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.inner_class_info_index)
    }

    pub fn outer_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.outer_class_info_index)
    }

    pub fn inner_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.inner_name_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LineNumber {
    pub start_pc: Node<u16>,
    pub line_number: Node<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariable {
    pub start_pc: Node<u16>,
    pub length: Node<u16>,
    pub name_index: Node<u16>,
    pub descriptor_index: Node<u16>,
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl LocalVariable {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.descriptor_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableType {
    pub start_pc: Node<u16>,
    pub length: Node<u16>,
    pub name_index: Node<u16>,
    pub signature_index: Node<u16>,
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl LocalVariableType {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }

    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.signature_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: Node<u16>,
    pub num_bootstrap_arguments: Node<u16>,
    pub bootstrap_arguments: Nodes<u16>,
}

impl BootstrapMethod {
    pub fn bootstrap_method_handle<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodHandle> {
        constant_pool.get_method_handle(*self.bootstrap_method_ref)
    }

    pub fn bootstrap_argument<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        self.bootstrap_arguments
            .get(index)
            .and_then(|argument_index| constant_pool.get_constant(**argument_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MethodParameter {
    pub name_index: Node<u16>,
    pub access_flags: Node<Vec<ParameterAccessFlag>>,
}

//noinspection DuplicatedCode
impl MethodParameter {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordComponent {
    pub name_index: Node<u16>,
    pub descriptor_index: Node<u16>,
    pub attributes_count: Node<u16>,
    pub attributes: Nodes<AttributeInfo>,
}

//noinspection DuplicatedCode
impl RecordComponent {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.descriptor_index)
    }
}
