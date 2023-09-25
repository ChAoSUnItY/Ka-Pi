use indexmap::IndexMap;

use crate::byte_vec::{
  ByteVec,
  ByteVector,
  ToBytes,
};

#[repr(u8)]
#[derive(Debug)]
pub(crate) enum ConstantTag {
  Utf8 = 1,
  Integer = 3,
  Float = 4,
  Long = 5,
  Double = 6,
  Class = 7,
  String = 8,
  FieldRef = 9,
  MethodRef = 10,
  InterfaceMethodRef = 11,
  NameAndType = 12,
  MethodHandle = 15,
  MethodType = 16,
  Dynamic = 17,
  InvokeDynamic = 18,
  Module = 19,
  Package = 20,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Constant {
  Utf8(String),
  Integer(i32),
  Float([u8; 4]),
  Long(i64),
  Double([u8; 8]),
  Class(u16),
  String(u16),
  FieldRef(u16, u16),
  MethodRef(u16, u16),
  InterfaceMethodRef(u16, u16),
  NameAndType(u16, u16),
  MethodHandle(u8, u16),
  MethodType(u16),
  Dynamic(u16, u16),
  InvokeDynamic(u16, u16),
  Module(u16),
  Package(u16),
}

impl Constant {
  pub(crate) const fn tag(&self) -> ConstantTag {
    match self {
      Constant::Utf8(..) => ConstantTag::Utf8,
      Constant::Integer(..) => ConstantTag::Integer,
      Constant::Float(..) => ConstantTag::Float,
      Constant::Long(..) => ConstantTag::Long,
      Constant::Double(..) => ConstantTag::Double,
      Constant::Class(..) => ConstantTag::Class,
      Constant::String(..) => ConstantTag::String,
      Constant::FieldRef(..) => ConstantTag::FieldRef,
      Constant::MethodRef(..) => ConstantTag::MethodRef,
      Constant::InterfaceMethodRef(..) => ConstantTag::InterfaceMethodRef,
      Constant::NameAndType(..) => ConstantTag::NameAndType,
      Constant::MethodHandle(..) => ConstantTag::MethodHandle,
      Constant::MethodType(..) => ConstantTag::MethodType,
      Constant::Dynamic(..) => ConstantTag::Dynamic,
      Constant::InvokeDynamic(..) => ConstantTag::InvokeDynamic,
      Constant::Module(..) => ConstantTag::Module,
      Constant::Package(..) => ConstantTag::Package,
    }
  }

  pub(crate) const fn size(&self) -> u16 {
    match self {
      Constant::Long(..) | Constant::Double(..) => 2,
      _ => 1,
    }
  }
}

impl ToBytes for Constant {
  fn put_bytes(&self, vec: &mut ByteVec) {
    vec.push_u8(self.tag() as u8);

    match self {
      Constant::Utf8(string) => {
        let bytes = cesu8::to_java_cesu8(string);

        vec.push_u16(bytes.len() as u16);
        vec.push_u8s(&bytes);
      }
      Constant::Integer(val) => {
        vec.push_u8s(&val.to_be_bytes());
      }
      Constant::Float(val) => {
        vec.push_u8s(val);
      }
      Constant::Long(val) => {
        vec.push_u8s(&val.to_be_bytes());
      }
      Constant::Double(val) => {
        vec.push_u8s(val);
      }
      Constant::Class(index) => {
        vec.push_u16(*index);
      }
      Constant::String(index) => {
        vec.push_u16(*index);
      }
      _ => todo!(),
    }
  }
}

#[derive(Debug)]
pub(crate) struct SymbolTable {
  pool: IndexMap<Constant, u16>,
  index: u16,
}

impl SymbolTable {
  fn put(&mut self, constant: Constant) -> u16 {
    if let Some(index) = self.pool.get(&constant) {
      *index
    } else {
      let index = self.index;
      self.index += constant.size();
      self.pool.insert(constant, index);
      index
    }
  }

  pub(crate) fn put_utf8<T>(&mut self, utf8: T) -> u16
  where
    T: Into<String>,
  {
    self.put(Constant::Utf8(utf8.into()))
  }

  pub(crate) fn put_integer(&mut self, integer: i32) -> u16 {
    self.put(Constant::Integer(integer))
  }

  pub(crate) fn put_float(&mut self, float: f32) -> u16 {
    self.put(Constant::Float(float.to_be_bytes()))
  }

  pub(crate) fn put_long(&mut self, long: i64) -> u16 {
    self.put(Constant::Long(long))
  }

  pub(crate) fn put_double(&mut self, double: f64) -> u16 {
    self.put(Constant::Double(double.to_be_bytes()))
  }

  pub(crate) fn put_class(&mut self, class_name: &str) -> u16 {
    let utf8 = self.put_utf8(class_name);

    self.put(Constant::Class(utf8))
  }

  pub(crate) fn put_string(&mut self, string: &str) -> u16 {
    let utf8 = self.put_utf8(string);

    self.put(Constant::String(utf8))
  }

  pub(crate) fn put_field_ref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
    let class = self.put_utf8(class);
    let name_and_type = self.put_name_and_type(name, descriptor);

    self.put(Constant::FieldRef(class, name_and_type))
  }

  pub(crate) fn put_method_ref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
    let class = self.put_utf8(class);
    let name_and_type = self.put_name_and_type(name, descriptor);

    self.put(Constant::MethodRef(class, name_and_type))
  }

  pub(crate) fn put_interface_method_ref(
    &mut self,
    class: &str,
    name: &str,
    descriptor: &str,
  ) -> u16 {
    let class = self.put_utf8(class);
    let name_and_type = self.put_name_and_type(name, descriptor);

    self.put(Constant::InterfaceMethodRef(class, name_and_type))
  }

  pub(crate) fn put_name_and_type(&mut self, name: &str, descriptor: &str) -> u16 {
    let name = self.put_utf8(name);
    let descriptor = self.put_utf8(descriptor);

    self.put(Constant::NameAndType(name, descriptor))
  }

  pub(crate) fn get_utf8<T>(&self, utf8: T) -> Option<u16>
  where
    T: Into<String>,
  {
    let str = utf8.into();

    self.pool.get(&Constant::Utf8(str)).copied()
  }
}

impl Default for SymbolTable {
  fn default() -> Self {
    Self {
      pool: Default::default(),
      index: 1,
    }
  }
}

impl ToBytes for SymbolTable {
  fn put_bytes(&self, vec: &mut ByteVec) {
    vec.push_u16(self.index);

    for (constant, _) in &self.pool {
      constant.put_bytes(vec);
    }
  }
}
