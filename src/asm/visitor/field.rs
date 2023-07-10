use crate::asm::node::attribute::{Attribute, ConstantValue};
use crate::asm::node::field::Field;

/// Visitor used to visiting [Field].
pub trait FieldVisitor {
    /// Visits the [ConstantValue] from [Attribute::ConstantValue].
    fn visit_constant(&mut self, constant_value: &mut ConstantValue);
}
