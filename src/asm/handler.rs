use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::error::{KapiError, KapiResult};

use super::label::Label;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Handler {
    start_pc: Option<Label>,
    end_pc: Option<Label>,
    handler_pc: Option<Label>,
    catch_type: i32,
    catch_type_descriptor: String,
}

impl Handler {
    pub(crate) const fn new(
        start_pc: Option<Label>,
        end_pc: Option<Label>,
        handler_pc: Option<Label>,
        catch_type: i32,
        catch_type_descriptor: String,
    ) -> Self {
        Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
            catch_type_descriptor,
        }
    }

    pub(crate) fn from_handler(
        handler: &Self,
        start_pc: Option<Label>,
        end_pc: Option<Label>,
    ) -> Self {
        Self::new(
            start_pc,
            end_pc,
            handler.handler_pc.clone(),
            handler.catch_type,
            handler.catch_type_descriptor.clone(),
        )
    }

    pub(crate) fn remove_range(
        handlers: &mut Vec<Self>,
        start: &Option<Label>,
        end: &Option<Label>,
    ) -> KapiResult<()> {
        for i in (1..handlers.len()).rev() {
            let handler = handlers[i].clone();
            let handler_start = handler
                .start_pc
                .as_ref()
                .ok_or_else(|| KapiError::StateError("Handler start pc must not be None"))?
                .bytecode_offset;
            let handler_end = handler
                .end_pc
                .as_ref()
                .ok_or_else(|| KapiError::StateError("Handler end pc must not be None"))?
                .bytecode_offset;
            let range_start = start
                .as_ref()
                .ok_or_else(|| KapiError::StateError("Label's start pc must not be None"))?
                .bytecode_offset;
            let range_end = if let Some(end_label) = end.as_ref() {
                end_label.bytecode_offset
            } else {
                i32::MAX
            };

            if range_start >= handler_end || range_end <= handler_start {
                break;
            }

            if range_start <= handler_start {
                if range_end >= handler_end {
                    handlers.swap(i - 1, i + 1);
                } else {
                    handlers[i] =
                        Self::from_handler(&handler, end.clone(), handler.end_pc.clone());
                }
            } else if range_end >= handler_end {
                handlers[i] =
                    Self::from_handler(&handler, handler.start_pc.clone(), start.clone());
            } else {
                handlers.insert(
                    i + 1,
                    Self::from_handler(&handler, end.clone(), handler.end_pc.clone()),
                );
                handlers[i] =
                    Self::from_handler(&handler, handler.start_pc.clone(), start.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::asm::handler::Handler;
    use crate::asm::label::Label;

    #[test]
    fn test_new_handler() {
        let handler = Handler::new(
            Label::new().into(),
            Label::new().into(),
            Label::new().into(),
            123,
            String::from("123"),
        );
        
        assert_eq!(Label::new(), handler.start_pc.unwrap());
        assert_eq!(Label::new(), handler.end_pc.unwrap());
        assert_eq!(Label::new(), handler.handler_pc.unwrap());
        assert_eq!(123, handler.catch_type);
        assert_eq!("123", handler.catch_type_descriptor);
    }
    
    #[test]
    fn test_copy_handler() {
        let handler = Handler::new(
            Label::new().into(),
            Label::new().into(),
            Label::new().into(),
            123,
            String::from("123"),
        );
        let copied_handler = Handler::from_handler(&handler, Label::new().into(), Label::new().into());
        
        assert_eq!(handler, copied_handler);
    }
}