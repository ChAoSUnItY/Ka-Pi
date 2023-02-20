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
    pub(crate) fn new<L1, L2, L3>(
        start_pc: L1,
        end_pc: L2,
        handler_pc: L3,
        catch_type: i32,
        catch_type_descriptor: String,
    ) -> Self
    where
        L1: Into<Option<Label>>,
        L2: Into<Option<Label>>,
        L3: Into<Option<Label>>,
    {
        Self {
            start_pc: start_pc.into(),
            end_pc: end_pc.into(),
            handler_pc: handler_pc.into(),
            catch_type,
            catch_type_descriptor,
        }
    }

    pub(crate) fn from_handler<L1, L2>(handler: &Self, start_pc: L1, end_pc: L2) -> Self
    where
        L1: Into<Option<Label>>,
        L2: Into<Option<Label>>,
    {
        Self::new(
            start_pc,
            end_pc,
            handler.handler_pc.clone(),
            handler.catch_type,
            handler.catch_type_descriptor.clone(),
        )
    }

    pub(crate) fn remove_range<'start, 'end, L1, L2>(
        handlers: &mut Vec<Self>,
        start: L1,
        end: L2,
    ) -> KapiResult<()>
    where
        L1: Into<&'start Option<Label>>,
        L2: Into<&'end Option<Label>>,
    {
        let start = start.into();
        let end = end.into();
        let range_start = start
            .as_ref()
            .ok_or_else(|| KapiError::StateError("Label's start pc must not be None"))?
            .bytecode_offset;
        let range_end = if let Some(end_label) = end.as_ref() {
            end_label.bytecode_offset
        } else {
            i32::MAX
        };

        for i in (0..handlers.len()).rev() {
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

            if range_start >= handler_end || range_end <= handler_start {
                continue;
            }

            if range_start <= handler_start {
                if range_end >= handler_end {
                    handlers.remove(i);
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

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use rstest::rstest;

    use crate::asm::handler::Handler;
    use crate::asm::label::Label;

    fn new_handler(start_pc: i32, end_pc: i32) -> Handler {
        Handler::new(
            new_label(start_pc),
            new_label(end_pc),
            new_label(0),
            0,
            String::from(""),
        )
    }

    fn new_label(pc: i32) -> Label {
        let mut label = Label::default();
        label.bytecode_offset = pc;
        label
    }

    #[test]
    fn test_new_handler() {
        let handler = Handler::new(
            Label::default(),
            Label::default(),
            Label::default(),
            123,
            String::from("123"),
        );

        assert_eq!(Label::default(), handler.start_pc.unwrap());
        assert_eq!(Label::default(), handler.end_pc.unwrap());
        assert_eq!(Label::default(), handler.handler_pc.unwrap());
        assert_eq!(123, handler.catch_type);
        assert_eq!("123", handler.catch_type_descriptor);
    }

    #[test]
    fn test_copy_handler() {
        let handler = Handler::new(
            Label::default(),
            Label::default(),
            Label::default(),
            123,
            String::from("123"),
        );
        let copied_handler = Handler::from_handler(&handler, Label::default(), Label::default());

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
    ) where
        L1: Into<Option<Label>>,
        L2: Into<Option<Label>>,
    {
        let mut handlers = input_handlers.to_owned();

        Handler::remove_range(&mut handlers, &start_pc.into(), &end_pc.into()).unwrap();

        assert_eq!(expected_handlers, handlers);
    }

    #[test]
    fn test_remove_range_remove_start() {
        let mut handlers = vec![new_handler(10, 20)];

        Handler::remove_range(&mut handlers, &new_label(0).into(), &new_label(15).into()).unwrap();
        
        assert_eq!(1, handlers.len());
        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
        } = &handlers[0];
        
        assert!(start_pc.is_some());
        
        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(15, start_pc.bytecode_offset);
        
        assert!(end_pc.is_some());
        
        let end_pc = end_pc.as_ref().unwrap();
        
        assert_eq!(20, end_pc.bytecode_offset);
    }
    
    #[test]
    fn test_remove_range_remove_middle() {
        let mut handlers = vec![new_handler(10, 20)];

        Handler::remove_range(&mut handlers, &new_label(13).into(), &new_label(17).into()).unwrap();

        assert_eq!(2, handlers.len());
        
        // Assert first handler
        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
        } = &handlers[0];

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(10, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(13, end_pc.bytecode_offset);

        // Assert second handler
        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
        } = &handlers[1];

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(17, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(20, end_pc.bytecode_offset);
    }
    
    #[test]
    fn test_remove_range_remove_end() {
        let mut handlers = vec![new_handler(10, 20)];
        
        Handler::remove_range(&mut handlers, &new_label(15).into(), &new_label(30).into()).unwrap();
        
        assert_eq!(1, handlers.len());

        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
        } = &handlers[0];
        
        assert!(start_pc.is_some());
        
        let start_pc = start_pc.as_ref().unwrap();
        
        assert_eq!(10, start_pc.bytecode_offset);
        
        assert!(end_pc.is_some());
        
        let end_pc = end_pc.as_ref().unwrap();
        
        assert_eq!(15, end_pc.bytecode_offset);
    }
}
