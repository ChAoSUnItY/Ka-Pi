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
        let handlers_len = handlers.len();
        let mut discarded_count = 0;
        
        for i in (0..handlers_len).rev() {
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
                continue;
            }

            if range_start <= handler_start {
                if range_end >= handler_end {
                    handlers.swap(i, handlers_len - 1);
                    discarded_count += 1;
                } else {
                    handlers[i] = Self::from_handler(&handler, end.clone(), handler.end_pc.clone());
                }
            } else if range_end >= handler_end {
                handlers[i] = Self::from_handler(&handler, handler.start_pc.clone(), start.clone());
            } else {
                handlers.insert(
                    i + 1,
                    Self::from_handler(&handler, end.clone(), handler.end_pc.clone()),
                );
                handlers[i] = Self::from_handler(&handler, handler.start_pc.clone(), start.clone());
            }
        }
        
        handlers.truncate(handlers_len - discarded_count);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::asm::handler::Handler;
    use crate::asm::label::Label;

    fn new_handler(start_pc: i32, end_pc: i32) -> Handler {
        Handler::new(
            new_label(start_pc).into(),
            new_label(end_pc).into(),
            new_label(0).into(),
            0,
            String::from(""),
        )
    }

    fn new_label(pc: i32) -> Label {
        let mut label = Label::new();
        label.bytecode_offset = pc;
        label
    }

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
        let copied_handler =
            Handler::from_handler(&handler, Label::new().into(), Label::new().into());

        assert_eq!(handler, copied_handler);
    }

    #[rstest]
    #[case(new_label(0), new_label(10), vec![], vec![])]
    #[case(new_label(0), new_label(10), vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
    #[case(new_label(20), new_label(30), vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
    #[case(new_label(30), None, vec![new_handler(10, 20)], vec![new_handler(10, 20)])]
    #[case(new_label(0), new_label(30), vec![new_handler(10, 20)], vec![])]
    fn test_remove_range_remove_all_or_nothing<L1, L2>(
        #[case] start_pc: L1,
        #[case] end_pc: L2,
        #[case] input_handlers: Vec<Handler>,
        #[case] expected_handlers: Vec<Handler>,
    ) where L1: Into<Option<Label>>, L2: Into<Option<Label>> {
        let mut handlers = input_handlers.to_owned();

        Handler::remove_range(&mut handlers, &start_pc.into(), &end_pc.into()).unwrap();
        
        assert_eq!(
            expected_handlers,
            handlers
        );
    }
}
