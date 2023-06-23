use crate::asm::generate::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::generate::symbol::SymbolTable;
use crate::asm::generate::ByteVecGen;
use crate::asm::node::attribute::{Attribute, ConstantValue};

impl ByteVecGen for Attribute {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) {
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
    }
}
