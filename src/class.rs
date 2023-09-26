use std::{
  cell::RefCell,
  rc::Rc,
};

use crate::{
  access_flag::{
    ClassAccessFlag,
    MethodAccessFlag,
  },
  attrs,
  byte_vec::{
    ByteVec,
    ByteVector,
    SizeComputable,
    ToBytes,
  },
  method::{
    MethodVisitor,
    MethodWriter,
  },
  symbol::SymbolTable,
};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum JavaVersion {
  V1_1,
  V1_2,
  V1_3,
  V1_4,
  V1_5,
  V1_6,
  V1_7,
  V1_8,
  V9,
  V10,
  V11,
  V12,
  V13,
  V14,
  V15,
  V16,
  V17,
  V18,
  V19,
  V20,
  V21,
  Custom { minor: u16, major: u16 },
}

impl JavaVersion {
  pub fn version(&self) -> u32 {
    match self {
      Self::V1_1 => 3 << 16 | 45,
      Self::V1_2 => 0 << 16 | 46,
      Self::V1_3 => 0 << 16 | 47,
      Self::V1_4 => 0 << 16 | 48,
      Self::V1_5 => 0 << 16 | 49,
      Self::V1_6 => 0 << 16 | 50,
      Self::V1_7 => 0 << 16 | 51,
      Self::V1_8 => 0 << 16 | 52,
      Self::V9 => 0 << 16 | 53,
      Self::V10 => 0 << 16 | 54,
      Self::V11 => 0 << 16 | 55,
      Self::V12 => 0 << 16 | 56,
      Self::V13 => 0 << 16 | 57,
      Self::V14 => 0 << 16 | 58,
      Self::V15 => 0 << 16 | 59,
      Self::V16 => 0 << 16 | 60,
      Self::V17 => 0 << 16 | 61,
      Self::V18 => 0 << 16 | 62,
      Self::V19 => 0 << 16 | 63,
      Self::V20 => 0 << 16 | 64,
      Self::V21 => 0 << 16 | 65,
      JavaVersion::Custom { minor, major } => {
        let minor = minor.to_be_bytes();
        let major = major.to_be_bytes();

        u32::from_be_bytes([minor[0], minor[1], major[0], major[1]])
      }
    }
  }
}

impl Default for JavaVersion {
  fn default() -> Self {
    Self::V17
  }
}

pub trait ClassVisitor {
  fn inner(&mut self) -> Option<&mut dyn ClassVisitor> {
    None
  }

  fn visit(
    &mut self,
    version: JavaVersion,
    access: ClassAccessFlag,
    name: &str,
    signature: Option<&str>,
    super_name: &str,
    interfaces: &[&str],
  ) {
    if let Some(inner) = self.inner() {
      inner.visit(version, access, name, signature, super_name, interfaces);
    }
  }

  fn visit_method(
    &mut self,
    access: MethodAccessFlag,
    name: &str,
    descriptor: &str,
    signature: Option<&str>,
    exceptions: &[&str],
  ) -> Option<&mut dyn MethodVisitor> {
    if let Some(inner) = self.inner() {
      inner.visit_method(access, name, descriptor, signature, exceptions)
    } else {
      None
    }
  }

  fn visit_deprecated(&mut self) {
    if let Some(inner) = self.inner() {
      inner.visit_deprecated();
    }
  }

  fn visit_source(&mut self, source_file: &str) {
    if let Some(inner) = self.inner() {
      inner.visit_source(source_file);
    }
  }

  fn visit_debug_extension(&mut self, debug_extension: &str) {
    if let Some(inner) = self.inner() {
      inner.visit_debug_extension(debug_extension);
    }
  }

  fn visit_nest_host(&mut self, nest_host: &str) {
    if let Some(inner) = self.inner() {
      inner.visit_nest_host(nest_host);
    }
  }

  fn visit_outer_class(&mut self, class: &str, name: Option<&str>, descriptor: Option<&str>) {
    if let Some(inner) = self.inner() {
      inner.visit_outer_class(class, name, descriptor);
    }
  }

