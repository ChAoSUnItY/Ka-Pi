use std::cell::{RefCell, RefMut};
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use itertools::Itertools;

use crate::error::{KapiError, KapiResult};
use crate::generate::bytes::ByteVecGen;
use crate::generate::bytes::{ByteVec, ByteVecImpl};
use crate::generate::label::Label;
use crate::generate::opcode::{ConstantObject, Instruction};
use crate::generate::symbol::SymbolTable;
use crate::generate::types::Type;
use crate::node::access_flag::{AccessFlags, MethodAccessFlag};
use crate::node::attribute;
use crate::node::class::JavaVersion;
use crate::node::opcode::Opcode;

pub struct MethodWriter {
    // Class file predefined info
    java_version: JavaVersion,
    // Internal writing buffers
    symbol_table: SymbolTable,
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
    pub fn new<F>(
        java_version: &JavaVersion,
        access_flags: F,
        name: &str,
        descriptor: &str,
    ) -> KapiResult<Self>
    where
        F: IntoIterator<Item = MethodAccessFlag>,
    {
        let mut symbol_table = SymbolTable::default();
        let access_flags = access_flags.into_iter().collect::<Vec<_>>();
        let name_index = symbol_table.add_utf8(name);
        let descriptor_index = symbol_table.add_utf8(descriptor);

        let mut initial_locals = Type::from_method_descriptor(descriptor)?
            .0
            .iter()
            .map(Type::size)
            .sum::<usize>();
        if !access_flags.contains(&MethodAccessFlag::Static) {
            initial_locals += 1;
        }

        Ok(Self {
            java_version: *java_version,
            symbol_table,
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

    fn peek_stack(&self) -> KapiResult<&Type> {
        if let Some(typ) = self.stack_status.back() {
            Ok(typ)
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack, no stack items persist at this moment"
            )))
        }
    }

