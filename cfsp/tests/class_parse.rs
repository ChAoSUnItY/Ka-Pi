use cfsp::parse::{to_class, ParseResult, ParsingOption};
use insta::assert_yaml_snapshot;
use std::io::Cursor;

#[test]
fn test_main() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/Main.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}

#[test]
fn test_enum() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/Enum.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}

#[test]
fn test_record() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/Record.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}

#[test]
fn test_visible_annotation() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/VisibleAnnotation.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}

#[test]
fn test_invisible_annotation() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/InvisibleAnnotation.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}

#[test]
fn test_annotation_target() -> ParseResult<()> {
    let mut cursor = Cursor::new(include_bytes!(
        "../../compiled_source/out/production/compiled_source/AnnotationTarget.class"
    ));

    assert_yaml_snapshot!(to_class(
        &mut cursor,
        ParsingOption::default().parse_attribute()
    )?);

    Ok(())
}
