use std::cell::{RefCell, RefMut};
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use itertools::Itertools;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::constants;
use crate::asm::label::Label;
use crate::asm::opcodes::{AccessFlag, ConstantObject, Instruction, MethodAccessFlag, Opcode};
use crate::asm::symbol::SymbolTable;
use crate::asm::types::Type;
use crate::error::{KapiError, KapiResult};

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
    locals: HashMap<usize, Type>,
    stack_status: VecDeque<Type>,
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
            max_locals: initial_locals,
            locals: HashMap::new(),
            stack_status: VecDeque::new(),
            labels: Vec::new(),
        })
    }

    fn inc_stack(&mut self, item_type: Type) -> usize {
        self.stack_status.push_back(item_type);
        self.max_stack = max(self.stack_status.len(), self.max_stack);
        self.stack_status.len() - 1
    }

    fn dec_stack(&mut self, expected_item_type: Type) -> KapiResult<()> {
        if let Some(typ) = self.stack_status.pop_back() {
            if expected_item_type != typ {
                Err(KapiError::StateError(format!(
                    "Unexpected type `{}` on stack while expects type `{}` on stack to be popped",
                    typ.to_string(),
                    expected_item_type.to_string()
                )))
            } else {
                Ok(())
            }
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack, no stack items persist at this moment"
            )))
        }
    }

    fn dec_array_stack(&mut self) -> KapiResult<Type> {
        if let Some(typ) = self.stack_status.pop_back() {
            if let Type::Array(inner_type) = typ {
                Ok(*inner_type.clone())
            } else {
                Err(KapiError::StateError(format!(
                    "Unexpected type `{}` on stack while expects array type on stack to be popped",
                    typ.to_string(),
                )))
            }
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack, no stack items persist at this moment"
            )))
        }
    }

    fn dec_object_stack(&mut self) -> KapiResult<()> {
        if let Some(typ) = self.stack_status.pop_back() {
            if matches!(typ, Type::ObjectRef(_) | Type::Array(_)) {
                Ok(())
            } else {
                Err(KapiError::StateError(format!(
                    "Unexpected type `{}` on stack while expects object type on stack to be popped",
                    typ.to_string(),
                )))
            }
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack, no stack items persist at this moment"
            )))
        }
    }

    fn put_opcode(&mut self, opcode: Opcode) {
        self.code_byte_vec.put_be(opcode as u8);
    }

    fn get_local(&self, index: usize, expected_local_type: Type) -> KapiResult<Type> {
        if let Some(typ) = self.locals.get(&(index as usize)) {
            let typ = typ.clone();

            // Ignore object casting check
            if matches!(typ, Type::ObjectRef(_) if matches!(expected_local_type, Type::ObjectRef(_)))
                || typ == expected_local_type
            {
                Ok(typ)
            } else {
                Err(KapiError::StateError(format!(
                    "Expects local variable at index {} with type `{}` but got type `{}`",
                    index,
                    expected_local_type.to_string(),
                    typ.to_string()
                )))
            }
        } else {
            // Build local stack graph
            let mut error_message = String::with_capacity(self.locals.len() * 4);

            error_message.push_str(format!("Illegal access to local variable at index {}, only {} local variables persist at this moment", index, self.locals.len()).as_str());
            error_message.push_str("Locals\n");

            for (index, typ) in self.locals.iter().sorted_by_key(|(index, _)| **index) {
                error_message.push_str(format!("{:<6}| {}\n", index, typ.to_string()).as_str());

                if matches!(typ, Type::Long | Type::Double) {
                    error_message.push_str(
                        format!("{:<6}| {}\n", format!("({})", index), typ.to_string()).as_str(),
                    );
                }
            }

            Err(KapiError::StateError(error_message))
        }
    }

    fn load_local(&mut self, index: usize, expected_local_type: Type) -> KapiResult<()> {
        self.inc_stack(self.get_local(index, expected_local_type)?);

        Ok(())
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
                self.inc_stack(Type::Null);
            }
            Instruction::ICONST_M1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_2 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_3 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_4 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::ICONST_5 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Int);
            }
            Instruction::LCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Long);
                self.inc_stack(Type::Long);
            }
            Instruction::LCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Long);
                self.inc_stack(Type::Long);
            }
            Instruction::FCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Float);
                self.inc_stack(Type::Float);
            }
            Instruction::FCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Float);
                self.inc_stack(Type::Float);
            }
            Instruction::FCONST_2 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Float);
                self.inc_stack(Type::Float);
            }
            Instruction::DCONST_0 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Double);
                self.inc_stack(Type::Double);
            }
            Instruction::DCONST_1 => {
                self.put_opcode(inst.opcode());
                self.inc_stack(Type::Double);
                self.inc_stack(Type::Double);
            }
            Instruction::BIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack(Type::Byte);
            }
            Instruction::SIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.inc_stack(Type::Short);
            }
            Instruction::LDC(constant) => {
                self.visit_ldc(constant.to_owned());
            }
            Instruction::LDC_W(constant) => {
                self.visit_ldc(constant.to_owned());
            }
            Instruction::LDC2_W(constant) => {
                self.visit_ldc(constant.to_owned());
            }
            Instruction::ILOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.load_local(*val as usize, Type::Int)?;
            }
            Instruction::LLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.load_local(*val as usize, Type::Long)?;
            }
            Instruction::FLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.load_local(*val as usize, Type::Float)?;
            }
            Instruction::DLOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.load_local(*val as usize, Type::Double)?;
            }
            Instruction::ALOAD(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.load_local(*val as usize, Type::object_type())?;
            }
            Instruction::ILOAD_0 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Int)?;
            }
            Instruction::ILOAD_1 => {
                self.put_opcode(inst.opcode());
                self.load_local(1, Type::Int)?;
            }
            Instruction::ILOAD_2 => {
                self.put_opcode(inst.opcode());
                self.load_local(2, Type::Int)?;
            }
            Instruction::ILOAD_3 => {
                self.put_opcode(inst.opcode());
                self.load_local(3, Type::Int)?;
            }
            Instruction::LLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Long)?;
            }
            Instruction::LLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.load_local(1, Type::Long)?;
            }
            Instruction::LLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.load_local(2, Type::Long)?;
            }
            Instruction::LLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.load_local(3, Type::Long)?;
            }
            Instruction::FLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Float)?;
            }
            Instruction::FLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Float)?;
            }
            Instruction::FLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Float)?;
            }
            Instruction::FLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Float)?;
            }
            Instruction::DLOAD_0 => {
                self.put_opcode(inst.opcode());
                self.load_local(0, Type::Double)?;
            }
            Instruction::DLOAD_1 => {
                self.put_opcode(inst.opcode());
                self.load_local(1, Type::Double)?;
            }
            Instruction::DLOAD_2 => {
                self.put_opcode(inst.opcode());
                self.load_local(2, Type::Double)?;
            }
            Instruction::DLOAD_3 => {
                self.put_opcode(inst.opcode());
                self.load_local(3, Type::Double)?;
            }
            Instruction::ALOAD_0 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(0);
                self.load_local(0, Type::object_type())?;
            }
            Instruction::ALOAD_1 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(1);
                self.load_local(0, Type::object_type())?;
            }
            Instruction::ALOAD_2 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(2);
                self.load_local(0, Type::object_type())?;
            }
            Instruction::ALOAD_3 => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(3);
                self.load_local(0, Type::object_type())?;
            }
            Instruction::IALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Int)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Int);
            }
            Instruction::LALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Long)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Long);
            }
            Instruction::FALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Float)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Float);
            }
            Instruction::DALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Double)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Double);
            }
            Instruction::AALOAD => {
                self.put_opcode(inst.opcode());
                let array_inner_type = self.dec_array_stack()?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(array_inner_type);
            }
            Instruction::BALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Byte)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Byte);
            }
            Instruction::CALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Char)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Char);
            }
            Instruction::SALOAD => {
                self.put_opcode(inst.opcode());
                self.dec_stack(Type::Array(Box::new(Type::Short)))?;
                self.dec_stack(Type::Int)?;
                self.inc_stack(Type::Short);
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

        self.inc_stack(constant_object.constant_type());
    }

    pub fn visit_return(&mut self, return_opcode: Opcode) -> KapiResult<()> {
        self.put_opcode(return_opcode);

        match return_opcode {
            Opcode::RETURN => {}
            Opcode::IRETURN => {
                self.dec_stack(Type::Int)?;
            }
            Opcode::FRETURN => {
                self.dec_stack(Type::Float)?;
            }
            Opcode::LRETURN => {
                self.dec_stack(Type::Long)?;
            }
            Opcode::DRETURN => {
                self.dec_stack(Type::Double)?;
            }
            Opcode::ARETURN => {
                self.dec_object_stack()?;
            }
            _ => {
                return Err(KapiError::ArgError(format!(
                    "Invalid return opcode `{:#04x}`",
                    return_opcode as u8
                )));
            }
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
            label.set_offset((self.code_byte_vec.len()) as u32);
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
            locals: _,
            stack_status: _,
            labels,
        } = self;

        let mut byte_vec = byte_vec.borrow_mut();
        let mut symbol_table = symbol_table.borrow_mut();

        if code_byte_vec.len() != 0 {
            // Retrieve label's offset and put back to correct position
            for (start_index, label) in labels {
                let index = *start_index as usize;
                let label = label.borrow();

                code_byte_vec[index + 1..=index + 2].swap_with_slice(
                    &mut ((label.destination_pos - *start_index) as i16).to_be_bytes(),
                );
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
            let attribute_name_index = symbol_table.add_utf8(constants::CODE);
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
