use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
  constant::{ConstantPool, ConstantTag},
  opcodes::*,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
  Top,
  Integer,
  Float,
  Double,
  Long,
  Null,
  UninitializedThis,
  Object { name: String },
  Uninitialized,
}

impl Type {
  fn new_dummy_obj() -> Self {
    Self::Object {
      name: "*objectref*".to_string(),
    }
  }

  fn new_obj(name: &str) -> Self {
    Self::Object {
      name: name.to_owned(),
    }
  }

  const fn tag(&self) -> u8 {
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

  const fn is_2_word(&self) -> bool {
    matches!(self, Type::Double | Type::Long)
  }

  fn is_array_type(&self) -> bool {
    if let Type::Object { name } = self {
      name.starts_with("[")
    } else {
      false
    }
  }

  /// Gets the inner type of array type, returns [None] if current type is
  /// not an array type.
  fn get_inner_type(&self) -> Option<Type> {
    match self {
      Type::Object { name } => {
        if name.starts_with('[') {
          let inner_type_internal_name = name.chars().skip(1).collect::<String>();
          let inner_type = match inner_type_internal_name.as_str() {
            "I" => Type::Integer,
            "F" => Type::Float,
            "D" => Type::Double,
            "J" => Type::Long,
            _ => {
              if inner_type_internal_name.starts_with('[') {
                Type::Object {
                  name: inner_type_internal_name,
                }
              } else {
                Type::Object {
                  name: inner_type_internal_name
                    .chars()
                    .skip(1)
                    .take_while(|&c| c != ';')
                    .map(|c| if c == '.' { '/' } else { c })
                    .collect(),
                }
              }
            }
          };

          Some(inner_type)
        } else {
          None
        }
      }
      _ => None,
    }
  }

  fn as_array_type(&self, dimension: u16) -> Type {
    let mut base_type = "[".repeat(dimension as usize);

    match self {
      Type::Integer => base_type.push_str("I"),
      Type::Float => base_type.push_str("F"),
      Type::Double => base_type.push_str("D"),
      Type::Long => base_type.push_str("J"),
      Type::Object { name } => {
        if name.starts_with('[') {
          base_type.push_str(&name);
        } else {
          base_type.push_str(&format!("L{};", name.replace(".", "/")));
        }
      }
      _ => stack_map_gen_err!("invalid array base type {}", self),
    }

    Type::Object { name: base_type }
  }

  pub fn init(capacity: usize) -> Vec<Type> {
    vec![Self::Top; capacity]
  }
}

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Type::Top => f.write_str("*TOP*"),
      Type::Integer => f.write_str("INTEGER"),
      Type::Float => f.write_str("FLOAT"),
      Type::Double => f.write_str("DOUBLE"),
      Type::Long => f.write_str("LONG"),
      Type::Null => f.write_str("NULL"),
      Type::UninitializedThis => f.write_str("*UNINITIALIZED_THIS*"),
      Type::Object { name } => f.write_str("OBJECT_REF"),
      Type::Uninitialized => f.write_str("*UNINITIALIZED*"),
    }
  }
}

macro_rules! opcode_wlk_err {
    ($($e:expr),+) => {
        panic!("Opcode Walker error: {}", format!($($e),+))
    };
}

pub struct OpcodeWalker {
  constant_pool: Rc<RefCell<ConstantPool>>,
  return_type: String,
  stack_types: Vec<Type>,
  local_types: Vec<Type>,
  // Byproducts
  max_stack: u16,
  max_local: u16,
}

impl OpcodeWalker {
  fn new(constant_pool: Rc<RefCell<ConstantPool>>, return_type: String, is_static: bool) -> Self {
    Self {
      constant_pool,
      return_type,
      stack_types: Type::init(4),
      local_types: Type::init(if is_static { 0 } else { 1 }),
      max_stack: 0,
      max_local: if is_static { 0 } else { 1 },
    }
  }

  fn push(&mut self, typ: Type) {
    if typ.is_2_word() {
      self.stack_types.push(typ);
      self.stack_types.push(Type::Top);
    } else {
      self.stack_types.push(typ);
    }

    self.max_stack = self.max_stack.max(self.stack_types.len() as u16);
  }

  fn pop(&mut self, size: u16) {
    for _ in 0..size {
      self.stack_types.pop();
    }
  }

  fn set_local_vars(&mut self, index: u16, typ: Type) {
    self.local_types.reserve(index as usize);

    if typ.is_2_word() {
      self.local_types[index as usize] = typ;
      self.local_types[(index + 1) as usize] = Type::Top;
    } else {
      self.local_types[index as usize] = typ;
    }
  }