  fn visit_nest_member(&mut self, nest_member: &str) {
    if let Some(inner) = self.inner() {
      inner.visit_nest_member(nest_member);
    }
  }

  fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct ClassWriter {
  version: JavaVersion,
  access: ClassAccessFlag,
  constant_pool: Rc<RefCell<SymbolTable>>,
  this_class: Option<u16>,
  signature: Option<u16>,
  super_class: Option<u16>,
  interfaces: Vec<u16>,
  // fields: Vec<_>,
  methods: Vec<MethodWriter>,
  // Attribute SourceFile
  source: Option<u16>,
  // Attribute SourceDebugExtension
  debug_extension: Option<Vec<u8>>,
  // Attribute NestHost
  nest_host: Option<u16>,
  // Ka-Pi Specified
  deprecated: bool,
  // Attribute EnclosingMethod
  enclosing_class: Option<u16>,
  enclosing_method: Option<u16>,
  // Attribute NestMember
  nest_members: Option<ByteVec>,
}

impl ClassWriter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let size = self.compute_size();
    // We avoid additional reallocation by precomputing the
    // class file size based on spec
    let mut vec = ByteVec::with_capacity(size);

    self.put_bytes(&mut vec);

    vec
  }
}

impl ClassVisitor for ClassWriter {
  fn visit(
    &mut self,
    version: JavaVersion,
    access: ClassAccessFlag,
    name: &str,
    signature: Option<&str>,
    super_name: &str,
    interfaces: &[&str],
  ) {
    let mut cp = self.constant_pool.borrow_mut();

    self.version = version;
    self.access = access;
    self.this_class = Some(cp.put_class(name));

    if let Some(signature) = signature {
      cp.put_utf8(attrs::SIGNATURE);
      self.signature = Some(cp.put_class(signature));
    }

    self.super_class = Some(cp.put_class(super_name));
    self.interfaces = interfaces
      .into_iter()
      .map(|interface| cp.put_class(interface))
      .collect()
  }

  fn visit_method(
    &mut self,
    access: MethodAccessFlag,
    name: &str,
    descriptor: &str,
    signature: Option<&str>,
    exceptions: &[&str],
  ) -> Option<&mut dyn MethodVisitor> {
    let mw = MethodWriter::new(
      self.constant_pool.clone(),
      access,
      name,
      descriptor,
      signature,
      exceptions,
    );

    self.methods.push(mw);
    self
      .methods
      .last_mut()
      .map(|mw| mw as &mut dyn MethodVisitor)
  }

  fn visit_deprecated(&mut self) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::DEPRECATED);
    self.deprecated = true;
  }

  fn visit_source(&mut self, source_file: &str) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::SOURCE_FILE);
    self.source = Some(cp.put_utf8(source_file));
  }

  fn visit_debug_extension(&mut self, debug_extension: &str) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::SOURCE_DEBUG_EXTENSION);
    self.debug_extension = Some(cesu8::to_java_cesu8(debug_extension).to_vec());
  }

  fn visit_nest_host(&mut self, nest_host: &str) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::NEST_HOST);
    self.nest_host = Some(cp.put_class(nest_host));
  }

  fn visit_outer_class(&mut self, class: &str, name: Option<&str>, descriptor: Option<&str>) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::ENCLOSING_METHOD);
    self.enclosing_class = Some(cp.put_class(class));

    match (name, descriptor) {
      (Some(name), Some(descriptor)) => {
        self.enclosing_method = Some(cp.put_name_and_type(name, descriptor));
      }
      _ => {}
    }
  }

  fn visit_nest_member(&mut self, nest_member: &str) {
    let mut cp = self.constant_pool.borrow_mut();

    if let Some(nest_members) = &mut self.nest_members {
      nest_members.push_u16(cp.put_class(nest_member));
    } else {
      cp.put_utf8(attrs::NEST_MEMBERS);

      let mut nest_members = ByteVec::with_capacity(2);

      nest_members.push_u16(cp.put_class(nest_member));

      self.nest_members = Some(nest_members);
    }
  }
}

