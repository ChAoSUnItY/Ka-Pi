#![cfg(feature = "generate")]

use insta::assert_yaml_snapshot;

use ka_pi::error::KapiResult;
use ka_pi::generate::class::ClassWriter;
use ka_pi::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
use ka_pi::node::class::JavaVersion;
use ka_pi::parse::to_class;

#[test]
fn test_class_writer_append_1_method_1_field() -> KapiResult<()> {
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
    )?;
    class_writer.write_method(
        vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
        "method",
        "()Z",
    )?;

    let bytes = class_writer.write_output()?;

    assert!(!bytes.is_empty());

    let class = to_class(&bytes)?;

    assert_yaml_snapshot!(class);

    Ok(())
}
