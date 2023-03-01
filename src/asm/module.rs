pub trait ModuleVisitor {
    fn visit_main_class(&mut self, main_class: String) {}
    fn visit_package(&mut self, package: String) {}
    fn visit_required(&mut self, module: String, access: u32, version: String) {}
    fn visit_export(&mut self, package: String, access: u32, modules: &[String]) {}
    fn visit_open(&mut self, package: String, access: u32, modules: &[String]) {}
    fn visit_use(&mut self, service: String) {}
    fn visit_provide(&mut self, service: String, provides: &[String]) {}
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}