impl ToBytes for ClassWriter {
  fn put_bytes(&self, vec: &mut ByteVec) {
    let cp = self.constant_pool.borrow();

    vec.push_u32(0xCAFEBABE).push_u32(self.version.version());

    cp.put_bytes(vec);

    vec
      .push_u16(self.access.bits())
      .push_u16(
        self
          .this_class
          .expect("This class is unset, probably missing `visit` call?"),
      )
      .push_u16(
        self
          .super_class
          .expect("Super class is unset, probably missing `visit` call?"),
      )
      .push_u16(self.interfaces.len() as u16);

    for interface in &self.interfaces {
      vec.push_u16(*interface);
    }

    // TODO: Field
    vec.push_u16(0);
    // TODO: Method
    vec.push_u16(self.methods.len() as u16);

    for mw in &self.methods {
      mw.put_bytes(vec);
    }

    // TODO: Attribute
    vec.push_u16(self.attributes_count() as u16);

    if let Some(signature) = self.signature {
      vec
        .push_u16(cp.get_utf8(attrs::SIGNATURE).unwrap())
        .push_u32(2)
        .push_u16(signature);
    }

    if self.deprecated {
      vec
        .push_u16(cp.get_utf8(attrs::DEPRECATED).unwrap())
        .push_u32(0);
    }

    if let Some(source) = self.source {
      vec
        .push_u16(cp.get_utf8(attrs::SOURCE_FILE).unwrap())
        .push_u32(2)
        .push_u16(source);
    }

    if let Some(debug_extension) = &self.debug_extension {
      vec
        .push_u16(cp.get_utf8(attrs::SOURCE_DEBUG_EXTENSION).unwrap())
        .push_u32(debug_extension.len() as u32)
        .push_u8s(&debug_extension);
    }

    if let Some(nest_host) = self.nest_host {
      vec
        .push_u16(cp.get_utf8(attrs::NEST_HOST).unwrap())
        .push_u32(2)
        .push_u16(nest_host);
    }

    if let Some(enclosing_class) = self.enclosing_class {
      vec
        .push_u16(cp.get_utf8(attrs::ENCLOSING_METHOD).unwrap())
        .push_u32(4)
        .push_u16(enclosing_class)
        .push_u16(self.enclosing_method.unwrap_or_default());
    }

    if let Some(nest_members) = &self.nest_members {
      vec
        .push_u16(cp.get_utf8(attrs::NEST_MEMBERS).unwrap())
        .push_u32((nest_members.len() + 2) as u32)
        .push_u16((nest_members.len() / 2) as u16)
        .extend(nest_members);
    }
  }
}

impl SizeComputable for ClassWriter {
  fn compute_size(&self) -> usize {
    let mut size = 24 + 2 * self.interfaces.len();
    // TODO: Fields
    // TODO: Methods
    // TODO: Attributes
    if self.signature.is_some() {
      size += 8;
    }

    if self.deprecated {
      size += 6;
    }

    if self.source.is_some() {
      size += 8;
    }

    if let Some(debug_extension) = &self.debug_extension {
      size += 6 + debug_extension.len();
    }

    if self.nest_host.is_some() {
      size += 8;
    }

    if self.enclosing_class.is_some() {
      size += 10;
    }

    if let Some(nest_members) = &self.nest_members {
      size += 8 + nest_members.len();
    }

    size
  }

  fn attributes_count(&self) -> usize {
    let mut count = 0;

    if self.signature.is_some() {
      count += 1;
    }

    if self.deprecated {
      count += 1;
    }

    if self.source.is_some() {
      count += 1;
    }

    if self.debug_extension.is_some() {
      count += 1;
    }

    if self.nest_host.is_some() {
      count += 1;
    }

    if self.enclosing_class.is_some() {
      count += 1;
    }

    if self.nest_members.is_some() {
      count += 1;
    }

    count
  }
}
