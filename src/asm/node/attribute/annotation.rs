use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::constant::{Constant, ConstantPool, Utf8};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

// Used by
// Attribute::RuntimeVisibleAnnotations
// Attribute::RuntimeInvisibleAnnotations
// Attribute::RuntimeVisibleParameterAnnotations
// Attribute::RuntimeInvisibleParameterAnnotations

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

//noinspection DuplicatedCode
impl Annotation {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.type_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for Annotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.type_index, rearrangements);

        for element_value_pair in &mut self.element_value_pairs {
            element_value_pair.rearrange(rearrangements)?;
        }

        Ok(())
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

//noinspection DuplicatedCode
impl ConstantRearrangeable for ParameterAnnotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for annotation in &mut self.annotations {
            annotation.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

// Used by:
// Attribute::RuntimeVisibleTypeAnnotations
// Attribute::RuntimeInvisibleTypeAnnotations

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub target_type: u16,
    pub target_info: TargetInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

//noinspection DuplicatedCode
impl TypeAnnotation {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.type_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for TypeAnnotation {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.type_index, rearrangements);

        for element_value_pair in &mut self.element_value_pairs {
            element_value_pair.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetInfo {
    TypeParameter {
        type_parameter_index: u8,
    },
    SuperType {
        super_type_index: u16,
    },
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: u8,
    },
    Throws {
        throws_type_index: u16,
    },
    LocalVar {
        table_length: u16,
        table: Vec<TableEntry>,
    },
    Catch {
        exception_table_index: u16,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
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
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.element_name_index, rearrangements);
        self.value.rearrange(rearrangements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Value,
}

impl ConstantRearrangeable for ElementValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        self.value.rearrange(rearrangements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    ConstValue(ConstValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(ClassInfo),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

impl ConstantRearrangeable for Value {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        match self {
            Value::ConstValue(const_value) => const_value.rearrange(rearrangements),
            Value::EnumConstValue(enum_const_value) => enum_const_value.rearrange(rearrangements),
            Value::ClassInfo(class_info) => class_info.rearrange(rearrangements),
            Value::AnnotationValue(annotation) => annotation.rearrange(rearrangements),
            Value::ArrayValue(array_value) => array_value.rearrange(rearrangements),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstValue {
    pub const_value_index: u16,
}

impl ConstValue {
    pub fn const_value<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get(self.const_value_index)
    }
}

impl ConstantRearrangeable for ConstValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.const_value_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumConstValue {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

impl EnumConstValue {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.type_name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn const_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.const_name_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for EnumConstValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.type_name_index, rearrangements);
        Self::rearrange_index(&mut self.const_name_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClassInfo {
    pub class_info_index: u16,
}

impl ClassInfo {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.class_info_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for ClassInfo {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.class_info_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayValue {
    pub num_values: u16,
    pub values: Vec<ElementValue>,
}

impl ConstantRearrangeable for ArrayValue {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        for element_value in &mut self.values {
            element_value.rearrange(rearrangements)?;
        }

        Ok(())
    }
}
