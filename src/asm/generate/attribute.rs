use crate::asm::generate::bytes::ByteVecGen;
use crate::asm::generate::bytes::{ByteVec, ByteVecImpl};
use crate::asm::generate::symbol::SymbolTable;
use crate::asm::node::attribute::{Attribute, ConstantValue};
use crate::error::KapiResult;

impl ByteVecGen for Attribute {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) -> KapiResult<()> {
        let name_index = symbol_table.add_utf8(self.name());
        byte_vec.put_be(name_index);

        match self {
            Attribute::ConstantValue(ConstantValue {
                constant_value_index,
            }) => {
                byte_vec.put_be(2u32);
                byte_vec.put_be(*constant_value_index);
            }
            _ => {}
        }

        Ok(())
    }
}
