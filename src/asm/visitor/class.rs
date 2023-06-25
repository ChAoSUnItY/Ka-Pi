use crate::asm::node::access_flag::{FieldAccessFlag, MethodAccessFlag};

pub trait ClassVisitor {
    fn visit_method<F>(&mut self, access_flags: &mut Vec<MethodAccessFlag>, name: &str, descriptor: &str);

    fn visit_field<F>(&mut self, access_flags: &mut Vec<FieldAccessFlag>, name: &mut str, descriptor: &str);

    fn visit_end(&mut self) {}
}
