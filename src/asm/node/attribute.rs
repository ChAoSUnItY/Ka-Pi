use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::node::access_flag::{NestedClassAccessFlag, ParameterAccessFlag};
use crate::asm::symbol::SymbolTable;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}

impl Eq for ConstantValue {}

impl From<i32> for ConstantValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for ConstantValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for ConstantValue {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f64> for ConstantValue {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<String> for ConstantValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ConstantValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_len: u32,
    pub info: Vec<u8>,
    pub attribute: Option<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Attribute {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<Exception>,
        attributes_length: u16,
        attributes: Vec<AttributeInfo>,
    },
    StackMapTable {
        number_of_entries: u16,
        entries: Vec<StackMapFrameEntry>,
    },
    Exceptions {
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        number_of_classes: u16,
        class: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        source_file_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable {
        local_variable_table_length: u16,
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table_length: u16,
        local_variable_type_table: Vec<LocalVariableType>,
    },
    Deprecate,
    // RuntimeVisibleAnnotations,
    // RuntimeInvisibleAnnotations,
    // RuntimeVisibleParameterAnnotations,
    // RuntimeInvisibleParameterAnnotations,
    // RuntimeVisibleTypeAnnotations,
    // RuntimeInvisibleTypeAnnotations,
    // AnnotationDefault,
    BootstrapMethods {
        num_bootstrap_methods: u16,
        bootstrap_methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters_count: u16,
        method_parameters: Vec<MethodParameter>,
    },
    // Module,
    // ModulePackages,
    // ModuleMainClass,
    NestHost {
        host_class_index: u16,
    },
    NestMembers {
        number_of_classes: u16,
        classes: Vec<u16>,
    }, // Record,
       // PermittedSubclasses,
}

impl Attribute {
    pub const fn name(&self) -> &'static str {
        match self {
            Attribute::ConstantValue { .. } => CONSTANT_VALUE,
            Attribute::Code { .. } => CODE,
            Attribute::StackMapTable { .. } => STACK_MAP_TABLE,
            Attribute::Exceptions { .. } => EXCEPTIONS,
            Attribute::InnerClasses { .. } => INNER_CLASSES,
            Attribute::EnclosingMethod { .. } => ENCLOSING_METHOD,
            Attribute::Synthetic => SYNTHETIC,
            Attribute::Signature { .. } => SIGNATURE,
            Attribute::SourceFile { .. } => SOURCE_FILE,
            Attribute::SourceDebugExtension { .. } => SOURCE_DEBUG_EXTENSION,
            Attribute::LineNumberTable { .. } => LINE_NUMBER_TABLE,
            Attribute::LocalVariableTable { .. } => LOCAL_VARIABLE_TABLE,
            Attribute::LocalVariableTypeTable { .. } => LOCAL_VARIABLE_TYPE_TABLE,
            Attribute::Deprecate => DEPRECATED,
            // Attribute::RuntimeVisibleAnnotations,
            // Attribute::RuntimeInvisibleAnnotations,
            // Attribute::RuntimeVisibleParameterAnnotations,
            // Attribute::RuntimeInvisibleParameterAnnotations,
            // Attribute::RuntimeVisibleTypeAnnotations,
            // Attribute::RuntimeInvisibleTypeAnnotations,
            // Attribute::AnnotationDefault,
            Attribute::BootstrapMethods { .. } => BOOTSTRAP_METHODS,
            Attribute::MethodParameters { .. } => METHOD_PARAMETERS,
            // Attribute::Module,
            // Attribute::ModulePackages,
            // Attribute::ModuleMainClass,
            Attribute::NestHost { .. } => NEST_HOST,
            Attribute::NestMembers { .. } => NEST_MEMBERS,
            // Attribute::Record,
            // Attribute::PermittedSubclasses,
        }
    }

