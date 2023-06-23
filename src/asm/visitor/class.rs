use crate::asm::node::access_flag::{FieldAccessFlag, MethodAccessFlag};

pub trait ClassVisitor {
    fn visit_method<F>(&mut self, access_flags: F, name: &str, descriptor: &str)
    where
        F: IntoIterator<Item = MethodAccessFlag>;

    fn visit_field<F>(&mut self, access_flags: F, name: &str, descriptor: &str)
    where
        F: IntoIterator<Item = FieldAccessFlag>;

    fn visit_end(&mut self) {}
}
