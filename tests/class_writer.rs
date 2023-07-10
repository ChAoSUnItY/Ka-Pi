#![cfg(feature = "generate")]
use insta::assert_yaml_snapshot;

use ka_pi::error::KapiResult;
use ka_pi::generate::class::ClassWriter;
use ka_pi::generate::field::FieldWriter;
use ka_pi::generate::method::MethodWriter;
use ka_pi::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use ka_pi::node::class::JavaVersion;
use ka_pi::parse::to_class;

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

#[test]
fn test_class_writer_append_1_method_1_field() -> KapiResult<()> {
    let mut class_writer = ClassWriter::new(
        JavaVersion::V17,
        vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
        "Main",
        "java/lang/Object",
        vec![],
    );

    let field_writer = FieldWriter::new(
        vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
        "field",
        "Z",
    )?;
    class_writer.append_field(field_writer);

    let method_writer = MethodWriter::new(
        &JavaVersion::V17,
        vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
        "method",
        "()Z",
    )?;
    class_writer.append_method(method_writer);

    let bytes = class_writer.write_output()?;

    assert!(!bytes.is_empty());

    let class = to_class(&bytes)?;

    assert_yaml_snapshot!(class);

    Ok(())
}
