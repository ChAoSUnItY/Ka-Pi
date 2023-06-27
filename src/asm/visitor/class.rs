use crate::asm::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use crate::asm::node::class::JavaVersion;
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::node::field::Field;
use crate::asm::node::method::Method;
use crate::asm::visitor::constant::ConstantVisitor;
use crate::asm::visitor::field::FieldVisitor;
use crate::asm::visitor::method::MethodVisitor;

#[allow(unused_variables)]
pub trait ClassVisitor {
    type CPV: ConstantVisitor;
    type TCCV: ConstantVisitor;
    type SCCV: ConstantVisitor;
    type ICV: ConstantVisitor;
    type MV: MethodVisitor;
    type FV: FieldVisitor;

    fn visit_version(&mut self, version: &mut JavaVersion) {}

    fn visit_constant_pool(&mut self, constant_pool: &ConstantPool) {}

    fn visit_constant(&mut self, index: &u16, constant: &Constant) -> Self::CPV;

    fn visit_access_flags(&mut self, access_flags: &mut Vec<ClassAccessFlag>) {}

    fn visit_this_class(&mut self) -> Self::TCCV;

    fn visit_super_class(&mut self) -> Self::SCCV;

    fn visit_interfaces(&mut self, interface_indices: &[u16]) {}

    fn visit_interface(&mut self) -> Self::ICV;

    fn visit_fields(&mut self, fields: &mut Vec<Field>) {}

    fn visit_field(
        &mut self,
        access_flags: &mut Vec<FieldAccessFlag>,
        name: &String,
        descriptor: &String,
    ) -> Self::FV;

    fn visit_methods(&mut self, methods: &mut Vec<Method>) {}

    fn visit_method(
        &mut self,
        access_flags: &mut Vec<MethodAccessFlag>,
        name: &String,
        descriptor: &String,
    ) -> Self::MV;
}
