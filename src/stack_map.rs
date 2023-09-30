use std::{rc::Rc, cell::RefCell};

use crate::constant::ConstantPool;

const TOP: u8 = 0;
const INTEGER: u8 = 1;
const FLOAT: u8 = 2;
const DOUBLE: u8 = 3;
const LONG: u8 = 4;
const NULL: u8 = 5;
const UNINITIALIZED_THIS: u8 = 6;
const OBJECT: u8 = 7;
const UNINITIALIZED: u8 = 8;

macro_rules! stack_map_gen_err {
    ($($e:expr),+) => {
        panic!("StackMapTable gen error: {}", format!($($e),+))
    };
}

pub enum Type {
  Top,
  Integer,
  Float,
  Double,
  Long,
  Null,
  UninitializedThis,
  Object {
    name: String,
  },
  Uninitialized,
}

impl Type {
  pub const fn tag(&self) -> u8 {
    match self {
        Type::Top => TOP,
        Type::Integer => INTEGER,
        Type::Float => FLOAT,
        Type::Double => DOUBLE,
        Type::Long => LONG,
        Type::Null => NULL,
        Type::UninitializedThis => UNINITIALIZED_THIS,
        Type::Object { .. } => OBJECT,
        Type::Uninitialized => UNINITIALIZED,
    }
  }

  pub const fn is_2_word(&self) -> bool {
    matches!(self, Type::Double | Type::Long)
  }
}

pub struct OpcodeWalker {
  constant_pool: Rc<RefCell<ConstantPool>>
}
