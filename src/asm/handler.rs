use std::collections::VecDeque;
use std::ops::Deref;

use jni::errors::StartJvmError;

use crate::error::{KapiError, KapiResult};

use super::label::Label;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Handler {
    start_pc: Option<Label>,
    end_pc: Option<Label>,
    handler_pc: Option<Label>,
    catch_type: i32,
    catch_type_descriptor: String,
    next_handler: Option<Box<Handler>>,
}

impl Handler {
    pub(crate) fn new(
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
            next_handler: None,
        }
    }

    pub(crate) fn start_pc(&self) -> Option<i32> {
        self.start_pc.as_ref().map(|label| label.bytecode_offset)
    }

    pub(crate) fn end_pc(&self) -> Option<i32> {
        self.end_pc.as_ref().map(|label| label.bytecode_offset)
    }

    pub(crate) fn from_handler(
        handler: &Self,
        start_pc: &Option<Label>,
        end_pc: &Option<Label>,
    ) -> Self {
        Self::new(
            start_pc.clone(),
            end_pc.clone(),
            handler.handler_pc.clone(),
            handler.catch_type,
            handler.catch_type_descriptor.clone(),
        )
    }

    pub(crate) fn remove_range(
        &mut self,
        start: &Option<Label>,
        end: &Option<Label>,
    ) -> KapiResult<Option<Handler>> {
        if let Some(next_handler) = &mut self.next_handler {
            self.next_handler = next_handler.remove_range(start, end)?.map(Box::new);
        }

        let range_start = start
            .as_ref()
            .ok_or(KapiError::StateError("Start label cannot be None"))?
            .bytecode_offset;
        let range_end = end
            .as_ref()
            .map_or_else(|| i32::MAX, |label| label.bytecode_offset);
        let handler_start = self
            .start_pc()
            .ok_or(KapiError::StateError("Handler start label cannot be None"))?;
        let handler_end = self
            .end_pc()
            .ok_or(KapiError::StateError("Handler end label cannot be None"))?;

        if range_start >= handler_end || range_end <= handler_start {
            Ok(Some(self.clone()))
        } else if range_start <= handler_start {
            if range_end >= handler_end {
                Ok(self.next_handler.clone().map(|handler| *handler))
            } else {
                Ok(Some(Self::from_handler(&self, &end, &self.end_pc)))
            }
        } else if range_end >= handler_end {
            Ok(Some(Self::from_handler(&self, &self.start_pc, start)))
        } else {
            self.next_handler = Some(Box::new(Self::from_handler(&self, &end, &self.end_pc)));
            Ok(Some(Self::from_handler(&self, &self.start_pc, start)))
        }
    }
}

