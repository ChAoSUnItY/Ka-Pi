use crate::asm::node::attribute::ConstantValue;
use crate::error::KapiResult;

pub trait FieldVisitor {
    fn visit_constant(&mut self, constant_value: ConstantValue) -> KapiResult<()>;
}
