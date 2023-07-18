use crate::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use crate::node::class::JavaVersion;
use crate::node::constant::{Constant, ConstantPool};
use crate::node::field::Field;
use crate::node::method::Method;
use crate::visitor::constant::ConstantVisitor;
use crate::visitor::field::FieldVisitor;
use crate::visitor::method::MethodVisitor;

// TODO(ChAoSUnItY): Allow mutation on constant pool.
// TODO(ChAoSUnItY): Implement inner class visitor.
/// A visitor used to visiting [class](Class).
#[allow(unused_variables)]
pub trait ClassVisitor {
    /// Constant visitor for [constants](Constant) visiting from [constant pool](ConstantPool).
    type CPV: ConstantVisitor;
    /// Constant visitor for this class constant visiting.
    type TCCV: ConstantVisitor;
    /// Constant visitor for super class constant visiting.
    type SCCV: ConstantVisitor;
    /// Constant visitor for interface constant visiting.
    type ICV: ConstantVisitor;
    /// Method visitor for [method](Method) visiting.
    type MV: MethodVisitor;
    /// Field visitor for [field](Field) visiting.
    type FV: FieldVisitor;

    /// Visits [major and minor versions](JavaVersion).
    fn visit_version(&mut self, version: &JavaVersion) {}

    /// Visits [ConstantPool].
    fn visit_constant_pool(&mut self, constant_pool: &ConstantPool) {}

    /// Visits [constants](Constant) in [constant pool](ConstantPool).
    fn visit_constant(&mut self, index: &u16, constant: &Constant) -> Self::CPV;

    /// Visits [ClassAccessFlag]s.
    fn visit_access_flags(&mut self, access_flags: &[ClassAccessFlag]) {}

    /// Visits this class [constant](Constant), it is guaranteed only function
    /// [visit_utf8](ConstantVisitor::visit_utf8) will be invoked on [Self::TCCV].
    fn visit_this_class(&mut self) -> Self::TCCV;

    /// Visits super class [constant](Constant), it is guaranteed only function
    /// [visit_utf8](ConstantVisitor::visit_utf8) will be invoked on [Self::SCCV].
    fn visit_super_class(&mut self) -> Self::SCCV;

    /// Visits interface constant indices.
    fn visit_interfaces(&mut self, interface_indices: &[u16]) {}

    /// Visits [interface constant](Constant), it is guaranteed only function
    /// [visit_utf8](ConstantVisitor::visit_utf8) will be invoked on [Self::ICV].
    fn visit_interface(&mut self) -> Self::ICV;

    /// Visits [fields](Field).
    fn visit_fields(&mut self, fields: &[Field]) {}

    /// Visits [Field].
    fn visit_field(
        &mut self,
        access_flags: &[FieldAccessFlag],
        name: &str,
        descriptor: &str,
    ) -> Self::FV;

    /// Visits [methods](Method).
    fn visit_methods(&mut self, methods: &[Method]) {}

    /// Visits [Method].
    fn visit_method(
        &mut self,
        access_flags: &[MethodAccessFlag],
        name: &str,
        descriptor: &str,
    ) -> Self::MV;
}
