#![cfg(feature = "generate")]
use insta::assert_yaml_snapshot;
use ka_pi::generate::signature::{
    ClassSignatureWriter, FieldSignatureWriter, MethodSignatureWriter,
};

#[test]
fn test_class_signature_writer() {
    let mut writer = ClassSignatureWriter::default();

    writer
        .formal_type_parameter(&"T".to_string())
        .class_bound()
        .class_type("java/lang/Object");

    writer.super_class().class_type("java/lang/Object");
    writer.interface().class_type("java/lang/Comparable");

    assert_yaml_snapshot!(writer.to_string());
}

#[test]
fn test_field_signature_writer() {
    let mut writer = FieldSignatureWriter::default();

    writer.field_type().class_type("java/lang/String");

    assert_yaml_snapshot!(writer.to_string());
}

#[test]
fn test_method_signature_writer() {
    let mut writer = MethodSignatureWriter::default();

    writer
        .formal_type_parameter(&"T".to_string())
        .class_bound()
        .class_type("java/lang/Object");

    writer.parameter_type().class_type("java/lang/Object");
    writer.return_type().class_type("java/lang/String");

    assert_yaml_snapshot!(writer.to_string());
}
