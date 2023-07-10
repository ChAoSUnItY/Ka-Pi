use insta::assert_yaml_snapshot;
use ka_pi::parse::to_class;

use ka_pi::error::KapiResult;

#[test]
fn test_main() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ))?);

    Ok(())
}

#[test]
fn test_enum() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/Enum.class"
    ))?);

    Ok(())
}

#[test]
fn test_record() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/Record.class"
    ))?);

    Ok(())
}

#[test]
fn test_visible_annotation() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/VisibleAnnotation.class"
    ))?);

    Ok(())
}

#[test]
fn test_invisible_annotation() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/InvisibleAnnotation.class"
    ))?);

    Ok(())
}

#[test]
fn test_annotation_target() -> KapiResult<()> {
    assert_yaml_snapshot!(to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/AnnotationTarget.class"
    ))?);

    Ok(())
}
