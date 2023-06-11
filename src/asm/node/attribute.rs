use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::generate::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::generate::symbol::SymbolTable;
use crate::asm::node::access_flag::{ModuleAccessFlag, NestedClassAccessFlag, ParameterAccessFlag};
use crate::asm::node::attribute::annotation::{
    Annotation, ElementValue, ParameterAnnotation, TypeAnnotation,
};
use crate::asm::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::asm::node::constant::{Class, Constant, ConstantPool, MethodHandle, NameAndType, Utf8};
use crate::asm::node::opcode::Instruction;
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

pub mod annotation;
pub mod constant_value;
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_len: u32,
    /// `info` will not be available on generation process, it's an readonly field.
    pub info: Vec<u8>,
    pub attribute: Option<Attribute>,
}

impl ConstantRearrangeable for AttributeInfo {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.attribute_name_index, rearrangements);

        if let Some(attribute) = &mut self.attribute {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

    pub(crate) fn put_u8s(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) {
        let name_index = symbol_table.add_utf8(self.name());
        byte_vec.put_be(name_index);

        match self {
            Attribute::ConstantValue(ConstantValue {
                constant_value_index,
            }) => {
                byte_vec.put_be(2u32);
                byte_vec.put_be(*constant_value_index);
            }
            _ => {}
        }
    }
}

impl ConstantRearrangeable for Attribute {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        match self {
            Attribute::ConstantValue(constant_value) => {
                constant_value.rearrange(rearrangements)?;
            }
            Attribute::Code(code) => {
                code.rearrange(rearrangements)?;
            }
            Attribute::StackMapTable(stack_map_table) => {
                stack_map_table.rearrange(rearrangements)?;
            }
            Attribute::Exceptions(exceptions) => {
                exceptions.rearrange(rearrangements)?;
            }
            Attribute::InnerClasses(inner_classes) => {
                inner_classes.rearrange(rearrangements)?;
            }
            Attribute::EnclosingMethod(enclosing_method) => {
                enclosing_method.rearrange(rearrangements)?;
            }
            Attribute::Synthetic => {}
            Attribute::Signature(signature) => {
                signature.rearrange(rearrangements)?;
            }
            Attribute::SourceFile(source_file) => {
                source_file.rearrange(rearrangements)?;
            }
            Attribute::SourceDebugExtension(..) => {}
            Attribute::LineNumberTable(..) => {}
            Attribute::LocalVariableTable(local_variable_table) => {
                local_variable_table.rearrange(rearrangements)?;
            }
            Attribute::LocalVariableTypeTable(local_variable_type_table) => {
                local_variable_type_table.rearrange(rearrangements)?;
            }
            Attribute::Deprecate => {}
            Attribute::RuntimeVisibleAnnotations(runtime_visible_annotations) => {
                runtime_visible_annotations.rearrange(rearrangements)?;
            }
            Attribute::RuntimeInvisibleAnnotations(runtime_invisible_annotations) => {
                runtime_invisible_annotations.rearrange(rearrangements)?;
            }
            Attribute::RuntimeVisibleParameterAnnotations(
                runtime_visible_parameter_annotations,
            ) => {
                runtime_visible_parameter_annotations.rearrange(rearrangements)?;
            }
            Attribute::RuntimeInvisibleParameterAnnotations(
                runtime_invisible_parameter_annotations,
            ) => {
                runtime_invisible_parameter_annotations.rearrange(rearrangements)?;
            }
            Attribute::RuntimeVisibleTypeAnnotations(runtime_visible_type_annotations) => {
                runtime_visible_type_annotations.rearrange(rearrangements)?;
            }
            Attribute::RuntimeInvisibleTypeAnnotations(runtime_invisible_type_annotations) => {
                runtime_invisible_type_annotations.rearrange(rearrangements)?;
            }
            Attribute::AnnotationDefault(annotation_default) => {
                annotation_default.rearrange(rearrangements)?;
            }
            Attribute::BootstrapMethods(bootstrap_method) => {
                bootstrap_method.rearrange(rearrangements)?;
            }
            Attribute::MethodParameters(method_parameters) => {
                method_parameters.rearrange(rearrangements)?;
            }
            Attribute::Module(module) => module.rearrange(rearrangements)?,
            Attribute::ModulePackages(module_packages) => {
                module_packages.rearrange(rearrangements)?
            }
            Attribute::ModuleMainClass(module_main_class) => {
                module_main_class.rearrange(rearrangements)?
            }
            Attribute::NestHost(nest_host) => {
                nest_host.rearrange(rearrangements)?;
            }
            Attribute::NestMembers(nest_members) => {
                nest_members.rearrange(rearrangements)?;
            }
            Attribute::Record(record) => {
                record.rearrange(rearrangements)?;
            }
            Attribute::PermittedSubclasses(permitted_subclasses) => {
                permitted_subclasses.rearrange(rearrangements)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConstantValue {
    pub constant_value_index: u16,
}

impl ConstantValue {
    pub fn constant<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get(self.constant_value_index)
    }
}

impl ConstantRearrangeable for ConstantValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.constant_value_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ConstantRearrangeable for Code {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for exception in &mut self.exception_table {
            Self::rearrange_index(&mut exception.catch_type, rearrangements);
        }

