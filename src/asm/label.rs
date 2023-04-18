use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Label(pub(crate) u32);

impl Default for Label {
    fn default() -> Self {
        Self(0)
    }
}

impl Label {
    pub fn new_label() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }
}
