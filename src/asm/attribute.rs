use crate::asm::label::Label;

pub(crate) trait Attribute<'a> {
    fn r#type(&self) -> &String;
    fn is_unknown() -> bool {
        true
    }
    fn is_code_attribute() -> bool {
        false
    }
    fn labels(&self) -> Vec<Box<dyn Label + 'a>> {
        vec![]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct AttributeImpl {
    r#type: String,
    content: Vec<u8>,
}

impl AttributeImpl {
    pub(crate) const fn new(r#type: String) -> Self {
        Self {
            r#type,
            content: Vec::new(),
        }
    }
}

impl<'a> Attribute<'a> for AttributeImpl {
    fn r#type(&self) -> &String {
        &self.r#type
    }
}
