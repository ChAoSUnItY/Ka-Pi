use crate::node::attribute::ConstantValue;

/// Visitor used to visiting [Field].
pub trait FieldVisitor {
    /// Visits the [ConstantValue] from [Attribute::ConstantValue].
    fn visit_constant(&mut self, constant_value: &mut ConstantValue);
}
