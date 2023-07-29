use cfsp::parse::{class_signature, field_signature, method_signature, ParseResult};
use insta::assert_yaml_snapshot;

#[test]
fn test_class_signature_with_generic() -> ParseResult<()> {
    let class_signature =
        class_signature("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")?;

    assert_yaml_snapshot!(class_signature);

    Ok(())
}

#[test]
fn test_field_signature_object() -> ParseResult<()> {
    let field_signature = field_signature("Ljava/lang/Object;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_field_signature_type_variable() -> ParseResult<()> {
    let field_signature = field_signature("TT;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_method_signature_with_generic() -> ParseResult<()> {
    let method_signature = method_signature(
        "<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;",
    )?;

    assert_yaml_snapshot!(method_signature);

    Ok(())
}
