use crate::asm::node::attribute::ConstantValue;

pub trait FieldVisitor {
    fn visit_constant(&mut self, constant_value: &mut ConstantValue);
}
