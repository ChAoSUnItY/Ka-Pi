use std::mem;
use std::rc::Rc;

use crate::asm::constants::{ConstantDynamic, ConstantObject};
use crate::asm::{symbol, Handle};
use crate::asm::types::Type;
use crate::error::{KapiError, KapiResult};
use crate::utils::PushReturn;

pub(crate) const EXPAND_ASM_INSNS: u8 = 256u16 as u8;

pub trait Reader {
    #[inline]
    fn bytes(&self) -> &Vec<u8>;
    #[inline]
    fn header(&self) -> usize;
    #[inline]
    fn constant_pool_info_offsets(&self) -> &Vec<usize>;
    #[inline]
    fn constant_utf8_values(&self) -> &Vec<Rc<String>>;
    #[inline]
    fn constant_dynamic_values(&self) -> &Vec<Rc<ConstantDynamic>>;
    #[inline]
    fn bootstrap_method_offsets(&self) -> &Vec<usize>;

    #[inline]
    fn put_utf8(&mut self, utf8_string: Rc<String>, entry_index: usize);
    #[inline]
    fn put_constant_dynamic(&mut self, constant_dynamic: Rc<ConstantDynamic>, entry_index: usize);

    fn slice_rev<const SIZE: usize>(&self, offset: usize) -> [u8; SIZE] {
        let mut bytes = [0u8; SIZE];
        bytes.clone_from((&self.bytes()[offset..offset + SIZE]).try_into().unwrap());
        bytes.reverse();
        bytes
    }

    #[inline]
    fn read_u8(&self, offset: usize) -> u8 {
        self.bytes()[offset]
    }

