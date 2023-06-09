use insta::assert_yaml_snapshot;

use ka_pi::error::KapiResult;

fn assert_class_file_parse(bytes: &[u8]) -> KapiResult<()> {
    let class_tree = ka_pi::asm::parse::to_class(bytes)?;

    assert_yaml_snapshot!(class_tree);

    Ok(())
}

#[test]
fn test_main() -> KapiResult<()> {
    assert_class_file_parse(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ))
}