    pub fn attribute_len(&self) -> u32 {
        match self {
            Attribute::ConstantValue { .. } => 2,
            Attribute::Code {
                max_stack: _,
                max_locals: _,
                code_length,
                code: _,
                exception_table_length,
                exception_table: _,
                attributes_length: _,
                attributes,
            } => {
                // max_stack: 2
                // max_locals: 2
                // code_length: 4
                // code: code_length
                // exception_table_length: 2
                // exception_table: [Exception] (8) * exception_table_length
                //     start_pc: 2
                //     end_pc: 2
                //     handler_pc: 2
                //     catch_type: 2
                //     `total`: 8
                // attributes_length: 2
                // attributes: [Attribute] * attributes_length
                // `total`: 12 + code_length + 8 * exception_table_length + [Attribute] * attribute_length
                12 + *code_length as u32
                    + 8 * *exception_table_length as u32
                    + attributes.iter().map(|info| info.attribute_len).sum::<u32>()
            }
            Attribute::StackMapTable {
                number_of_entries: _,
                entries,
            } => 2 + entries.iter().map(StackMapFrameEntry::len).sum::<u32>(),
            Attribute::Exceptions {
                number_of_exceptions,
                exception_index_table: _,
            } => 2 + 2 * *number_of_exceptions as u32,
            Attribute::InnerClasses {
                number_of_classes,
                class: _,
            } => 8 * *number_of_classes as u32,
            Attribute::EnclosingMethod { .. } => 4,
            Attribute::Synthetic => 0,
            Attribute::Signature { .. } => 2,
            Attribute::SourceFile { .. } => 2,
            Attribute::SourceDebugExtension { debug_extension } => debug_extension.len() as u32,
            Attribute::LineNumberTable {
                line_number_table_length,
                line_number_table: _,
            } => 4 * *line_number_table_length as u32,
            Attribute::LocalVariableTable {
                local_variable_table_length,
                local_variable_table: _,
            } => 10 * *local_variable_table_length as u32,
            Attribute::LocalVariableTypeTable {
                local_variable_type_table_length,
                local_variable_type_table: _,
            } => 10 * *local_variable_type_table_length as u32,
            Attribute::Deprecate => 0,
            // Attribute::RuntimeVisibleAnnotations,
            // Attribute::RuntimeInvisibleAnnotations,
            // Attribute::RuntimeVisibleParameterAnnotations,
            // Attribute::RuntimeInvisibleParameterAnnotations,
            // Attribute::RuntimeVisibleTypeAnnotations,
            // Attribute::RuntimeInvisibleTypeAnnotations,
            // Attribute::AnnotationDefault,
            Attribute::BootstrapMethods {
                num_bootstrap_methods: _,
                bootstrap_methods,
            } => {
                2 + bootstrap_methods
                    .iter()
                    .map(BootstrapMethod::len)
                    .sum::<u32>()
            },
            Attribute::MethodParameters { parameters_count, method_parameters: _ } => {
                2 + 4 * *parameters_count as u32
            }
            // Attribute::Module,
            // Attribute::ModulePackages,
            // Attribute::ModuleMainClass,
            Attribute::NestHost { .. } => 2,
            Attribute::NestMembers { number_of_classes, classes: _ } => {
                2 + 2 * *number_of_classes as u32
            }
            // Attribute::Record,
            // Attribute::PermittedSubclasses,
        }
    }

    fn rearrange_index(original: &mut u16, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(original) {
            *original = *target_index;
        }
    }

    pub(crate) fn rearrange_indices(&mut self, rearrangements: &HashMap<u16, u16>) {
        match self {
            Attribute::ConstantValue {
                constant_value_index,
            } => {
                Self::rearrange_index(constant_value_index, &rearrangements);
            }
            Attribute::Code {
                max_stack: _,
                max_locals: _,
                code_length: _,
                code: _,
                exception_table_length: _,
                exception_table,
                attributes_length: _,
                attributes,
            } => {
                for exception in exception_table {
                    Self::rearrange_index(&mut exception.catch_type, rearrangements);
                }

                for attribute in attributes {
                    if let Some(attribute) = &mut attribute.attribute {
                        attribute.rearrange_indices(rearrangements);
                    }
                }
            }
            Attribute::StackMapTable {
                number_of_entries: _,
                entries,
            } => {
                for entry in entries {
                    entry.rearrange_index(rearrangements);
                }
            }
            Attribute::Exceptions {
                number_of_exceptions: _,
                exception_index_table,
            } => {
                for exception_index in exception_index_table {
                    Self::rearrange_index(exception_index, rearrangements);
                }
            }
            Attribute::InnerClasses {
                number_of_classes: _,
                class,
            } => {
                for class in class {
                    class.rearrange_index(rearrangements);
                }
            }
            Attribute::EnclosingMethod {
                class_index,
                method_index,
            } => {
                Self::rearrange_index(class_index, rearrangements);
                Self::rearrange_index(method_index, rearrangements);
            }
            Attribute::Synthetic => {}
            Attribute::Signature { signature_index } => {
                Self::rearrange_index(signature_index, rearrangements);
            }
            Attribute::SourceFile { source_file_index } => {
                Self::rearrange_index(source_file_index, rearrangements);
            }
            Attribute::SourceDebugExtension { .. } => {}
            Attribute::LineNumberTable { .. } => {}
            Attribute::LocalVariableTable {
                local_variable_table_length: _,
                local_variable_table,
            } => {
                for local_variable in local_variable_table {
                    local_variable.rearrange_index(rearrangements);
                }
            }
            Attribute::LocalVariableTypeTable {
                local_variable_type_table_length: _,
                local_variable_type_table,
            } => {
                for local_variable_type in local_variable_type_table {
                    local_variable_type.rearrange_index(rearrangements);
                }
            }
            Attribute::Deprecate => {}
            Attribute::BootstrapMethods {
                num_bootstrap_methods: _,
                bootstrap_methods,
            } => {
                for bootstrap_method in bootstrap_methods {
                    bootstrap_method.rearrange_index(rearrangements);
                }
            }
            Attribute::MethodParameters {
                parameters_count: _,
                method_parameters,
            } => {
                for method_parameter in method_parameters {
                    method_parameter.rearrange_index(rearrangements);
                }
            }
            Attribute::NestHost { host_class_index } => {
                Self::rearrange_index(host_class_index, rearrangements);
            }
            Attribute::NestMembers {
                number_of_classes: _,
                classes,
            } => {
                for class in classes {
                    Self::rearrange_index(class, rearrangements);
                }
            }
        }
    }

