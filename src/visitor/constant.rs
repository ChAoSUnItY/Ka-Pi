use crate::node::constant::{
    Class, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef, InvokeDynamic, Long,
    MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, String, Utf8,
};

/// A constant visitor used to visiting [constants](Constant) in [constant pool](ConstantPool).
#[allow(unused_variables)]
pub trait ConstantVisitor {
    /// Visits [Utf8] from [Constant::Utf8].
    fn visit_utf8(&mut self, utf8: &Utf8) {}

    /// Visits [Integer] from [Constant::Integer].
    fn visit_integer(&mut self, integer: &Integer) {}

    /// Visits [Float] from [Constant::Float].
    fn visit_float(&mut self, float: &Float) {}

    /// Visits [Long] from [Constant::Long].
    fn visit_long(&mut self, long: &Long) {}

    /// Visits [Double] from [Constant::Double].
    fn visit_double(&mut self, double: &Double) {}

    /// Visits [Class] from [Constant::Class].
    fn visit_class(&mut self, class: &Class) {}

    /// Visits [String] from [Constant::String].
    fn visit_string(&mut self, string: &String) {}

    /// Visits [FieldRef] from [Constant::FieldRef].
    fn visit_field_ref(&mut self, field_ref: &FieldRef) {}

    /// Visits [MethodRef] from [Constant::MethodRef].
    fn visit_method_ref(&mut self, method_ref: &MethodRef) {}

    /// Visits [InterfaceMethodRef] from [Constant::InterfaceMethodRef].
    fn visit_interface_method_ref(&mut self, interface_method_ref: &InterfaceMethodRef) {}

    /// Visits [NameAndType] from [Constant::NameAndType].
    fn visit_name_and_type(&mut self, name_and_type: &NameAndType) {}

    /// Visits [MethodHandle] from [Constant::MethodHandle].
    fn visit_method_handle(&mut self, method_handle: &MethodHandle) {}

    /// Visits [MethodType] from [Constant::MethodType].
    fn visit_method_type(&mut self, method_type: &MethodType) {}

    /// Visits [Dynamic] from [Constant::Dynamic].
    fn visit_dynamic(&mut self, dynamic: &Dynamic) {}

    /// Visits [InvokeDynamic] from [Constant::InvokeDynamic].
    fn visit_invoke_dynamic(&mut self, invoke_dynamic: &InvokeDynamic) {}

    /// Visits [Module] from [Constant::Module].
    fn visit_module(&mut self, module: &Module) {}

    /// Visits [Package] from [Constant::Package].
    fn visit_package(&mut self, package: &Package) {}
}
