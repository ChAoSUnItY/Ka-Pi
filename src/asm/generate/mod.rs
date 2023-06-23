use crate::asm::generate::byte_vec::ByteVecImpl;
use crate::asm::generate::symbol::SymbolTable;
use crate::error::KapiResult;

pub mod annotation;
pub(crate) mod attribute;
pub mod byte_vec;
pub mod class;
pub mod constant_value;
pub mod field;
pub mod handle;
pub mod label;
pub mod method;
pub mod module;
pub mod opcode;
pub mod record;
pub mod signature;
pub(crate) mod symbol;
pub mod types;

pub(crate) trait ByteVecGen {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) -> KapiResult<()>;
}