    #[inline]
    fn read_i16(&self, offset: usize) -> i16 {
        i16::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_u16(&self, offset: usize) -> u16 {
        u16::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_i32(&self, offset: usize) -> i32 {
        i32::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_i64(&self, offset: usize) -> i64 {
        i64::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_f32(&self, offset: usize) -> f32 {
        f32::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_f64(&self, offset: usize) -> f64 {
        f64::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_utf8(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        let entry_index = self.read_u16(offset) as usize;

        if entry_index == 0 || offset == 0 {
            Err(KapiError::StateError("Constant pool entry index for UTF8 String is invalid while index or offset should larger than 0"))
        } else {
            self.read_utf8_from_cache(entry_index)
        }
    }

    #[inline]
    fn read_utf8_from_cache(&mut self, entry_index: usize) -> KapiResult<Rc<String>> {
        if let Some(entry) = self.constant_utf8_values().get(entry_index) {
            Ok(entry.clone())
        } else {
            let info_offset = self.constant_pool_info_offsets()[entry_index];
            let len = self.read_u16(info_offset) as usize;
            let string = Rc::new(self.read_utf8_from_bytes(info_offset + 2, len)?);

            self.put_utf8(string.clone(), entry_index);

            Ok(string)
        }
    }

    #[inline]
    fn read_utf8_from_bytes(&self, data_offset: usize, len: usize) -> KapiResult<String> {
        let utf8_bytes = &self.bytes()[data_offset..data_offset + len];

        Ok(String::from_utf8_lossy(utf8_bytes).into())
    }

    #[inline]
    fn read_stringish(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_utf8(self.constant_pool_info_offsets()[self.read_u16(offset) as usize])
    }

    #[inline]
    fn read_class(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    #[inline]
    fn read_module(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    #[inline]
    fn read_package(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    fn read_constant_dynamic(&mut self, entry_index: usize) -> KapiResult<Rc<ConstantDynamic>> {
        let constant_dynamic = self.constant_dynamic_values().get(entry_index);

        if let Some(constant_dynamic) = constant_dynamic {
            Ok(constant_dynamic.clone())
        } else {
            let info_offset = self.constant_pool_info_offsets()[entry_index];
            let name_and_type_info_offset =
                self.constant_pool_info_offsets()[self.read_u16(info_offset + 2) as usize];
            let name = self.read_utf8(name_and_type_info_offset)?;
            let descriptor = self.read_utf8(name_and_type_info_offset + 2)?;
            let mut bootstrap_method_offset =
                self.bootstrap_method_offsets()[self.read_u16(info_offset) as usize];
            let handler_entry_index = self.read_u16(bootstrap_method_offset) as usize;
            let handler = self
                .read_const(handler_entry_index)?;
            let handler = handler
                .as_any()
                .downcast_ref::<Handle>()
                .ok_or(KapiError::ArgError(format!("Expected handler constant dynamic but got incorrect constant dynamic info at constant pool index {}", handler_entry_index)))?;
            let mut bootstrap_method_arguments =
                Vec::with_capacity(self.read_u16(bootstrap_method_offset + 2) as usize);
            bootstrap_method_offset += 4;

            for _ in 0..bootstrap_method_arguments.capacity() {
                bootstrap_method_arguments.push(self.read_const(bootstrap_method_offset)?);
                bootstrap_method_offset += 2;
            }

            let constant_dynamic = Rc::new(ConstantDynamic::new(
                name.to_string(),
                descriptor.to_string(),
                handler.to_owned(),
                bootstrap_method_arguments,
            ));
            
            self.put_constant_dynamic(constant_dynamic.clone(), entry_index);

            Ok(constant_dynamic)
        }
    }

    fn read_const(&mut self, entry_index: usize) -> KapiResult<Box<dyn ConstantObject>> {
        let info_offset = self.constant_pool_info_offsets()[entry_index];

        match self.bytes()[info_offset - 1] {
            symbol::CONSTANT_INTEGER_TAG => Ok(Box::new(self.read_i32(info_offset))),
            symbol::CONSTANT_LONG_TAG => Ok(Box::new(self.read_i64(info_offset))),
            symbol::CONSTANT_FLOAT_TAG => Ok(Box::new(self.read_f32(info_offset))),
            symbol::CONSTANT_DOUBLE_TAG => Ok(Box::new(self.read_f64(info_offset))),
            symbol::CONSTANT_CLASS_TAG => Ok(Box::new(Type::object_type_from_string(self.read_utf8(info_offset)?.to_string()))),
            symbol::CONSTANT_STRING_TAG => Ok(Box::new(self.read_utf8(info_offset)?)),
            symbol::CONSTANT_METHOD_TYPE_TAG => Ok(Box::new(Type::method_type_from_string(self.read_utf8(info_offset)?.to_string()))),
            symbol::CONSTANT_METHOD_HANDLE_TAG => {
                let reference_kind = self.read_u8(info_offset);
                let reference_info_offset = self.constant_pool_info_offsets()[self.read_u16(info_offset + 1) as usize];
                let name_and_type_info_offset = self.constant_pool_info_offsets()[self.read_u16(info_offset + 2) as usize];
                let owner = self.read_class(reference_info_offset)?;
                let descriptor = self.read_class(name_and_type_info_offset)?;
                let is_interface = self.bytes()[reference_info_offset - 1] == symbol::CONSTANT_INTERFACE_METHODREF_TAG;
                
                Ok(Box::new(Handle::new(reference_kind, owner.to_string(), descriptor.to_string(), is_interface.to_string())))
            },
            symbol::CONSTANT_DYNAMIC_TAG => Ok(Box::from(self.read_constant_dynamic(info_offset)?)),
            _ => Err(KapiError::ArgError(format!(
                "Illegal constant object at constant pool index {}",
                entry_index
            ))),
        }
    }
}

pub trait Writer {}

#[derive(Debug, Default)]
pub struct ClassReader {
    header: usize,
    class_file_buffer: Vec<u8>,
    constant_pool_info_offsets: Vec<usize>,
    constant_utf8_values: Vec<Rc<String>>,
    constant_dynamic_values: Vec<Rc<ConstantDynamic>>,
    bootstrap_method_offsets: Vec<usize>,
    max_string_len: usize,
}

impl ClassReader {
    pub(crate) fn new_reader(
        class_file_buffer: Vec<u8>,
        class_file_offset: usize,
        check_version: bool,
    ) -> Self {
        let mut reader = Self::default();

        reader.class_file_buffer = class_file_buffer;

        reader
    }
}

impl Reader for ClassReader {
    #[inline]
    fn bytes(&self) -> &Vec<u8> {
        &self.class_file_buffer
    }

    #[inline]
    fn header(&self) -> usize {
        self.header
    }

    #[inline]
    fn constant_pool_info_offsets(&self) -> &Vec<usize> {
        &self.constant_pool_info_offsets
    }

    #[inline]
    fn constant_utf8_values(&self) -> &Vec<Rc<String>> {
        &self.constant_utf8_values
    }

    #[inline]
    fn constant_dynamic_values(&self) -> &Vec<Rc<ConstantDynamic>> {
        &self.constant_dynamic_values
    }

    #[inline]
    fn bootstrap_method_offsets(&self) -> &Vec<usize> {
        &self.bootstrap_method_offsets
    }

    #[inline]
    fn put_utf8(&mut self, utf8_string: Rc<String>, _: usize) {
        self.constant_utf8_values.push(utf8_string)
    }

    #[inline]
    fn put_constant_dynamic(&mut self, constant_dynamic: Rc<ConstantDynamic>, _: usize) {
        self.constant_dynamic_values.push(constant_dynamic)
    }
}
