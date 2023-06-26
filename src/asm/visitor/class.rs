use crate::asm::node::access_flag::{FieldAccessFlag, MethodAccessFlag};
use crate::asm::node::class::JavaVersion;
use crate::asm::node::constant::ConstantPool;
use crate::asm::visitor::constant::ConstantVisitor;
use crate::asm::visitor::field::FieldVisitor;
use crate::asm::visitor::method::MethodVisitor;

pub trait ClassVisitor {
    type CV: ConstantVisitor;
    type MV: MethodVisitor;
    type FV: FieldVisitor;

    fn visit_version(&mut self, version: &mut JavaVersion);

    fn visit_constant_pool(&mut self, constant_pool: &mut ConstantPool) -> Self::CV;

    fn visit_field(
        &mut self,
        access_flags: &mut Vec<FieldAccessFlag>,
        name: &mut String,
        descriptor: &mut String,
    ) -> Self::FV;

    fn visit_method(
        &mut self,
        access_flags: &mut Vec<MethodAccessFlag>,
        name: &mut String,
        descriptor: &mut String,
    ) -> Self::MV;
}
