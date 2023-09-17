use std::fs;

use ka_pi::{
  access_flag::ClassAccessFlag,
  class::{
    ClassVisitor,
    ClassWriter,
    JavaVersion,
  },
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

  writer.visit_end();

  let bytes = writer.to_bytes();

  fs::write("output/Main.class", bytes)
    .expect("Unexpected error while writing class file bytecode");
}
