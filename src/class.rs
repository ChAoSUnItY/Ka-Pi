pub trait ClassVisitor {
    fn inner(&mut self) -> Option<&mut dyn ClassVisitor> {
        None
    }
    
    
}