        for attribute in &mut self.attributes {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StackMapTable {
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrameEntry>,
}

impl ConstantRearrangeable for StackMapTable {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for entry in &mut self.entries {
            entry.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Exceptions {
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u16>,
}

impl ConstantRearrangeable for Exceptions {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for exception_index in &mut self.exception_index_table {
            Self::rearrange_index(exception_index, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InnerClasses {
    pub number_of_classes: u16,
    pub class: Vec<InnerClass>,
}

impl ConstantRearrangeable for InnerClasses {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for class in &mut self.class {
            class.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

impl EnclosingMethod {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.class_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn method_name_and_type<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool NameAndType> {
        if let Some(Constant::NameAndType(name_and_type)) = constant_pool.get(self.method_index) {
            Some(name_and_type)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for EnclosingMethod {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.class_index, rearrangements);
        Self::rearrange_index(&mut self.method_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Signature {
    pub signature_index: u16,
}

//noinspection DuplicatedCode
impl Signature {
    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.signature_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Signature {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.signature_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceFile {
    pub source_file_index: u16,
}

impl SourceFile {
    pub fn source_file<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.source_file_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for SourceFile {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.source_file_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceDebugExtension {
    pub debug_extension: Vec<u8>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LineNumberTable {
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumber>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LocalVariableTable {
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariable>,
}

impl ConstantRearrangeable for LocalVariableTable {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for local_variable in &mut self.local_variable_table {
            local_variable.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableType>,
}

impl ConstantRearrangeable for LocalVariableTypeTable {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for local_variable_type in &mut self.local_variable_type_table {
            local_variable_type.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeVisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeVisibleAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeInvisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeInvisibleAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeVisibleParameterAnnotations {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeVisibleParameterAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.parameter_annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeInvisibleParameterAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.parameter_annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeVisibleTypeAnnotations {
    pub num_annotations: u16,
    pub type_annotations: Vec<TypeAnnotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeVisibleTypeAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.type_annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub num_annotations: u16,
    pub type_annotations: Vec<TypeAnnotation>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for RuntimeInvisibleTypeAnnotations {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.type_annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AnnotationDefault {
    pub default_value: ElementValue,
}

impl ConstantRearrangeable for AnnotationDefault {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        self.default_value.rearrange(rearrangements)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BootstrapMethods {
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

impl ConstantRearrangeable for BootstrapMethods {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for bootstrap_method in &mut self.bootstrap_methods {
            bootstrap_method.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodParameters {
    pub parameters_count: u8,
    pub parameters: Vec<MethodParameter>,
}

impl ConstantRearrangeable for MethodParameters {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for parameter in &mut self.parameters {
            parameter.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Module {
    pub module_name_index: u16,
    pub module_flags: Vec<ModuleAccessFlag>,
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
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.module_name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn module_version<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.module_version_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn uses<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = self
            .uses_index
            .get(index)
            .map(|uses_index| constant_pool.get(*uses_index))
            .flatten()
        {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Module {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.module_name_index, rearrangements);
        Self::rearrange_index(&mut self.module_version_index, rearrangements);

        for requires in &mut self.requires {
            requires.rearrange(rearrangements)?;
        }

        for exports in &mut self.exports {
            exports.rearrange(rearrangements)?;
        }

        for opens in &mut self.opens {
            opens.rearrange(rearrangements)?;
        }

        for uses_index in &mut self.uses_index {
            Self::rearrange_index(uses_index, rearrangements);
        }

        for provides in &mut self.provides {
            provides.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Utf8(utf8)) = self
            .package_index
            .get(index)
            .map(|package_index| constant_pool.get(*package_index))
            .flatten()
        {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for ModulePackages {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for package_index in &mut self.package_index {
            Self::rearrange_index(package_index, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModuleMainClass {
    pub main_class_index: u16,
}

impl ModuleMainClass {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.main_class_index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for ModuleMainClass {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.main_class_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NestHost {
    pub host_class_index: u16,
}

impl NestHost {
    pub fn main_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.host_class_index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for NestHost {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.host_class_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Class(class)) = self
            .classes
            .get(index)
            .map(|class_index| constant_pool.get(*class_index))
            .flatten()
        {
            Some(class)
        } else {
            None
        }
    }
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for NestMembers {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for class in &mut self.classes {
            Self::rearrange_index(class, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Record {
    pub components_count: u16,
    pub components: Vec<RecordComponent>,
}

impl ConstantRearrangeable for Record {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for component in &mut self.components {
            component.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Class(class)) = self
            .classes
            .get(index)
            .map(|class_index| constant_pool.get(*class_index))
            .flatten()
        {
            Some(class)
        } else {
            None
        }
    }
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for PermittedSubclasses {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for class in &mut self.classes {
            Self::rearrange_index(class, rearrangements);
        }

        Ok(())
    }
}

// Inner structs

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Class(class)) = constant_pool.get(self.catch_type) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Exception {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.catch_type, rearrangements);

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl StackMapFrameEntry {
    pub fn len(&self) -> u32 {
        // frame_type: 1
        1 + match self {
            StackMapFrameEntry::Same { .. } => 0,
            StackMapFrameEntry::SameLocal1StackItem {
                frame_type: _,
                stack,
            } => stack.len(),
            StackMapFrameEntry::SameLocal1StackItemExtended {
                frame_type: _,
                offset_delta: _,
                stack,
            } => 2 + stack.len(),
            StackMapFrameEntry::Chop { .. } => 2,
            StackMapFrameEntry::SameExtended { .. } => 4,
            StackMapFrameEntry::Append {
                frame_type,
                offset_delta: _,
                locals: _,
            } => 2 + (*frame_type as u32 - 251),
            StackMapFrameEntry::Full {
                frame_type: _,
                offset_delta: _,
                number_of_locals: _,
                locals,
                number_of_stack_items: _,
                stack,
            } => {
                2 + locals.iter().map(VerificationType::len).sum::<u32>()
                    + stack.iter().map(VerificationType::len).sum::<u32>()
            }
        }
    }
}

impl ConstantRearrangeable for StackMapFrameEntry {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        match self {
            StackMapFrameEntry::SameLocal1StackItem {
                frame_type: _,
                stack,
            } => {
                stack.rearrange(rearrangements)?;
            }
            StackMapFrameEntry::SameLocal1StackItemExtended {
                frame_type: _,
                offset_delta: _,
                stack,
            } => {
                stack.rearrange(rearrangements)?;
            }
            StackMapFrameEntry::Full {
                frame_type: _,
                offset_delta: _,
                number_of_locals: _,
                locals,
                number_of_stack_items: _,
                stack,
            } => {
                for local in locals {
                    local.rearrange(rearrangements)?;
                }

                for stack_entry in stack {
                    stack_entry.rearrange(rearrangements)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

//noinspection SpellCheckingInspection
#[repr(u8)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl VerificationType {
    pub const fn len(&self) -> u32 {
        match self {
            Self::Object { .. } => 2,
            _ => 1,
        }
    }
}

impl ConstantRearrangeable for VerificationType {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        if let Self::Object(object) = self {
            object.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

//noinspection SpellCheckingInspection

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Object {
    pub(crate) cpool_index: u16,
}

//noinspection DuplicatedCode
impl Object {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.cpool_index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Object {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.cpool_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: Vec<NestedClassAccessFlag>,
}

impl InnerClass {
    pub fn inner_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.inner_class_info_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn outer_class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.outer_class_info_index) {
            Some(class)
        } else {
            None
        }
    }

    pub fn inner_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.inner_name_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InnerClass {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.inner_class_info_index, rearrangements);
        Self::rearrange_index(&mut self.outer_class_info_index, rearrangements);
        Self::rearrange_index(&mut self.inner_name_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.descriptor_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for LocalVariable {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);
        Self::rearrange_index(&mut self.descriptor_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn signature<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.signature_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for LocalVariableType {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);
        Self::rearrange_index(&mut self.signature_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::MethodHandle(method_handle)) =
            constant_pool.get(self.bootstrap_method_ref)
        {
            Some(method_handle)
        } else {
            None
        }
    }

    pub fn bootstrap_argument<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        self.bootstrap_arguments
            .get(index)
            .map(|argument_index| constant_pool.get(*argument_index))
            .flatten()
    }
}

impl ConstantRearrangeable for BootstrapMethod {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.bootstrap_method_ref, rearrangements);

        for bootstrap_argument in &mut self.bootstrap_arguments {
            Self::rearrange_index(bootstrap_argument, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: Vec<ParameterAccessFlag>,
}

//noinspection DuplicatedCode
impl MethodParameter {
    pub fn name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.name_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for MethodParameter {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn descriptor<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.descriptor_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for RecordComponent {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);
        Self::rearrange_index(&mut self.descriptor_index, rearrangements);

        for attribute in &mut self.attributes {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}
