use crate::asm::byte_vec::ByteVec;
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
            next_handler: None,
        }
    }

    pub(crate) fn start_pc(&self) -> Option<i32> {
        self.start_pc.as_ref().map(|label| label.bytecode_offset)
    }

    pub(crate) fn end_pc(&self) -> Option<i32> {
        self.end_pc.as_ref().map(|label| label.bytecode_offset)
    }

    pub(crate) fn handler_pc(&self) -> Option<i32> {
        self.handler_pc.as_ref().map(|label| label.bytecode_offset)
    }

    pub(crate) fn from_handler(
        handler: &Self,
        start_pc: &Option<Label>,
        end_pc: &Option<Label>,
    ) -> Self {
        let mut new_handler = Self::new(
            start_pc.clone(),
            end_pc.clone(),
            handler.handler_pc.clone(),
            handler.catch_type,
            handler.catch_type_descriptor.clone(),
        );
        new_handler.next_handler = handler.next_handler.clone();
        new_handler
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

    pub(crate) fn exception_table_len(&self) -> usize {
        let mut len = 1;
        let mut next_handler = &self.next_handler;
        while let Some(handler) = next_handler {
            next_handler = &handler.next_handler;
            len += 1;
        }
        len
    }

    pub(crate) fn exception_table_size(&self) -> usize {
        2 + 8 * self.exception_table_len()
    }

    pub(crate) fn put(&self, output: &mut impl ByteVec) -> KapiResult<()> {
        output.put_u8s(&self.exception_table_len().to_ne_bytes()[..2]);
        self.write_bytes(output)?;
        let mut next_handler = &self.next_handler;
        
        while let Some(handler) = next_handler {
            handler.write_bytes(output)?;
            next_handler = &handler.next_handler;
        }

        Ok(())
    }

    fn write_bytes(&self, output: &mut impl ByteVec) -> KapiResult<()> {
        output.put(self.start_pc().ok_or(KapiError::StateError("Handler start label must not be None"))? as u16);
        output.put(self.end_pc().ok_or(KapiError::StateError("Handler end label must not be None"))? as u16);
        output.put(self.handler_pc().ok_or(KapiError::StateError("Handler handler label must not be None"))? as u16);
        output.put(self.catch_type as u16);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::asm::byte_vec::{ByteVec, ByteVecImpl};

    use crate::asm::handler::Handler;
    use crate::asm::label::Label;
    use crate::error::{KapiError, KapiResult};

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
        let copied_handler =
            Handler::from_handler(&handler, &Label::default().into(), &Label::default().into());

        assert_eq!(handler, copied_handler);
    }

    #[rstest]
    #[case(
        new_label(0),
        new_label(10),
        new_handler(10, 20),
        Some(new_handler(10, 20))
    )]
    #[case(
        new_label(20),
        new_label(30),
        new_handler(10, 20),
        Some(new_handler(10, 20))
    )]
    #[case(new_label(30), None, new_handler(10, 20), Some(new_handler(10, 20)))]
    #[case(new_label(0), new_label(30), new_handler(10, 20), None)]
    fn test_remove_range_remove_all_or_nothing<L1, L2>(
        #[case] start_pc: L1,
        #[case] end_pc: L2,
        #[case] mut input_handler: Handler,
        #[case] expected_handler: Option<Handler>,
    ) -> KapiResult<()>
    where
        L1: Into<Option<Label>>,
        L2: Into<Option<Label>>,
    {
        let handler = input_handler.remove_range(&start_pc.into(), &end_pc.into())?;

        assert_eq!(expected_handler, handler);

        Ok(())
    }

    #[test]
    fn test_remove_range_remove_start() -> KapiResult<()> {
        let handler =
            new_handler(10, 20).remove_range(&new_label(0).into(), &new_label(15).into())?;

        assert!(handler.is_some());

        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
            next_handler,
        } = &handler.unwrap();

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(15, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(20, end_pc.bytecode_offset);

        assert!(next_handler.is_none());

        Ok(())
    }

    #[test]
    fn test_remove_range_remove_middle() -> KapiResult<()> {
        let handler =
            new_handler(10, 20).remove_range(&new_label(13).into(), &new_label(17).into())?;

        assert!(handler.is_some());

        // Assert first handler
        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
            next_handler,
        } = &handler.unwrap();

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(10, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(13, end_pc.bytecode_offset);

        assert!(next_handler.is_some());

        // Assert second handler
        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
            next_handler,
        } = next_handler.as_ref().unwrap().as_ref();

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(17, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(20, end_pc.bytecode_offset);

        assert!(next_handler.is_none());

        Ok(())
    }

    #[test]
    fn test_remove_range_remove_end() -> KapiResult<()> {
        let handler =
            new_handler(10, 20).remove_range(&new_label(15).into(), &new_label(30).into())?;

        assert!(handler.is_some());

        let Handler {
            start_pc,
            end_pc,
            handler_pc: _,
            catch_type: _,
            catch_type_descriptor: _,
            next_handler,
        } = &handler.unwrap();

        assert!(start_pc.is_some());

        let start_pc = start_pc.as_ref().unwrap();

        assert_eq!(10, start_pc.bytecode_offset);

        assert!(end_pc.is_some());

        let end_pc = end_pc.as_ref().unwrap();

        assert_eq!(15, end_pc.bytecode_offset);

        assert!(next_handler.is_none());

        Ok(())
    }
    
    #[test]
    fn test_exception_table_len() {
        let handler = new_handler(10, 20);
        
        assert_eq!(1, handler.exception_table_len());


    }
    
    #[test]
    fn test_exception_table_size() -> KapiResult<()> {
        let handler = new_handler(10, 20).remove_range(&new_label(13).into(), &new_label(17).into())?;
        
        assert!(handler.is_some());
        
        let handler = handler.unwrap();
        
        assert_eq!(18, handler.exception_table_size());
        
        Ok(())
    }
    
    #[test]
    fn test_put() -> KapiResult<()> {
        let mut output = ByteVecImpl::new();
        let handler = new_handler(10, 20).remove_range(&new_label(13).into(), &new_label(17).into())?;

        assert!(handler.is_some());
        
        let handler = handler.unwrap();
        
        handler.put(&mut output)?;
        
        assert_eq!(18, output.len());
        
        Ok(())
    }
}
