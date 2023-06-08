use crate::asm::node::ConstantRearrangeable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Used by
// Attribute::RuntimeVisibleAnnotations
// Attribute::RuntimeInvisibleAnnotations

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for Annotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.type_index, rearrangements);

        for element_value_pair in &mut self.element_value_pairs {
            element_value_pair.rearrange(rearrangements);
        }
    }
}

// Used by:
// Attribute::RuntimeVisibleParameterAnnotations
// Attribute::RuntimeInvisibleParameterAnnotations

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl ConstantRearrangeable for ParameterAnnotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        for annotation in &mut self.annotations {
            annotation.rearrange(rearrangements);
        }
    }
}

// Used by:
// Attribute::RuntimeVisibleTypeAnnotations
// Attribute::RuntimeInvisibleTypeAnnotations

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub target_type: u16,
    pub target_info: TargetType,
    pub type_path: TypePath,
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for TypeAnnotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.type_index, rearrangements);

        for element_value_pair in &mut self.element_value_pairs {
            element_value_pair.rearrange(rearrangements);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetType {
    TypeParameter(TypeParameter),
    SuperType(SuperType),
    TypeParameterBound(TypeParameterBound),
    Empty,
    FormalParameter(FormalParameter),
    Throws(Throws),
    LocalVar(LocalVar),
    Catch(Catch),
    Offset(Offset),
    TypeArgument(TypeArgument),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeParameter {
    pub type_parameter_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SuperType {
    pub super_type_index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeParameterBound {
    pub type_parameter_index: u8,
    pub bound_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FormalParameter {
    pub formal_parameter_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Throws {
    pub throws_type_index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalVar {
    pub table_length: u16,
    pub table: Vec<TableEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Catch {
    pub exception_table_index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Offset {
    pub offset: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeArgument {
    pub offset: u16,
    pub type_argument_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypePath {
    pub path_length: u8,
    pub path: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathSegment {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

// Common inner structures

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

impl ConstantRearrangeable for ElementValuePair {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.element_name_index, rearrangements);
        self.value.rearrange(rearrangements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Value,
}

impl ConstantRearrangeable for ElementValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        self.value.rearrange(rearrangements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    ConstValue(ConstValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(ClassInfo),
    ArrayValue(ArrayValue),
}

impl ConstantRearrangeable for Value {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        match self {
            Value::ConstValue(const_value) => const_value.rearrange(rearrangements),
            Value::EnumConstValue(_) => {}
            Value::ClassInfo(_) => {}
            Value::ArrayValue(_) => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstValue {
    pub const_value_index: u16,
}

impl ConstantRearrangeable for ConstValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.const_value_index, rearrangements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumConstValue {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

impl ConstantRearrangeable for EnumConstValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.type_name_index, rearrangements);
        Self::rearrange_index(&mut self.const_name_index, rearrangements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClassInfo {
    pub class_info_index: u16,
}

impl ConstantRearrangeable for ClassInfo {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.class_info_index, rearrangements);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayValue {
    pub num_values: u16,
    pub values: Vec<ElementValue>,
}

impl ConstantRearrangeable for ArrayValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        for element_value in &mut self.values {
            element_value.rearrange(rearrangements);
        }
    }
}
