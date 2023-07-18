use insta::assert_yaml_snapshot;

use ka_pi::error::KapiResult;
use ka_pi::parse::{parse_class_signature, parse_field_signature, parse_method_signature};

#[test]
fn test_class_signature_with_generic() -> KapiResult<()> {
    let class_signature =
        parse_class_signature("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")?;

    assert_yaml_snapshot!(class_signature);

    Ok(())
}

#[test]
fn test_field_signature_object() -> KapiResult<()> {
    let field_signature = parse_field_signature("Ljava/lang/Object;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_field_signature_type_variable() -> KapiResult<()> {
    let field_signature = parse_field_signature("TT;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_method_signature_with_generic() -> KapiResult<()> {
    let method_signature = parse_method_signature(
        "<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;",
    )?;

    assert_yaml_snapshot!(method_signature);

    Ok(())
}
