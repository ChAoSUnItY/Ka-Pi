use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::opcodes::{MethodAccessFlag, NativeOpcode};
use crate::asm::symbol::SymbolTable;
use crate::asm::Label::Label;
use crate::error::{KapiError, KapiResult};

pub trait MethodVisitor {
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub struct MethodWriter<'output>
{
    byte_vec: &'output mut ByteVecImpl,
    symbol_table: &'output mut SymbolTable,
    access: MethodAccessFlag,
    name_index: u16,
    descriptor_index: u16,
    code_byte_vec: ByteVecImpl,
    labels: Vec<(u32, Rc<RefCell<Label>>)>,
}

impl<'output> MethodWriter<'output>
{
    pub(crate) fn new(
        byte_vec: &'output mut ByteVecImpl,
        symbol_table: &'output mut SymbolTable,
        access: MethodAccessFlag,
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
            labels: Vec::new(),
        }
    }

    pub(crate) fn visit_jmp(
        &mut self,
        jmp_opcode: NativeOpcode,
        destination_label: &Rc<RefCell<Label>>,
    ) {
        self.labels.push((
            self.code_byte_vec.len() as u32,
            destination_label.clone(),
        ));
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

impl<'output> MethodVisitor for MethodWriter<'output>
{
    fn visit_end(self)
    where
        Self: Sized,
    {
        let byte_vec = self.byte_vec;
        let mut code_byte_vec = self.code_byte_vec;
        
        // Retrieve label's offset and put back to correct position
        for (start_index, label) in self.labels {
            let index = start_index as usize;
            let label = label.borrow();
            
            code_byte_vec[index + 1..=index + 2].swap_with_slice(&mut ((label.0 - start_index) as i16).to_be_bytes());
        }
        
        byte_vec.append(&mut code_byte_vec);
    }
}

#[cfg(test)]
mod test {
    use crate::asm::byte_vec::ByteVecImpl;
    use crate::asm::method::{MethodVisitor, MethodWriter};
    use crate::asm::opcodes::{MethodAccessFlag, NativeOpcode};
    use crate::asm::symbol::SymbolTable;
    use crate::asm::Label::Label;
    use crate::error::KapiResult;

    #[test]
    fn test_method_writer_label_visit() -> KapiResult<()> {
        let mut bv = ByteVecImpl::new();
        let mut table = SymbolTable::default();
        let mut mv = MethodWriter::new(&mut bv, &mut table, MethodAccessFlag::Static, "Main", "()");
        let mut label = Label::new_label();

        mv.visit_jmp(NativeOpcode::GOTO, &label);
        mv.visit_label(label)?;
        mv.visit_end();

        assert_eq!(&bv[..], [167, 0, 3]);

        Ok(())
    }
}
