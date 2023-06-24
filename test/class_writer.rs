use std::fs::File;
use std::io::Write;
use insta::assert_yaml_snapshot;
use ka_pi::asm::generate::class::ClassWriter;
use ka_pi::asm::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use ka_pi::asm::node::class::JavaVersion;
use ka_pi::asm::parse::to_class;
use ka_pi::error::KapiResult;

#[test]
fn validate_class_writer_output() -> KapiResult<()> {
    let mut class_writer = ClassWriter::new(
        JavaVersion::V17,
        vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
        "Main",
        "java/lang/Object",
        vec![],
    );

    class_writer.write_field(
        vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
        "field",
        "Z",
        |field| Ok(field),
    )?;
    class_writer.write_method(
        vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
        "method",
        "()Z",
        |method| Ok(method),
    )?;

    let bytes = class_writer.write_output()?;

    assert!(!bytes.is_empty());

    let class = to_class(&bytes)?;

    assert_yaml_snapshot!(class);

    Ok(())
}
