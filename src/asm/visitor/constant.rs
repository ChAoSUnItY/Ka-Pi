use crate::asm::node::constant::{
    Class, Constant, ConstantPool, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef,
    InvokeDynamic, Long, MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, String,
    Utf8,
};

/// A constant visitor used to visiting [constants](Constant) in [constant pool](ConstantPool).
#[allow(unused_variables)]
pub trait ConstantVisitor {
    /// Visits [Utf8] from [Constant::Utf8].
    fn visit_utf8(&mut self, utf8: &mut Utf8) {}

    /// Visits [Integer] from [Constant::Integer].
    fn visit_integer(&mut self, integer: &mut Integer) {}

    /// Visits [Float] from [Constant::Float].
    fn visit_float(&mut self, float: &mut Float) {}

    /// Visits [Long] from [Constant::Long].
    fn visit_long(&mut self, long: &mut Long) {}

    /// Visits [Double] from [Constant::Double].
    fn visit_double(&mut self, double: &mut Double) {}

    /// Visits [Class] from [Constant::Class].
    fn visit_class(&mut self, class: &mut Class) {}

    /// Visits [String] from [Constant::String].
    fn visit_string(&mut self, string: &mut String) {}

    /// Visits [FieldRef] from [Constant::FieldRef].
    fn visit_field_ref(&mut self, field_ref: &mut FieldRef) {}

    /// Visits [MethodRef] from [Constant::MethodRef].
    fn visit_method_ref(&mut self, method_ref: &mut MethodRef) {}

    /// Visits [InterfaceMethodRef] from [Constant::InterfaceMethodRef].
    fn visit_interface_method_ref(&mut self, interface_method_ref: &mut InterfaceMethodRef) {}

    /// Visits [NameAndType] from [Constant::NameAndType].
    fn visit_name_and_type(&mut self, name_and_type: &mut NameAndType) {}

    /// Visits [MethodHandle] from [Constant::MethodHandle].
    fn visit_method_handle(&mut self, method_handle: &mut MethodHandle) {}

    /// Visits [MethodType] from [Constant::MethodType].
    fn visit_method_type(&mut self, method_type: &mut MethodType) {}

    /// Visits [Dynamic] from [Constant::Dynamic].
    fn visit_dynamic(&mut self, dynamic: &mut Dynamic) {}

    /// Visits [InvokeDynamic] from [Constant::InvokeDynamic].
    fn visit_invoke_dynamic(&mut self, invoke_dynamic: &mut InvokeDynamic) {}

    /// Visits [Module] from [Constant::Module].
    fn visit_module(&mut self, module: &mut Module) {}

    /// Visits [Package] from [Constant::Package].
    fn visit_package(&mut self, package: &mut Package) {}
}
