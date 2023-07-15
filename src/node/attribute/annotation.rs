use serde::{Deserialize, Serialize};

use crate::node::constant::{Constant, ConstantPool, Utf8};
use crate::node::{Node, Nodes};

// Used by
// Attribute::RuntimeVisibleAnnotations
// Attribute::RuntimeInvisibleAnnotations
// Attribute::RuntimeVisibleParameterAnnotations
// Attribute::RuntimeInvisibleParameterAnnotations

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub type_index: Node<u16>,
    pub num_element_value_pairs: Node<u16>,
    pub element_value_pairs: Nodes<ElementValuePair>,
}

//noinspection DuplicatedCode
impl Annotation {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.type_index)
    }
}

// Used by:
// Attribute::RuntimeVisibleParameterAnnotations
// Attribute::RuntimeInvisibleParameterAnnotations

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterAnnotation {
    pub num_annotations: Node<u16>,
    pub annotations: Nodes<Annotation>,
}

// Used by:
// Attribute::RuntimeVisibleTypeAnnotations
// Attribute::RuntimeInvisibleTypeAnnotations

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub target_type: Node<u16>,
    pub target_info: Node<TargetInfo>,
    pub type_path: Node<TypePath>,
    pub type_index: Node<u16>,
    pub num_element_value_pairs: Node<u16>,
    pub element_value_pairs: Nodes<ElementValuePair>,
}

//noinspection DuplicatedCode
impl TypeAnnotation {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.type_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TargetInfo {
    TypeParameter {
        type_parameter_index: Node<u8>,
    },
    SuperType {
        super_type_index: Node<u16>,
    },
    TypeParameterBound {
        type_parameter_index: Node<u8>,
        bound_index: Node<u8>,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: Node<u8>,
    },
    Throws {
        throws_type_index: Node<u16>,
    },
    LocalVar {
        table_length: Node<u16>,
        table: Nodes<TableEntry>,
    },
    Catch {
        exception_table_index: Node<u16>,
    },
    Offset {
        offset: Node<u16>,
    },
    TypeArgument {
        offset: Node<u16>,
        type_argument_index: Node<u8>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableEntry {
    pub start_pc: Node<u16>,
    pub length: Node<u16>,
    pub index: Node<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypePath {
    pub path_length: Node<u8>,
    pub path: Nodes<PathSegment>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PathSegment {
    pub type_path_kind: Node<u8>,
    pub type_argument_index: Node<u8>,
}

// Common inner structures

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ElementValuePair {
    pub element_name_index: Node<u16>,
    pub value: Node<ElementValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ElementValue {
    pub tag: Node<u8>,
    pub value: Node<Value>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Value {
    ConstValue(ConstValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(ClassInfo),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConstValue {
    pub const_value_index: Node<u16>,
}

impl ConstValue {
    pub fn const_value<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(*self.const_value_index).map(|node| node)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnumConstValue {
    pub type_name_index: Node<u16>,
    pub const_name_index: Node<u16>,
}

impl EnumConstValue {
    pub fn type_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.type_name_index)
    }

    pub fn const_name<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.const_name_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassInfo {
    pub class_info_index: Node<u16>,
}

impl ClassInfo {
    pub fn class<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.class_info_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArrayValue {
    pub num_values: Node<u16>,
    pub values: Nodes<ElementValue>,
}
