use crate::asm::node::constant::{
    Class, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef, InvokeDynamic, Long,
    MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, String, Utf8,
};

#[allow(unused_variables)]
pub trait ConstantVisitor {
    fn visit_utf8(&mut self, utf8: &mut Utf8) {}

    fn visit_integer(&mut self, integer: &mut Integer) {}

    fn visit_float(&mut self, float: &mut Float) {}

    fn visit_long(&mut self, long: &mut Long) {}

    fn visit_double(&mut self, double: &mut Double) {}

    fn visit_class(&mut self, class: &mut Class) {}

    fn visit_string(&mut self, string: &mut String) {}

    fn visit_field_ref(&mut self, field_ref: &mut FieldRef) {}

    fn visit_method_ref(&mut self, method_ref: &mut MethodRef) {}

    fn visit_interface_method_ref(&mut self, interface_method_ref: &mut InterfaceMethodRef) {}

    fn visit_name_and_type(&mut self, name_and_type: &mut NameAndType) {}

    fn visit_method_handle(&mut self, method_handle: &mut MethodHandle) {}

    fn visit_method_type(&mut self, method_type: &mut MethodType) {}

    fn visit_dynamic(&mut self, dynamic: &mut Dynamic) {}

    fn visit_invoke_dynamic(&mut self, invoke_dynamic: &mut InvokeDynamic) {}

    fn visit_module(&mut self, module: &mut Module) {}

    fn visit_package(&mut self, package: &mut Package) {}
}
