use std::cell::{RefCell, RefMut};
use std::ops::BitOr;
use std::rc::Rc;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::opcodes::{AccessFlag, MethodAccessFlag, NativeOpcode};
use crate::asm::symbol::SymbolTable;
use crate::asm::label::Label;
use crate::error::KapiResult;

pub trait MethodVisitor {
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub struct MethodWriter<'output> {
    byte_vec: &'output mut ByteVecImpl,
    symbol_table: &'output mut SymbolTable,
    access: &'output [MethodAccessFlag],
    name_index: u16,
    descriptor_index: u16,
    code_byte_vec: ByteVecImpl,
    max_stack: usize,
    max_locals: usize,
    labels: Vec<(u32, Rc<RefCell<Label>>)>,
}

impl<'output> MethodWriter<'output> {
    pub(crate) fn new(
        byte_vec: &'output mut ByteVecImpl,
        symbol_table: &'output mut SymbolTable,
        access: &'output [MethodAccessFlag],
        name: &str,
        descriptor: &str,
    ) -> Self {
        let name_index = symbol_table.add_utf8(name);
        let descriptor_index = symbol_table.add_utf8(descriptor);

        Self {
            byte_vec,
            symbol_table,
            access,
            name_index,
            descriptor_index,
            code_byte_vec: ByteVecImpl::default(),
            max_stack: 0,
            max_locals: 0,
            labels: Vec::new(),
        }
    }

    pub(crate) fn visit_jmp(
        &mut self,
        jmp_opcode: NativeOpcode,
        destination_label: &Rc<RefCell<Label>>,
    ) {
        self.labels
            .push((self.code_byte_vec.len() as u32, destination_label.clone()));
        self.code_byte_vec.put_u8(jmp_opcode as u8);
        self.code_byte_vec.put_u8s(&[0, 0]); // Placeholder
    }

    pub(crate) fn visit_label(&mut self, destination_label: Rc<RefCell<Label>>) -> KapiResult<()> {
        RefMut::map(destination_label.borrow_mut(), |label| {
            *label = Label((self.code_byte_vec.len()) as u32);
            label
        });

        Ok(())
    }
}

impl<'output> MethodVisitor for MethodWriter<'output> {
    fn visit_end(self)
    where
        Self: Sized,
    {
        let Self {
            byte_vec,
            symbol_table,
            access,
            name_index,
            descriptor_index,
            mut code_byte_vec,
            max_stack,
            max_locals,
            labels,
        } = self;

        if code_byte_vec.len() != 0 {
            // Retrieve label's offset and put back to correct position
            for (start_index, label) in labels {
                let index = start_index as usize;
                let label = label.borrow();

                code_byte_vec[index + 1..=index + 2]
                    .swap_with_slice(&mut ((label.0 - start_index) as i16).to_be_bytes());
            }
        }
        
        // Generate method
        byte_vec.put_be(access.fold_flags());
        byte_vec.put_be(name_index);
        byte_vec.put_be(descriptor_index);
        // TODO: Remove attribute_len hardcode
        let mut attribute_len = 0u16;
        
        if code_byte_vec.len() != 0 {
            attribute_len += 1;
        }
        
        byte_vec.put_be(attribute_len);

        // If code_byte_vec is empty, do not emit Code attribute for the method
        if code_byte_vec.len() != 0 {
            let attribute_name_index = symbol_table.add_utf8("Code");
            let code_len = code_byte_vec.len();
            let attribute_len = 12 + code_len; // TODO
            
            byte_vec.put_be(attribute_name_index); // attribute_name_index
            byte_vec.put_be(attribute_len as u32); // attribute_length
            byte_vec.put_be(max_stack as u16); // max_stack
            byte_vec.put_be(max_locals as u16); // max_locals
            byte_vec.put_be(code_len as u32); // code_length
            byte_vec.append(&mut code_byte_vec); // code[code_length]
            // TODO: Implement exceptions
            byte_vec.put_be(0u16);
            // TODO_END
            // TODO: Implement attributes
            byte_vec.put_be(0u16);
            // TODO_END
        }
    }
}

#[cfg(test)]
mod test {
    use crate::asm::byte_vec::ByteVecImpl;
    use crate::asm::method::{MethodVisitor, MethodWriter};
    use crate::asm::opcodes::{MethodAccessFlag, NativeOpcode};
    use crate::asm::symbol::SymbolTable;
    use crate::asm::label::Label;
    use crate::error::KapiResult;
    
    #[test]
    fn test_method_writer_init() {
        let mut bv = ByteVecImpl::new();
        let mut table = SymbolTable::default();
        let mut mv = MethodWriter::new(
            &mut bv,
            &mut table,
            &[MethodAccessFlag::Static],
            "Main",
            "()",
        );
        
        mv.visit_end();
        
        assert_eq!(&bv[..], [0, 8, 0, 1, 0, 2, 0, 0])
    }

    #[test]
    fn test_method_writer_label_visit() -> KapiResult<()> {
        let mut bv = ByteVecImpl::new();
        let mut table = SymbolTable::default();
        let mut mv = MethodWriter::new(
            &mut bv,
            &mut table,
            &[MethodAccessFlag::Static],
            "Main",
            "()",
        );
        let mut label = Label::new_label();

        mv.visit_jmp(NativeOpcode::GOTO, &label);
        mv.visit_label(label)?;
        mv.visit_end();

        assert_eq!(&bv[..], [0, 8, 0, 1, 0, 2, 0, 1, 0, 3, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 3, 167, 0, 3, 0, 0, 0, 0]);

        Ok(())
    }
}
