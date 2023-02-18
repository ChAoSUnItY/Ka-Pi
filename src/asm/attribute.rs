use std::rc::Rc;

use crate::asm::label::Label;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Attribute {
    r#type: String,
    content: Vec<u8>,
}

impl Attribute {
    pub(crate) const fn new(r#type: String) -> Self {
        Self {
            r#type,
            content: Vec::new(),
        }
    }

    fn r#type(&self) -> &String {
        &self.r#type
    }

    fn is_unknown() -> bool {
        true
    }

    fn is_code_attribute() -> bool {
        false
    }

    fn labels(&self) -> Vec<Rc<Label>> {
        vec![]
    }
}