  fn get_local_vars(&self, index: u16) -> &Type {
    if let Some(local_var_type) = self.local_types.get(index as usize) {
      local_var_type
    } else {
      opcode_wlk_err!(
        "invalid index {} access to local variables, index out of bound",
        index
      );
    }
  }

  fn opcode(&mut self, pos: usize, code: &[u8]) -> usize {
    let opcode = code[pos];

    match opcode {
      0..=53 => self.opcode_0_53(pos, code, opcode),
      54..=95 => self.opcode_54_95(pos, code, opcode),
      _ => opcode_wlk_err!("opcode out of bound"),
    }
  }

  fn opcode_0_53(&mut self, pos: usize, code: &[u8], opcode: u8) -> usize {
    match opcode {
      NOP => 1,
      ACONST_NULL => {
        self.push(Type::Null);
        1
      }
      ICONST_M1..=ICONST_5 => {
        self.push(Type::Integer);
        1
      }
      LCONST_0..=LCONST_1 => {
        self.push(Type::Long);
        1
      }
      FCONST_0..=FCONST_2 => {
        self.push(Type::Float);
        1
      }
      DCONST_0..=DCONST_1 => {
        self.push(Type::Double);
        1
      }
      BIPUSH..=SIPUSH => {
        self.push(Type::Integer);

        if opcode == SIPUSH {
          3
        } else {
          2
        }
      }
      LDC => {
        self.ldc(code[pos + 1] as u16);
        2
      }
      LDC_W..=LDC2_W => {
        self.ldc(u16::from_be_bytes([code[pos + 1], code[pos + 2]]));
        3
      }
      ILOAD => self.xload(Type::Integer),
      LLOAD => self.xload(Type::Long),
      FLOAD => self.xload(Type::Float),
      DLOAD => self.xload(Type::Double),
      ALOAD => {
        let local_var_type = self.get_local_vars(code[pos + 1] as u16);
        self.push(local_var_type.clone());
        2
      }
      ILOAD_0..=ILOAD_3 => {
        self.push(Type::Integer);
        1
      }
      LLOAD_0..=LLOAD_3 => {
        self.push(Type::Long);
        1
      }
      FLOAD_0..=FLOAD_3 => {
        self.push(Type::Float);
        1
      }
      DLOAD_0..=DLOAD_3 => {
        self.push(Type::Double);
        1
      }
      ALOAD_0..=ALOAD_3 => {
        let local_var_index = opcode - ALOAD_0;
        let local_var_type = self.get_local_vars(local_var_index as u16);
        self.push(local_var_type.clone());
        1
      }
      IALOAD => {
        self.pop(2);
        self.push(Type::Integer);
        1
      }
      LALOAD => {
        self.pop(2);
        self.push(Type::Long);
        1
      }
      FALOAD => {
        self.pop(2);
        self.push(Type::Float);
        1
      }
      DALOAD => {
        self.pop(2);
        self.push(Type::Double);
        1
      }
      AALOAD => {
        let Some(object_type) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to pop an empty stack");
        };
        let Some(inner_type) = object_type.get_inner_type() else {
          opcode_wlk_err!("type {} is not an array type", object_type);
        };

        self.push(inner_type);

        1
      }
      BALOAD..=SALOAD => {
        self.pop(2);
        self.push(Type::Integer);
        1
      }
      _ => unreachable!(),
    }
  }

  fn opcode_54_95(&mut self, pos: usize, code: &[u8], opcode: u8) -> usize {
    match opcode {
      ISTORE => self.xstore(pos, code, Type::Integer),
      LSTORE => self.xstore(pos, code, Type::Long),
      FSTORE => self.xstore(pos, code, Type::Float),
      DSTORE => self.xstore(pos, code, Type::Double),
      ASTORE => {
        let Some(object_type) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to pop an empty stack");
        };
        self.set_local_vars(code[pos + 1] as u16, object_type);
        2
      }
      ISTORE_0..=ISTORE_1 => {
        self.pop(1);
        self.set_local_vars((opcode - ISTORE_0) as u16, Type::Integer);
        1
      }
      LSTORE_0..=LSTORE_3 => {
        self.pop(2);
        self.set_local_vars((opcode - LSTORE_0) as u16, Type::Long);
        1
      }
      FSTORE_0..=FSTORE_3 => {
        self.pop(1);
        self.set_local_vars((opcode - FSTORE_0) as u16, Type::Float);
        1
      }
      DSTORE_0..=DSTORE_3 => {
        self.pop(2);
        self.set_local_vars((opcode - DSTORE_0) as u16, Type::Double);
        1
      }
      ASTORE_0..=ASTORE_3 => {
        let Some(object_type) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to pop an empty stack");
        };
        self.set_local_vars((opcode - ASTORE_0) as u16, object_type);
        1
      }
      IASTORE..=DASTORE => {
        match opcode {
          IASTORE | FASTORE => {
            self.pop(3);
          }
          LASTORE | DASTORE => {
            self.pop(4);
          }
          _ => unreachable!(),
        }
        1
      }
      AASTORE => {
        // TODO: Cast?
        self.pop(3);
        1
      }
      BASTORE..=SASTORE => {
        self.pop(3);
        1
      }
      POP => {
        self.pop(1);
        1
      }
      POP2 => {
        self.pop(2);
        1
      }
      DUP => {
        let Some(typ) = self.stack_types.last() else {
          opcode_wlk_err!("unable to dup last stack item, no items on stack");
        };
        self.push(typ.clone());
        1
      }
      DUP_X1..=DUP_X2 => {
        let delta = (opcode - DUP_X1) + 3;
        let Some(typ) = self.stack_types.last() else {
          opcode_wlk_err!("unable to dup last stack item, no items on stack");
        };
        self.stack_types.insert(self.stack_types.len() - delta as usize, typ.clone());
        1
      }
      DUP2 => {
        let Some(typ1) = self.stack_types.get(self.stack_types.len() - 1).cloned() else {
          opcode_wlk_err!("unable to dup2 last stack item, no items on stack");
        };
        let Some(typ2) = self.stack_types.get(self.stack_types.len() - 2).cloned() else {
          opcode_wlk_err!("unable to dup2 last stack item, no items on stack");
        };
        self.stack_types.push(typ2);
        self.stack_types.push(typ1);
        1
      }
      DUP2_X1 => {
        let Some(typ1) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x1 last stack item, no items on stack");
        };
        let Some(typ2) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x1 last stack item, no items on stack");
        };
        let Some(typ3) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x1 last stack item, no items on stack");
        };
        self.stack_types.push(typ2.clone());
        self.stack_types.push(typ1.clone());
        self.stack_types.push(typ3);
        self.stack_types.push(typ2);
        self.stack_types.push(typ1);
        1
      }
      DUP2_X2 => {
        let Some(typ1) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x2 last stack item, no items on stack");
        };
        let Some(typ2) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x2 last stack item, no items on stack");
        };
        let Some(typ3) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x2 last stack item, no items on stack");
        };
        let Some(typ4) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to dup2_x2 last stack item, no items on stack");
        };
        self.stack_types.push(typ4.clone());
        self.stack_types.push(typ3.clone());
        self.stack_types.push(typ2.clone());
        self.stack_types.push(typ1.clone());
        self.stack_types.push(typ4);
        self.stack_types.push(typ3);
        self.stack_types.push(typ2);
        self.stack_types.push(typ1);
        1
      }
      SWAP => {
        let Some(typ1) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to swap last stack item, no items on stack");
        };
        let Some(typ2) = self.stack_types.pop() else {
          opcode_wlk_err!("unable to swap last stack item, no items on stack");
        };
        self.stack_types.push(typ1);
        self.stack_types.push(typ2);
        1
      }
      _ => unreachable!(),
    }
  }

  fn ldc(&mut self, index: u16) {
    let Some(tag) = self.constant_pool.borrow().get_tag(index) else {
      opcode_wlk_err!("invalid constant pool index {index}");
    };

    match tag {
      ConstantTag::Integer => self.push(Type::Integer),
      ConstantTag::Float => self.push(Type::Float),
      ConstantTag::Long => self.push(Type::Long),
      ConstantTag::Double => self.push(Type::Double),
      ConstantTag::Class => self.push(Type::new_obj("java.lang.Class")),
      ConstantTag::String => self.push(Type::new_obj("java.lang.String")),
      ConstantTag::Dynamic => {
        todo!("Constant Dynamic is not implemented for `ldc`")
      }
      _ => opcode_wlk_err!("invalid constant tag {tag:#?} to load with `ldc`"),
    }
  }

  fn xload(&mut self, typ: Type) -> usize {
    self.push(typ);
    return 2;
  }

  fn xstore(&mut self, pos: usize, code: &[u8], typ: Type) -> usize {
    let index = code[pos + 1];

    self.pop(if typ.is_2_word() { 2 } else { 1 });
    self.set_local_vars(index as u16, typ);

    return 2;
  }
}
