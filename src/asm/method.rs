use std::cell::{RefCell, RefMut};
use std::cmp::max;
use std::rc::Rc;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::label::Label;
use crate::asm::node::access_flag::{AccessFlags, MethodAccessFlag};
use crate::asm::node::attribute::CODE;
use crate::asm::node::opcode::{ConstantObject, Instruction, Opcode};
use crate::asm::symbol::SymbolTable;
use crate::asm::types::Type;
use crate::error::KapiResult;

pub trait MethodVisitor {
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct MethodVisitorImpl {}

impl MethodVisitor for MethodVisitorImpl {}

pub struct MethodWriter {
    // Internal writing buffers
    byte_vec: Rc<RefCell<ByteVecImpl>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    code_byte_vec: ByteVecImpl,
    // Class file format defined fields
    access_flags: Vec<MethodAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    max_stack: usize,
    max_locals: usize,
    // State utilities
    current_stack: usize,
    current_locals: usize,
    labels: Vec<(u32, Rc<RefCell<Label>>)>,
}

impl MethodWriter {
    pub(crate) fn new<F>(
        byte_vec: &Rc<RefCell<ByteVecImpl>>,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        access_flags: F,
        name: &str,
        descriptor: &str,
    ) -> KapiResult<Self>
    where
        F: IntoIterator<Item = MethodAccessFlag>,
    {
        let access_flags = access_flags.into_iter().collect::<Vec<_>>();
        let name_index = symbol_table.borrow_mut().add_utf8(name);
        let descriptor_index = symbol_table.borrow_mut().add_utf8(descriptor);

        let mut initial_locals = Type::from_method_descriptor(descriptor)?
            .0
            .iter()
            .map(Type::size)
            .sum::<usize>();
        if !access_flags.contains(&MethodAccessFlag::Static) {
            initial_locals += 1;
        }

        Ok(Self {
            byte_vec: byte_vec.clone(),
            symbol_table: symbol_table.clone(),
            access_flags,
            name_index,
            descriptor_index,
            code_byte_vec: ByteVecImpl::default(),
            max_stack: 0,
            current_stack: 0,
            max_locals: initial_locals,
            current_locals: 0,
            labels: Vec::new(),
        })
    }

    fn inc_stack(&mut self) {
        self.current_stack += 1;
        self.max_stack = max(self.current_stack, self.max_stack);
    }

    fn dec_stack(&mut self) {
        self.current_stack -= 1;
    }

    fn inc_locals(&mut self) {
        self.current_locals += 1;
        self.max_stack = max(self.current_locals, self.max_locals);
    }

    fn put_opcode(&mut self, opcode: Opcode) {
        self.code_byte_vec.put_be(opcode as u8);
    }

    /// Emit opcode with a [Opcode], following by the companion data which has dynamic length
    /// based on given opcode. This is useful when exist functions does not have such feature implementation,
    /// however, using this might results in unexpected errors.
    ///
    /// # Unsafety
    ///
    /// This function is highly unrecommended to use unless you know what you are doing, and also might
    /// occur undesired behaviour (very possibly leads to invalid bytecode) based on given parameters.
    ///
    /// Use [Self::visit_inst] instead for your own safety.
    pub unsafe fn visit_inst_raw(&mut self, opcode: Opcode, companion_data: &[u8]) {
        self.code_byte_vec.put_u8(opcode as u8);
        self.code_byte_vec.put_u8s(companion_data);
    }