    pub(crate) fn bytecode(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) {
        let name_index = symbol_table.add_utf8(self.name());
        byte_vec.put_be(name_index);

        match self {
            Attribute::ConstantValue {
                constant_value_index,
            } => {
                byte_vec.put_be(2u32);
                byte_vec.put_be(*constant_value_index);
            }
            Attribute::Code { .. } => {}
            Attribute::StackMapTable { .. } => {}
            Attribute::Exceptions { .. } => {}
            Attribute::InnerClasses { .. } => {}
            Attribute::EnclosingMethod { .. } => {}
            Attribute::Synthetic => {}
            Attribute::Signature { .. } => {}
            Attribute::SourceFile { .. } => {}
            Attribute::SourceDebugExtension { .. } => {}
            Attribute::LineNumberTable { .. } => {}
            Attribute::LocalVariableTable { .. } => {}
            Attribute::LocalVariableTypeTable { .. } => {}
            Attribute::Deprecate => {}
            Attribute::BootstrapMethods { .. } => {}
            Attribute::MethodParameters { .. } => {}
            Attribute::NestHost { .. } => {}
            Attribute::NestMembers { .. } => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl Exception {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.catch_type) {
            self.catch_type = *target_index;
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        match self {
            StackMapFrameEntry::SameLocal1StackItem {
                frame_type: _,
                stack,
            } => {
                stack.rearrange_index(rearrangements);
            }
            StackMapFrameEntry::SameLocal1StackItemExtended {
                frame_type: _,
                offset_delta: _,
                stack,
            } => {
                stack.rearrange_index(rearrangements);
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
                    local.rearrange_index(rearrangements);
                }

                for stack_entry in stack {
                    stack_entry.rearrange_index(rearrangements);
                }
            }
            _ => {}
        }
    }
}

// noinspection ALL
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

impl VerificationType {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Self::Object { cpool_index } = self {
            if let Some(target_index) = rearrangements.get(cpool_index) {
                *cpool_index = *target_index;
            }
        }
    }
}

impl VerificationType {
    pub const fn len(&self) -> u32 {
        match self {
            Self::Object { .. } => 2,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: Vec<NestedClassAccessFlag>,
}

impl InnerClass {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.inner_class_info_index) {
            self.inner_class_info_index = *target_index;
        }
        if let Some(target_index) = rearrangements.get(&self.outer_class_info_index) {
            self.outer_class_info_index = *target_index;
        }
        if let Some(target_index) = rearrangements.get(&self.inner_name_index) {
            self.inner_name_index = *target_index;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariable {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.name_index) {
            self.name_index = *target_index;
        }
        if let Some(target_index) = rearrangements.get(&self.descriptor_index) {
            self.descriptor_index = *target_index;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl LocalVariableType {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.name_index) {
            self.name_index = *target_index;
        }
        if let Some(target_index) = rearrangements.get(&self.signature_index) {
            self.signature_index = *target_index;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    pub fn new(bootstrap_method_ref: u16, boostrap_arguments_indices: Vec<u16>) -> Self {
        Self {
            bootstrap_method_ref,
            num_bootstrap_arguments: boostrap_arguments_indices.len() as u16,
            bootstrap_arguments: boostrap_arguments_indices,
        }
    }

    pub fn len(&self) -> u32 {
        4 + self.bootstrap_arguments.len() as u32 * 2
    }

    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.bootstrap_method_ref) {
            self.bootstrap_method_ref = *target_index;
        }

        for bootstrap_argument in &mut self.bootstrap_arguments {
            if let Some(target_index) = rearrangements.get(bootstrap_argument) {
                *bootstrap_argument = *target_index;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MethodParameter {
    name_index: u16,
    access_flags: Vec<ParameterAccessFlag>,
}

impl MethodParameter {
    fn rearrange_index(&mut self, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(&self.name_index) {
            self.name_index = *target_index;
        }
    }
}