// #[cfg(test)]
// mod test {
//     use std::collections::VecDeque;
//     use rstest::rstest;
//
//     use crate::asm::handler::Handler;
//     use crate::asm::label::Label;
//
//     fn new_handler(start_pc: i32, end_pc: i32) -> Handler {
//         Handler::new(
//             new_label(start_pc),
//             new_label(end_pc),
//             new_label(0),
//             0,
//             String::from(""),
//         )
//     }
//
//     fn new_label(pc: i32) -> Label {
//         let mut label = Label::default();
//         label.bytecode_offset = pc;
//         label
//     }
//
//     #[test]
//     fn test_new_handler() {
//         let handler = Handler::new(
//             Label::default(),
//             Label::default(),
//             Label::default(),
//             123,
//             String::from("123"),
//         );
//
//         assert_eq!(Label::default(), handler.start_pc.unwrap());
//         assert_eq!(Label::default(), handler.end_pc.unwrap());
//         assert_eq!(Label::default(), handler.handler_pc.unwrap());
//         assert_eq!(123, handler.catch_type);
//         assert_eq!("123", handler.catch_type_descriptor);
//     }
//
//     #[test]
//     fn test_copy_handler() {
//         let handler = Handler::new(
//             Label::default(),
//             Label::default(),
//             Label::default(),
//             123,
//             String::from("123"),
//         );
//         let copied_handler = Handler::from_handler(&handler, Label::default(), Label::default());
//
//         assert_eq!(handler, copied_handler);
//     }
//
//     #[rstest]
//     #[case(new_label(0), new_label(10), vec![], vec![])]
//     #[case(new_label(0), new_label(10), vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
//     #[case(new_label(20), new_label(30), vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
//     #[case(new_label(30), None, vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
//     #[case(new_label(0), new_label(30), vec![new_handler(10, 20)], vec![])]
//     fn test_remove_range_remove_all_or_nothing<L1, L2>(
//         #[case] start_pc: L1,
//         #[case] end_pc: L2,
//         #[case] input_handlers: Vec<Handler>,
//         #[case] expected_handlers: Vec<Handler>,
//     ) where
//         L1: Into<Option<Label>>,
//         L2: Into<Option<Label>>,
//     {
//         let mut handlers = input_handlers.to_owned();
//
//         Handler::remove_range(&mut handlers, &start_pc.into(), &end_pc.into()).unwrap();
//
//         assert_eq!(expected_handlers, handlers);
//     }
//
//     #[test]
//     fn test_remove_range_remove_start() {
//         let mut handlers = vec![new_handler(10, 20)];
//
//         Handler::remove_range(&mut handlers, &new_label(0).into(), &new_label(15).into()).unwrap();
//
//         assert_eq!(1, handlers.len());
//         let Handler {
//             start_pc,
//             end_pc,
//             handler_pc: _,
//             catch_type: _,
//             catch_type_descriptor: _,
//         } = &handlers[0];
//
//         assert!(start_pc.is_some());
//
//         let start_pc = start_pc.as_ref().unwrap();
//
//         assert_eq!(15, start_pc.bytecode_offset);
//
//         assert!(end_pc.is_some());
//
//         let end_pc = end_pc.as_ref().unwrap();
//
//         assert_eq!(20, end_pc.bytecode_offset);
//     }
//
//     #[test]
//     fn test_remove_range_remove_middle() {
//         let mut handlers = vec![new_handler(10, 20)];
//
//         Handler::remove_range(&mut handlers, &new_label(13).into(), &new_label(17).into()).unwrap();
//
//         assert_eq!(2, handlers.len());
//
//         // Assert first handler
//         let Handler {
//             start_pc,
//             end_pc,
//             handler_pc: _,
//             catch_type: _,
//             catch_type_descriptor: _,
//         } = &handlers[0];
//
//         assert!(start_pc.is_some());
//
//         let start_pc = start_pc.as_ref().unwrap();
//
//         assert_eq!(10, start_pc.bytecode_offset);
//
//         assert!(end_pc.is_some());
//
//         let end_pc = end_pc.as_ref().unwrap();
//
//         assert_eq!(13, end_pc.bytecode_offset);
//
//         // Assert second handler
//         let Handler {
//             start_pc,
//             end_pc,
//             handler_pc: _,
//             catch_type: _,
//             catch_type_descriptor: _,
//         } = &handlers[1];
//
//         assert!(start_pc.is_some());
//
//         let start_pc = start_pc.as_ref().unwrap();
//
//         assert_eq!(17, start_pc.bytecode_offset);
//
//         assert!(end_pc.is_some());
//
//         let end_pc = end_pc.as_ref().unwrap();
//
//         assert_eq!(20, end_pc.bytecode_offset);
//     }
//
//     #[test]
//     fn test_remove_range_remove_end() {
//         let mut handlers = vec![new_handler(10, 20)];
//
//         Handler::remove_range(&mut handlers, &new_label(15).into(), &new_label(30).into()).unwrap();
//
//         assert_eq!(1, handlers.len());
//
//         let Handler {
//             start_pc,
//             end_pc,
//             handler_pc: _,
//             catch_type: _,
//             catch_type_descriptor: _,
//         } = &handlers[0];
//
//         assert!(start_pc.is_some());
//
//         let start_pc = start_pc.as_ref().unwrap();
//
//         assert_eq!(10, start_pc.bytecode_offset);
//
//         assert!(end_pc.is_some());
//
//         let end_pc = end_pc.as_ref().unwrap();
//
//         assert_eq!(15, end_pc.bytecode_offset);
//     }
// }
