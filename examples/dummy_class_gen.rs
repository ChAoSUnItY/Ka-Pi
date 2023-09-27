use std::fs;

use ka_pi::{
  access_flag::{
    ClassAccessFlag,
    MethodAccessFlag,
  },
  class::{
    ClassVisitor,
    ClassWriter,
    JavaVersion,
  },
  label::Label,
  opcodes,
};

fn main() {
  let mut writer = ClassWriter::new();

  writer.visit(
    JavaVersion::V17,
    ClassAccessFlag::Super | ClassAccessFlag::Public,
    "Main",
    None,
    "java/lang/Object",
    &[],
  );

  writer.visit_source("Main.java");
  writer.visit_debug_extension("Debug Message");

  let mut mw = writer
    .visit_method(
      MethodAccessFlag::Public | MethodAccessFlag::Static,
      "main",
      "([Ljava/lang/String;)V",
      None,
      &[],
    )
    .unwrap();

  mw.visit_code();

  let mut label = Label::default();

  mw.visit_jump_inst(opcodes::GOTO, &mut label);
  mw.visit_label(&mut label);
  mw.visit_inst(opcodes::RETURN);

  writer.visit_end();

  let bytes = writer.to_bytes();

  fs::write("output/Main.class", bytes)
    .expect("Unexpected error while writing class file bytecode");
}