    /// Emit instruction with a [Instruction]. This is useful when exist functions
    /// does not have such feature implementation, however, using this might results in unexpected
    /// errors.
    ///
    /// # Unsafety
    ///
    /// This function might occur undesired behaviour based on given parameter **`opcode`**.
    pub unsafe fn visit_inst(&mut self, inst: Instruction) -> KapiResult<()> {
        match &inst {
            Instruction::NOP => {
                self.put_opcode(inst.opcode());
            }
            Instruction::ACONST_NULL => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_M1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_2 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_3 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_4 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::ICONST_5 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::LCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::LCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::FCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::FCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::FCONST_2 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::DCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::DCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack();
            }
            Instruction::BIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
            }
            Instruction::SIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
            }
            Instruction::LDC(constant) => {
                self.put_opcode(inst.opcode());

                let constant_index = self.symbol_table.borrow_mut().add_constant_object(constant);

                self.code_byte_vec.put_be(constant_index);
                self.inc_stack();
            }
            Instruction::LDC_W(constants) => {}
            Instruction::LDC2_W(constants) => {}
            Instruction::ILOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack();
            }
            Instruction::LLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack();
            }
            Instruction::FLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack();
            }
            Instruction::DLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack();
            }
            Instruction::ALOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack();
            }
            Instruction::ILOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.inc_stack();
            }
            Instruction::ILOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.inc_stack();
            }
            Instruction::ILOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.inc_stack();
            }
            Instruction::ILOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.inc_stack();
            }
            Instruction::LLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.inc_stack();
            }
            Instruction::LLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.inc_stack();
            }
            Instruction::LLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.inc_stack();
            }
            Instruction::LLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.inc_stack();
            }
            Instruction::FLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.inc_stack();
            }
            Instruction::FLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.inc_stack();
            }
            Instruction::FLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.inc_stack();
            }
            Instruction::FLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.inc_stack();
            }
            Instruction::DLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.inc_stack();
            }
            Instruction::DLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.inc_stack();
            }
            Instruction::DLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.inc_stack();
            }
            Instruction::DLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.inc_stack();
            }
            Instruction::ALOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.inc_stack();
            }
            Instruction::ALOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.inc_stack();
            }
            Instruction::ALOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.inc_stack();
            }
            Instruction::ALOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.inc_stack();
            }
            Instruction::IALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::LALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::FALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::DALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::AALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::BALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::CALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::SALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack();
            }
            Instruction::ISTORE(_) => {}
            Instruction::LSTORE(_) => {}
            Instruction::FSTORE(_) => {}
            Instruction::DSTORE(_) => {}
            Instruction::ASTORE(_) => {}
            Instruction::ISTORE_0 => {}
            Instruction::ISTORE_1 => {}
            Instruction::ISTORE_2 => {}
            Instruction::ISTORE_3 => {}
            Instruction::LSTORE_0 => {}
            Instruction::LSTORE_1 => {}
            Instruction::LSTORE_2 => {}
            Instruction::LSTORE_3 => {}
            Instruction::FSTORE_0 => {}
            Instruction::FSTORE_1 => {}
            Instruction::FSTORE_2 => {}
            Instruction::FSTORE_3 => {}
            Instruction::DSTORE_0 => {}
            Instruction::DSTORE_1 => {}
            Instruction::DSTORE_2 => {}
            Instruction::DSTORE_3 => {}
            Instruction::ASTORE_0 => {}
            Instruction::ASTORE_1 => {}
            Instruction::ASTORE_2 => {}
            Instruction::ASTORE_3 => {}
            Instruction::IASTORE => {}
            Instruction::LASTORE => {}
            Instruction::FASTORE => {}
            Instruction::DASTORE => {}
            Instruction::AASTORE => {}
            Instruction::BASTORE => {}
            Instruction::CASTORE => {}
            Instruction::SASTORE => {}
            Instruction::POP => {}
            Instruction::POP2 => {}
            Instruction::DUP => {}
            Instruction::DUP_X1 => {}
            Instruction::DUP_X2 => {}
            Instruction::DUP2 => {}
            Instruction::DUP2_X1 => {}
            Instruction::DUP2_X2 => {}
            Instruction::SWAP => {}
            Instruction::IADD => {}
            Instruction::LADD => {}
            Instruction::FADD => {}
            Instruction::DADD => {}
            Instruction::ISUB => {}
            Instruction::LSUB => {}
            Instruction::FSUB => {}
            Instruction::DSUB => {}
            Instruction::IMUL => {}
            Instruction::LMUL => {}
            Instruction::FMUL => {}
            Instruction::DMUL => {}
            Instruction::IDIV => {}
            Instruction::LDIV => {}
            Instruction::FDIV => {}
            Instruction::DDIV => {}
            Instruction::IREM => {}
            Instruction::LREM => {}
            Instruction::FREM => {}
            Instruction::DREM => {}
            Instruction::INEG => {}
            Instruction::LNEG => {}
            Instruction::FNEG => {}
            Instruction::DNEG => {}
            Instruction::ISHL => {}
            Instruction::LSHL => {}
            Instruction::ISHR => {}
            Instruction::LSHR => {}
            Instruction::IUSHR => {}
            Instruction::LUSHR => {}
            Instruction::IAND => {}
            Instruction::LAND => {}
            Instruction::IOR => {}
            Instruction::LOR => {}
            Instruction::IXOR => {}
            Instruction::LXOR => {}
            Instruction::IINC(_, _) => {}
            Instruction::I2L => {}
            Instruction::I2F => {}
            Instruction::I2D => {}
            Instruction::L2I => {}
            Instruction::L2F => {}
            Instruction::L2D => {}
            Instruction::F2I => {}
            Instruction::F2L => {}
            Instruction::F2D => {}
            Instruction::D2I => {}
            Instruction::D2L => {}
            Instruction::D2F => {}
            Instruction::I2B => {}
            Instruction::I2C => {}
            Instruction::I2S => {}
            Instruction::LCMP => {}
            Instruction::FCMPL => {}
            Instruction::FCMPG => {}
            Instruction::DCMPL => {}
            Instruction::DCMPG => {}
            Instruction::IFEQ(_) => {}
            Instruction::IFNE(_) => {}
            Instruction::IFLT(_) => {}
            Instruction::IFGE(_) => {}
            Instruction::IFGT(_) => {}
            Instruction::IFLE(_) => {}
            Instruction::IF_ICMPEQ(_) => {}
            Instruction::IF_ICMPNE(_) => {}
            Instruction::IF_ICMPLT(_) => {}
            Instruction::IF_ICMPGE(_) => {}
            Instruction::IF_ICMPGT(_) => {}
            Instruction::IF_ICMPLE(_) => {}
            Instruction::IF_ACMPEQ(_) => {}
            Instruction::IF_ACMPNE(_) => {}
            Instruction::GOTO(_) => {}
            Instruction::JSR(_) => {}
            Instruction::RET(_) => {}
            Instruction::TABLESWITCH => {}
            Instruction::LOOKUPSWITCH => {}
            Instruction::IRETURN => {}
            Instruction::LRETURN => {}
            Instruction::FRETURN => {}
            Instruction::DRETURN => {}
            Instruction::ARETURN => {}
            Instruction::RETURN => {}
            Instruction::GETSTATIC => {}
            Instruction::PUTSTATIC => {}
            Instruction::GETFIELD => {}
            Instruction::PUTFIELD => {}
            Instruction::INVOKEVIRTUAL => {}
            Instruction::INVOKESPECIAL => {}
            Instruction::INVOKESTATIC => {}
            Instruction::INVOKEINTERFACE => {}
            Instruction::INVOKEDYNAMIC => {}
            Instruction::NEW => {}
            Instruction::NEWARRAY => {}
            Instruction::ANEWARRAY => {}
            Instruction::ARRAYLENGTH => {}
            Instruction::ATHROW => {}
            Instruction::CHECKCAST => {}
            Instruction::INSTANCEOF => {}
            Instruction::MONITORENTER => {}
            Instruction::MONITOREXIT => {}
            Instruction::MULTIANEWARRAY => {}
            Instruction::IFNULL => {}
            Instruction::IFNONNULL => {}
        }

        Ok(())
    }

    pub fn visit_ldc<C>(&mut self, constant_object: C)
    where
        C: Into<ConstantObject>,
    {
        let constant_object = constant_object.into();
        let constant_index = self
            .symbol_table
            .borrow_mut()
            .add_constant_object(&constant_object);

        match constant_object {
            ConstantObject::Long(_) | ConstantObject::Double(_) => {
                self.put_opcode(Opcode::LDC2_W);
                self.code_byte_vec.put_be(constant_index);
            }
            _ => {
                if constant_index > u8::MAX as u16 {
                    self.put_opcode(Opcode::LDC_W);
                    self.code_byte_vec.put_be(constant_index);
                } else {
                    self.put_opcode(Opcode::LDC);
                    self.code_byte_vec.put_be(constant_index as u8);
                }
            }
        }

        self.inc_stack();
    }

    pub fn visit_return(&mut self, return_opcode: Opcode) -> KapiResult<()> {
        self.put_opcode(return_opcode);

        if return_opcode != Opcode::RETURN {
            self.dec_stack();
        }

        Ok(())
    }

    pub(crate) fn visit_jmp(&mut self, jmp_opcode: Opcode, destination_label: &Rc<RefCell<Label>>) {
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

impl MethodVisitor for MethodWriter {
    fn visit_end(&mut self) {
        let Self {
            byte_vec,
            symbol_table,
            code_byte_vec,
            access_flags,
            name_index,
            descriptor_index,
            max_stack,
            max_locals,
            current_stack: _,
            current_locals: _,
            labels,
        } = self;

        let mut byte_vec = byte_vec.borrow_mut();
        let mut symbol_table = symbol_table.borrow_mut();

        if code_byte_vec.len() != 0 {
            // Retrieve label's offset and put back to correct position
            for (start_index, label) in labels {
                let index = *start_index as usize;
                let label = label.borrow();

                code_byte_vec[index + 1..=index + 2]
                    .swap_with_slice(&mut ((label.0 - *start_index) as i16).to_be_bytes());
            }
        }

        // Generate method
        byte_vec.put_be(access_flags.fold_flags());
        byte_vec.put_be(*name_index);
        byte_vec.put_be(*descriptor_index);
        // TODO: Remove attribute_len hardcode
        let mut attribute_len = 0u16;

        if code_byte_vec.len() != 0 {
            attribute_len += 1;
        }

        byte_vec.put_be(attribute_len);

        // If code_byte_vec is empty, do not emit Code attribute for the method
        if code_byte_vec.len() != 0 {
            let attribute_name_index = symbol_table.add_utf8(CODE);
            let code_len = code_byte_vec.len();
            let attribute_len = 12 + code_len; // TODO

            byte_vec.put_be(attribute_name_index); // attribute_name_index
            byte_vec.put_be(attribute_len as u32); // attribute_length
            byte_vec.put_be(*max_stack as u16); // max_stack
            byte_vec.put_be(*max_locals as u16); // max_locals
            byte_vec.put_be(code_len as u32); // code_length
            byte_vec.append(code_byte_vec); // code[code_length]
                                            // TODO: Implement exceptions
            byte_vec.put_be(0u16);
            // TODO_END
            // TODO: Implement attributes
            byte_vec.put_be(0u16);
            // TODO_END
        }
    }
}
