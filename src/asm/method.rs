use std::cell::{RefCell, RefMut};
use std::cmp::max;
use std::ops::BitOr;
use std::rc::Rc;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::constants;
use crate::asm::label::Label;
use crate::asm::opcodes::{AccessFlag, ConstantObject, MethodAccessFlag, NativeOpcode, Opcode};
use crate::asm::symbol::SymbolTable;
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
    current_stack: usize,
    max_locals: usize,
    current_locals: usize,
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
            current_stack: 0,
            max_locals: 0,
            current_locals: 0,
            labels: Vec::new(),
        }
    }

    fn inc_stack(&mut self) {
        self.current_stack += 1;
        self.max_stack = max(self.current_stack, self.max_stack);
    }

    fn inc_locals(&mut self) {
        self.current_locals += 1;
        self.max_stack = max(self.current_locals, self.max_locals);
    }

    /// Emit opcode with a predefined [Opcode] enum structure. This is useful when exist functions
    /// does not have such feature implementation, however, using this might results in unexpected
    /// errors.
    ///
    /// # Unsafety
    ///
    /// This function might occur undesired behaviour based on give parameter **`opcode`**.
    pub(crate) unsafe fn visit_opcode(&mut self, opcode: Opcode) -> KapiResult<()> {
        match &opcode {
            Opcode::NOP => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
            }
            Opcode::ACONST_NULL => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_M1 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_0 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_1 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_2 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_3 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_4 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::ICONST_5 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::LCONST_0 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::LCONST_1 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::FCONST_0 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::FCONST_1 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::FCONST_2 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::DCONST_0 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::DCONST_1 => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.inc_stack();
            }
            Opcode::BIPUSH(val) => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.code_byte_vec.put_be(*val);
            }
            Opcode::SIPUSH(val) => {
                self.code_byte_vec.put_be(opcode.opcode_byte());
                self.code_byte_vec.put_be(*val);
            }
            Opcode::LDC(constant) => {
                self.code_byte_vec.put_be(opcode.opcode_byte());

                let constant_index = self.symbol_table.add_constant_object(constant);
            }
            Opcode::ILOAD(_) => {}
            Opcode::LLOAD(_) => {}
            Opcode::FLOAD(_) => {}
            Opcode::DLOAD(_) => {}
            Opcode::ALOAD(_) => {}
            Opcode::ILOAD_0 => {}
            Opcode::ILOAD_1 => {}
            Opcode::ILOAD_2 => {}
            Opcode::ILOAD_3 => {}
            Opcode::LLOAD_0 => {}
            Opcode::LLOAD_1 => {}
            Opcode::LLOAD_2 => {}
            Opcode::LLOAD_3 => {}
            Opcode::FLOAD_0 => {}
            Opcode::FLOAD_1 => {}
            Opcode::FLOAD_2 => {}
            Opcode::FLOAD_3 => {}
            Opcode::DLOAD_0 => {}
            Opcode::DLOAD_1 => {}
            Opcode::DLOAD_2 => {}
            Opcode::DLOAD_3 => {}
            Opcode::ALOAD_0 => {}
            Opcode::ALOAD_1 => {}
            Opcode::ALOAD_2 => {}
            Opcode::ALOAD_3 => {}
            Opcode::IALOAD => {}
            Opcode::LALOAD => {}
            Opcode::FALOAD => {}
            Opcode::DALOAD => {}
            Opcode::AALOAD => {}
            Opcode::BALOAD => {}
            Opcode::CALOAD => {}
            Opcode::SALOAD => {}
            Opcode::ISTORE(_) => {}
            Opcode::LSTORE(_) => {}
            Opcode::FSTORE(_) => {}
            Opcode::DSTORE(_) => {}
            Opcode::ASTORE(_) => {}
            Opcode::ISTORE_0 => {}
            Opcode::ISTORE_1 => {}
            Opcode::ISTORE_2 => {}
            Opcode::ISTORE_3 => {}
            Opcode::LSTORE_0 => {}
            Opcode::LSTORE_1 => {}
            Opcode::LSTORE_2 => {}
            Opcode::LSTORE_3 => {}
            Opcode::FSTORE_0 => {}
            Opcode::FSTORE_1 => {}
            Opcode::FSTORE_2 => {}
            Opcode::FSTORE_3 => {}
            Opcode::DSTORE_0 => {}
            Opcode::DSTORE_1 => {}
            Opcode::DSTORE_2 => {}
            Opcode::DSTORE_3 => {}
            Opcode::ASTORE_0 => {}
            Opcode::ASTORE_1 => {}
            Opcode::ASTORE_2 => {}
            Opcode::ASTORE_3 => {}
            Opcode::IASTORE => {}
            Opcode::LASTORE => {}
            Opcode::FASTORE => {}
            Opcode::DASTORE => {}
            Opcode::AASTORE => {}
            Opcode::BASTORE => {}
            Opcode::CASTORE => {}
            Opcode::SASTORE => {}
            Opcode::POP => {}
            Opcode::POP2 => {}
            Opcode::DUP => {}
            Opcode::DUP_X1 => {}
            Opcode::DUP_X2 => {}
            Opcode::DUP2 => {}
            Opcode::DUP2_X1 => {}
            Opcode::DUP2_X2 => {}
            Opcode::SWAP => {}
            Opcode::IADD => {}
            Opcode::LADD => {}
            Opcode::FADD => {}
            Opcode::DADD => {}
            Opcode::ISUB => {}
            Opcode::LSUB => {}
            Opcode::FSUB => {}
            Opcode::DSUB => {}
            Opcode::IMUL => {}
            Opcode::LMUL => {}
            Opcode::FMUL => {}
            Opcode::DMUL => {}
            Opcode::IDIV => {}
            Opcode::LDIV => {}
            Opcode::FDIV => {}
            Opcode::DDIV => {}
            Opcode::IREM => {}
            Opcode::LREM => {}
            Opcode::FREM => {}
            Opcode::DREM => {}
            Opcode::INEG => {}
            Opcode::LNEG => {}
            Opcode::FNEG => {}
            Opcode::DNEG => {}
            Opcode::ISHL => {}
            Opcode::LSHL => {}
            Opcode::ISHR => {}
            Opcode::LSHR => {}
            Opcode::IUSHR => {}
            Opcode::LUSHR => {}
            Opcode::IAND => {}
            Opcode::LAND => {}
            Opcode::IOR => {}
            Opcode::LOR => {}
            Opcode::IXOR => {}
            Opcode::LXOR => {}
            Opcode::IINC(_, _) => {}
            Opcode::I2L => {}
            Opcode::I2F => {}
            Opcode::I2D => {}
            Opcode::L2I => {}
            Opcode::L2F => {}
            Opcode::L2D => {}
            Opcode::F2I => {}
            Opcode::F2L => {}
            Opcode::F2D => {}
            Opcode::D2I => {}
            Opcode::D2L => {}
            Opcode::D2F => {}
            Opcode::I2B => {}
            Opcode::I2C => {}
            Opcode::I2S => {}
            Opcode::LCMP => {}
            Opcode::FCMPL => {}
            Opcode::FCMPG => {}
            Opcode::DCMPL => {}
            Opcode::DCMPG => {}
            Opcode::IFEQ(_) => {}
            Opcode::IFNE(_) => {}
            Opcode::IFLT(_) => {}
            Opcode::IFGE(_) => {}
            Opcode::IFGT(_) => {}
            Opcode::IFLE(_) => {}
            Opcode::IF_ICMPEQ(_) => {}
            Opcode::IF_ICMPNE(_) => {}
            Opcode::IF_ICMPLT(_) => {}
            Opcode::IF_ICMPGE(_) => {}
            Opcode::IF_ICMPGT(_) => {}
            Opcode::IF_ICMPLE(_) => {}
            Opcode::IF_ACMPEQ(_) => {}
            Opcode::IF_ACMPNE(_) => {}
            Opcode::GOTO(_) => {}
            Opcode::JSR(_) => {}
            Opcode::RET(_) => {}
            Opcode::TABLESWITCH => {}
            Opcode::LOOKUPSWITCH => {}
            Opcode::IRETURN => {}
            Opcode::LRETURN => {}
            Opcode::FRETURN => {}
            Opcode::DRETURN => {}
            Opcode::ARETURN => {}
            Opcode::RETURN => {}
            Opcode::GETSTATIC => {}
            Opcode::PUTSTATIC => {}
            Opcode::GETFIELD => {}
            Opcode::PUTFIELD => {}
            Opcode::INVOKEVIRTUAL => {}
            Opcode::INVOKESPECIAL => {}
            Opcode::INVOKESTATIC => {}
            Opcode::INVOKEINTERFACE => {}
            Opcode::INVOKEDYNAMIC => {}
            Opcode::NEW => {}
            Opcode::NEWARRAY => {}
            Opcode::ANEWARRAY => {}
            Opcode::ARRAYLENGTH => {}
            Opcode::ATHROW => {}
            Opcode::CHECKCAST => {}
            Opcode::INSTANCEOF => {}
            Opcode::MONITORENTER => {}
            Opcode::MONITOREXIT => {}
            Opcode::MULTIANEWARRAY => {}
            Opcode::IFNULL => {}
            Opcode::IFNONNULL => {}
        }

        Ok(())
    }

    pub(crate) fn visit_ldc<C>(&mut self, constant_value: C)
    where
        C: Into<ConstantObject>,
    {
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
            current_stack: _,
            max_locals,
            current_locals: _,
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
            let attribute_name_index = symbol_table.add_utf8(constants::CODE);
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
    use crate::asm::label::Label;
    use crate::asm::method::{MethodVisitor, MethodWriter};
    use crate::asm::opcodes::{MethodAccessFlag, NativeOpcode};
    use crate::asm::symbol::SymbolTable;
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

        assert_eq!(
            &bv[..],
            [
                0, 8, 0, 1, 0, 2, 0, 1, 0, 3, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 3, 167, 0, 3, 0, 0,
                0, 0
            ]
        );

        Ok(())
    }
}
