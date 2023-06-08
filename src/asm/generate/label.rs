use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Label {
    pub(crate) destination_pos: u32,
    current_stack_size: usize,
}

impl Label {
    pub fn new_label(current_stack_size: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            destination_pos: 0,
            current_stack_size,
        }))
    }

    pub(crate) fn set_offset(&mut self, destination_pos: u32) {
        self.destination_pos = destination_pos;
    }
}
