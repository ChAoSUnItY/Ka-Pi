use crate::error::KapiResult;
use crate::generate::bytes::ByteVecGen;
use crate::generate::bytes::{ByteVec, ByteVecImpl};
use crate::generate::symbol::SymbolTable;
use crate::node::attribute::{Attribute, ConstantValue};

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