    fn peek_stack_by_pos(&self, pos: usize) -> KapiResult<&Type> {
        if let Some(typ) = self.stack_status.get(self.stack_status.len() - 1 - pos) {
            Ok(typ)
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack, no stack items persist at this moment"
            )))
        }
    }

    fn peek_stack_multiple<const SIZE: usize>(&self) -> KapiResult<[Type; SIZE]> {
        let mut items = Vec::with_capacity(SIZE);

        for i in 0..SIZE {
            let peek_item = self.peek_stack()?;

            if i == SIZE - 1 && peek_item.size() == 2 {
                // Illegal popping operation
                return Err(KapiError::StateError(format!(
                    "Illegal stack operation, attempt to pop single stack item but part of item with type `{}`",
                    peek_item.descriptor(),
                )));
            }

            if let Some(typ) = self.stack_status.get(self.stack_status.len() - 1 - i) {
                items.push(typ.clone());
            } else {
                return Err(KapiError::StateError(format!(
                    "Illegal access to top stack item, no stack items persist at this moment\
                     Attempts to peek {SIZE} stack items but only {i} stack items exist"
                )));
            }
        }

        Ok(items.try_into().unwrap())
    }

    fn push_stack(&mut self, item_type: Type) -> usize {
        if item_type.size() == 2 {
            self.push_stack_raw(item_type.clone());
        }
        self.push_stack_raw(item_type)
    }

    fn push_stack_raw(&mut self, item_type: Type) -> usize {
        self.stack_status.push_back(item_type);
        self.max_stack = max(self.stack_status.len(), self.max_stack);
        self.stack_status.len() - 1
    }

    fn pop_stack(&mut self) -> KapiResult<Type> {
        if let Some(typ) = self.stack_status.back() {
            if typ.size() == 2 {
                Err(KapiError::StateError(format!(
                    "Illegal stack operation, attempt to pop single stack item but part of item with type `{}`",
                    typ.descriptor(),
                )))
            } else {
                Ok(self.stack_status.pop_back().unwrap())
            }
        } else {
            Err(KapiError::StateError(format!(
                "Illegal access to top stack item, no stack items persist at this moment"
            )))
        }
    }

    fn pop_stack_multiple<const SIZE: usize>(&mut self) -> KapiResult<[Type; SIZE]> {
        let mut items = Vec::with_capacity(SIZE);

        for i in 0..SIZE {
            let peek_item = self.peek_stack()?;

            if i == SIZE - 1 && peek_item.size() == 2 {
                // Illegal popping operation
                return Err(KapiError::StateError(format!(
                    "Illegal stack operation, attempt to pop single stack item but part of item with type `{}`",
                    peek_item.descriptor(),
                )));
            }

            if let Some(typ) = self.stack_status.pop_back() {
                items.push(typ);
            } else {
                return Err(KapiError::StateError(format!(
                    "Illegal access to top stack item, no stack items persist at this moment\
                     Attempts to pop {SIZE} stack items but only {i} stack items exist"
                )));
            }
        }

        Ok(items.try_into().unwrap())
    }

    fn pop_stack_expect(&mut self, expected_item_type: Type) -> KapiResult<Type> {
        let typ = self.peek_stack()?;

        if expected_item_type.implicit_cmp(&typ) {
            Err(KapiError::StateError(format!(
                "Unexpected stack item with type `{}` while expects stack item with type `{}` to be popped",
                typ.to_string(),
                expected_item_type.descriptor()
            )))
        } else {
            Ok(self.pop_stack()?)
        }
    }

    fn pop_stack_decompose_array_type(&mut self) -> KapiResult<Type> {
        let typ = self.pop_stack()?;

        if let Type::Array(inner_type) = typ {
            Ok(*inner_type.clone())
        } else {
            // Revert pop operation
            self.push_stack(typ.clone());

            Err(KapiError::StateError(format!(
                "Unexpected stack item with type `{}` while expects array type on stack to be popped",
                typ.descriptor(),
            )))
        }
    }

    fn pop_stack_object(&mut self) -> KapiResult<()> {
        let typ = self.peek_stack()?;

        if matches!(typ, Type::ObjectRef(_) | Type::Array(_)) {
            self.pop_stack()?;

            Ok(())
        } else {
            Err(KapiError::StateError(format!(
                "Unexpected type `{}` on stack while expects object type on stack to be popped",
                typ.descriptor(),
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
                    expected_local_type.descriptor(),
                    typ.descriptor()
                )))
            }
        } else {
            // Build local stack graph
            let mut error_message = String::with_capacity(self.locals.len() * 4);

            error_message.push_str(format!("Illegal access to local variable at index {}, only {} local variables persist at this moment", index, self.locals.len()).as_str());
            error_message.push_str("Locals\n");

            for (index, typ) in self.locals.iter().sorted_by_key(|(index, _)| **index) {
                error_message.push_str(format!("{:<6}| {}\n", index, typ.descriptor()).as_str());

                if typ.size() == 2 {
                    error_message.push_str(
                        format!("{:<6}| {}\n", format!("({})", index), typ.descriptor()).as_str(),
                    );
                }
            }

            Err(KapiError::StateError(error_message))
        }
    }

    fn load_local(&mut self, index: usize, expected_local_type: Type) -> KapiResult<()> {
        self.push_stack(self.get_local(index, expected_local_type)?);

        Ok(())
    }

    fn store_local(&mut self, index: usize, load_type: Type) -> KapiResult<()> {
        if let Some(exists_type) = self.locals.get(&index) {
            // Attempt to store into exist type, if load_type is not same as exists_type, then return
            // error.
            if exists_type.implicit_cmp(&load_type) {
                Ok(())
            } else {
                // Type mismatch
                Err(KapiError::StateError(format!("Attempt to store local variable with type `{}` to index {} but local variable is already persisted with type `{}`", load_type.descriptor(), index, exists_type.descriptor())))
            }
        } else {
            Ok(())
        }
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

    /// Emit opcode with a [Instruction]. This is useful when exist functions
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
                self.push_stack(Type::Null);
            }
            Instruction::ICONST_M1 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_0 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_1 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_2 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_3 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_4 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::ICONST_5 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Int);
            }
            Instruction::LCONST_0 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Long);
                self.push_stack(Type::Long);
            }
            Instruction::LCONST_1 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Long);
                self.push_stack(Type::Long);
            }
            Instruction::FCONST_0 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Float);
                self.push_stack(Type::Float);
            }
            Instruction::FCONST_1 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Float);
                self.push_stack(Type::Float);
            }
            Instruction::FCONST_2 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Float);
                self.push_stack(Type::Float);
            }
            Instruction::DCONST_0 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Double);
                self.push_stack(Type::Double);
            }
            Instruction::DCONST_1 => {
                self.put_opcode(inst.opcode());
                self.push_stack(Type::Double);
                self.push_stack(Type::Double);
            }
            Instruction::BIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.push_stack(Type::Byte);
            }
            Instruction::SIPUSH(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.push_stack(Type::Short);
            }
            Instruction::LDC(constant) => {
                self.visit_ldc(constant.to_owned())?;
            }
            Instruction::LDC_W(constant) => {
                self.visit_ldc(constant.to_owned())?;
            }
            Instruction::LDC2_W(constant) => {
                self.visit_ldc(constant.to_owned())?;
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
                self.pop_stack_expect(Type::Array(Box::new(Type::Int)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Long)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Long);
            }
            Instruction::FALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Float)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Float);
            }
            Instruction::DALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Double)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Double);
            }
            Instruction::AALOAD => {
                self.put_opcode(inst.opcode());
                let array_inner_type = self.pop_stack_decompose_array_type()?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(array_inner_type);
            }
            Instruction::BALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Byte)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Byte);
            }
            Instruction::CALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Char)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Char);
            }
            Instruction::SALOAD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Short)))?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Short);
            }
            Instruction::ISTORE(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.pop_stack_expect(Type::Int)?;
                self.store_local(*val as usize, Type::Int)?;
            }
            Instruction::LSTORE(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.pop_stack_expect(Type::Long)?;
                self.store_local(*val as usize, Type::Long)?;
            }
            Instruction::FSTORE(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.pop_stack_expect(Type::Float)?;
                self.store_local(*val as usize, Type::Float)?;
            }
            Instruction::DSTORE(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                self.pop_stack_expect(Type::Double)?;
                self.store_local(*val as usize, Type::Double)?;
            }
            Instruction::ASTORE(val) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*val);
                let popped_object_type = self.pop_stack_expect(Type::object_type())?;
                self.store_local(*val as usize, popped_object_type)?;
            }
            Instruction::ISTORE_0 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.store_local(0, Type::Int)?;
            }
            Instruction::ISTORE_1 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.store_local(1, Type::Int)?;
            }
            Instruction::ISTORE_2 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.store_local(2, Type::Int)?;
            }
            Instruction::ISTORE_3 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.store_local(3, Type::Int)?;
            }
            Instruction::LSTORE_0 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.store_local(0, Type::Long)?;
            }
            Instruction::LSTORE_1 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.store_local(1, Type::Long)?;
            }
            Instruction::LSTORE_2 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.store_local(2, Type::Long)?;
            }
            Instruction::LSTORE_3 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.store_local(3, Type::Long)?;
            }
            Instruction::FSTORE_0 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.store_local(0, Type::Float)?;
            }
            Instruction::FSTORE_1 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.store_local(1, Type::Float)?;
            }
            Instruction::FSTORE_2 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.store_local(2, Type::Float)?;
            }
            Instruction::FSTORE_3 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.store_local(3, Type::Float)?;
            }
            Instruction::DSTORE_0 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.store_local(0, Type::Double)?;
            }
            Instruction::DSTORE_1 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.store_local(1, Type::Double)?;
            }
            Instruction::DSTORE_2 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.store_local(2, Type::Double)?;
            }
            Instruction::DSTORE_3 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.store_local(3, Type::Double)?;
            }
            Instruction::ASTORE_0 => {
                self.put_opcode(inst.opcode());
                let popped_object_type = self.pop_stack_expect(Type::object_type())?;
                self.store_local(0, popped_object_type)?;
            }
            Instruction::ASTORE_1 => {
                self.put_opcode(inst.opcode());
                let popped_object_type = self.pop_stack_expect(Type::object_type())?;
                self.store_local(1, popped_object_type)?;
            }
            Instruction::ASTORE_2 => {
                self.put_opcode(inst.opcode());
                let popped_object_type = self.pop_stack_expect(Type::object_type())?;
                self.store_local(2, popped_object_type)?;
            }
            Instruction::ASTORE_3 => {
                self.put_opcode(inst.opcode());
                let popped_object_type = self.pop_stack_expect(Type::object_type())?;
                self.store_local(3, popped_object_type)?;
            }
            Instruction::IASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Int)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::LASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Long)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Long)?;
            }
            Instruction::FASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Float)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Float)?;
            }
            Instruction::DASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Double)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Double)?;
            }
            Instruction::AASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_decompose_array_type()?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::object_type())?;
            }
            Instruction::BASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Byte)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Byte)?;
            }
            Instruction::CASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Char)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Char)?;
            }
            Instruction::SASTORE => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Array(Box::new(Type::Short)))?;
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Short)?;
            }
            Instruction::POP => {
                self.put_opcode(inst.opcode());
                self.pop_stack_multiple::<1>()?;
            }
            Instruction::POP2 => {
                self.put_opcode(inst.opcode());
                self.pop_stack_multiple::<2>()?;
            }
            Instruction::DUP => {
                self.put_opcode(inst.opcode());
                let [value1] = self.peek_stack_multiple::<1>()?;
                self.push_stack_raw(value1);
            }
            Instruction::DUP_X1 => {
                self.put_opcode(inst.opcode());
                let [value1, value2] = self.peek_stack_multiple::<2>()?;
                self.push_stack_raw(value1.clone());
                self.push_stack_raw(value2);
                self.push_stack_raw(value1);
            }
            Instruction::DUP_X2 => {
                self.put_opcode(inst.opcode());
                let [value1, value2, value3] = self.peek_stack_multiple::<3>()?;
                self.push_stack_raw(value1.clone());
                self.push_stack_raw(value3);
                self.push_stack_raw(value2);
                self.push_stack_raw(value1);
            }
            Instruction::DUP2 => {
                self.put_opcode(inst.opcode());
                let [value1, value2] = self.peek_stack_multiple::<2>()?;
                self.push_stack_raw(value2.clone());
                self.push_stack_raw(value1.clone());
                self.push_stack_raw(value2);
                self.push_stack_raw(value1);
            }
            Instruction::DUP2_X1 => {
                self.put_opcode(inst.opcode());
                let [value1, value2, value3] = self.peek_stack_multiple::<3>()?;
                self.push_stack_raw(value2.clone());
                self.push_stack_raw(value1.clone());
                self.push_stack_raw(value3);
                self.push_stack_raw(value2);
                self.push_stack_raw(value1);
            }
            Instruction::DUP2_X2 => {
                self.put_opcode(inst.opcode());
                let [value1, value2, value3, value4] = self.peek_stack_multiple::<4>()?;
                self.push_stack_raw(value2.clone());
                self.push_stack_raw(value1.clone());
                self.push_stack_raw(value3);
                self.push_stack_raw(value4);
                self.push_stack_raw(value2);
                self.push_stack_raw(value1);
            }
            Instruction::SWAP => {
                self.put_opcode(inst.opcode());
                let [value1, value2] = self.pop_stack_multiple::<2>()?;

                if value1.size() == 2 || value2.size() == 2 {
                    return Err(KapiError::StateError(format!("Invalid stack operation, attempts to swap part of stack item with type `{}`", value1.descriptor())));
                }

                self.push_stack_raw(value1);
                self.push_stack_raw(value2);
            }
            Instruction::IADD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LADD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FADD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DADD => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::ISUB => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LSUB => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FSUB => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DSUB => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::IMUL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LMUL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FMUL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DMUL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::IDIV => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LDIV => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FDIV => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DDIV => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::IREM => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LREM => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FREM => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DREM => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::INEG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LNEG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::FNEG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Float);
            }
            Instruction::DNEG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Double);
            }
            Instruction::ISHL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LSHL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::ISHR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LSHR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::IUSHR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::LUSHR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::IAND => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LAND => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::IOR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LOR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::IXOR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Int);
            }
            Instruction::LXOR => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Long);
            }
            Instruction::IINC { index, value } => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*index);
                self.code_byte_vec.put_be(*value);
            }
            Instruction::I2L => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Long);
            }
            Instruction::I2F => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Float);
            }
            Instruction::I2D => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Double);
            }
            Instruction::L2I => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Int);
            }
            Instruction::L2F => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Float);
            }
            Instruction::L2D => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Double);
            }
            Instruction::F2I => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Int);
            }
            Instruction::F2L => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Long);
            }
            Instruction::F2D => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Double);
            }
            Instruction::D2I => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Int);
            }
            Instruction::D2L => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Long);
            }
            Instruction::D2F => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Float);
            }
            Instruction::I2B => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Byte);
            }
            Instruction::I2C => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Char);
            }
            Instruction::I2S => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Int)?;
                self.push_stack(Type::Short);
            }
            Instruction::LCMP => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Long)?;
                self.pop_stack_expect(Type::Long)?;
                self.push_stack(Type::Int);
            }
            Instruction::FCMPL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Int);
            }
            Instruction::FCMPG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Float)?;
                self.pop_stack_expect(Type::Float)?;
                self.push_stack(Type::Int);
            }
            Instruction::DCMPL => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Int);
            }
            Instruction::DCMPG => {
                self.put_opcode(inst.opcode());
                self.pop_stack_expect(Type::Double)?;
                self.pop_stack_expect(Type::Double)?;
                self.push_stack(Type::Int);
            }
            Instruction::IFEQ(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IFNE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IFLT(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IFGE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IFGT(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IFLE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPEQ(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPNE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPLT(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPGE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPGT(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ICMPLE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::Int)?;
                self.pop_stack_expect(Type::Int)?;
            }
            Instruction::IF_ACMPEQ(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::object_type())?;
                self.pop_stack_expect(Type::object_type())?;
            }
            Instruction::IF_ACMPNE(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
                self.pop_stack_expect(Type::object_type())?;
                self.pop_stack_expect(Type::object_type())?;
            }
            Instruction::GOTO(branch_offset) => {
                self.put_opcode(inst.opcode());
                self.code_byte_vec.put_be(*branch_offset);
            }
            Instruction::JSR(_) => {
                return if (self.java_version as u32) < (JavaVersion::V1_7 as u32) {
                    Err(KapiError::StateError(format!(
                        "Deprecated opcode JSR should not be used after Java 7"
                    )))
                } else {
                    Err(KapiError::StateError(format!(
                        "Opcode JSR is not yet implemented"
                    )))
                }
            }
            Instruction::RET(_) => {
                return if (self.java_version as u32) < (JavaVersion::V1_7 as u32) {
                    Err(KapiError::StateError(format!(
                        "Deprecated opcode RET should not be used after Java 7"
                    )))
                } else {
                    Err(KapiError::StateError(format!(
                        "Opcode RET is not yet implemented"
                    )))
                }
            }
            Instruction::TABLESWITCH { .. } => {}
            Instruction::LOOKUPSWITCH { .. } => {}
            Instruction::IRETURN
            | Instruction::LRETURN
            | Instruction::FRETURN
            | Instruction::DRETURN
            | Instruction::ARETURN
            | Instruction::RETURN => {
                self.visit_return(inst.opcode())?;
            }
            Instruction::GETSTATIC(..) => {}
            Instruction::PUTSTATIC(..) => {}
            Instruction::GETFIELD(..) => {}
            Instruction::PUTFIELD(..) => {}
            Instruction::INVOKEVIRTUAL(..) => {}
            Instruction::INVOKESPECIAL(..) => {}
            Instruction::INVOKESTATIC(..) => {}
            Instruction::INVOKEINTERFACE { .. } => {}
            Instruction::INVOKEDYNAMIC(..) => {}
            Instruction::NEW(..) => {}
            Instruction::NEWARRAY(..) => {}
            Instruction::ANEWARRAY(..) => {}
            Instruction::ARRAYLENGTH => {}
            Instruction::ATHROW => {}
            Instruction::CHECKCAST(..) => {}
            Instruction::INSTANCEOF(..) => {}
            Instruction::MONITORENTER => {}
            Instruction::MONITOREXIT => {}
            Instruction::MULTIANEWARRAY { .. } => {}
            Instruction::IFNULL(..) => {}
            Instruction::IFNONNULL(..) => {}
        }

        Ok(())
    }

    pub fn visit_ldc<C>(&mut self, constant_object: C) -> KapiResult<()>
    where
        C: Into<ConstantObject>,
    {
        let constant_object = constant_object.into();
        let constant_index = self.symbol_table.add_constant_object(&constant_object);

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

        self.push_stack(constant_object.constant_type()?);

        Ok(())
    }

    pub fn visit_return(&mut self, return_opcode: Opcode) -> KapiResult<()> {
        self.put_opcode(return_opcode);

        match return_opcode {
            Opcode::RETURN => {}
            Opcode::IRETURN => {
                self.pop_stack_expect(Type::Int)?;
            }
            Opcode::FRETURN => {
                self.pop_stack_expect(Type::Float)?;
            }
            Opcode::LRETURN => {
                self.pop_stack_expect(Type::Long)?;
            }
            Opcode::DRETURN => {
                self.pop_stack_expect(Type::Double)?;
            }
            Opcode::ARETURN => {
                self.pop_stack_object()?;
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

impl ByteVecGen for MethodWriter {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) -> KapiResult<()> {
        let rearrangements = symbol_table.merge(&self.symbol_table)?;
        let name_index = rearrangements
            .get(&self.name_index)
            .unwrap_or(&self.name_index);
        let descriptor_index = rearrangements
            .get(&self.descriptor_index)
            .unwrap_or(&self.descriptor_index);

        let mut code_byte_vec = self.code_byte_vec.clone();
        let has_code_attr = !code_byte_vec.is_empty();

        if has_code_attr {
            // Retrieve label's offset and put back to correct position
            for (start_index, label) in &self.labels {
                let index = *start_index as usize;
                let label = label.borrow();

                code_byte_vec[index + 1..=index + 2].swap_with_slice(
                    &mut ((label.destination_pos - *start_index) as i16).to_be_bytes(),
                );
            }
        }

        // Generate method
        byte_vec.put_be(self.access_flags.fold_flags());
        byte_vec.put_be(*name_index);
        byte_vec.put_be(*descriptor_index);
        // TODO: Remove attribute_len hardcode
        let mut attribute_len = 0u16;

        if has_code_attr {
            attribute_len += 1;
        }

        byte_vec.put_be(attribute_len);

        // If code_byte_vec is empty, do not emit Code attribute for the method
        if has_code_attr {
            let attribute_name_index = symbol_table.add_utf8(attribute::CODE);
            let code_len = code_byte_vec.len();
            let attribute_len = 12 + code_len; // TODO

            byte_vec.put_be(attribute_name_index); // attribute_name_index
            byte_vec.put_be(attribute_len as u32); // attribute_length
            byte_vec.put_be(self.max_stack as u16); // max_stack
            byte_vec.put_be(self.max_locals as u16); // max_locals
            byte_vec.put_be(code_len as u32); // code_length
            byte_vec.append(&mut code_byte_vec); // code[code_length]
                                                 // TODO: Implement exceptions
            byte_vec.put_be(0u16);
            // TODO_END
            // TODO: Implement attributes
            byte_vec.put_be(0u16);
            // TODO_END
        }

        Ok(())
    }
}
