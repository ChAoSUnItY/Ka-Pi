use insta::assert_yaml_snapshot;

use ka_pi::error::KapiResult;

#[test]
fn test_main() -> KapiResult<()> {
    let class_tree = ka_pi::asm::parse::to_class(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ))?;

    assert_yaml_snapshot!(class_tree);

    Ok(())
}
