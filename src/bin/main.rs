use ka_pi::asm::node::constant::Constant;
use std::path::PathBuf;

use ka_pi::asm::parse::class::read_class;
use ka_pi::error::KapiResult;

fn main() -> KapiResult<()> {
    // let mut class_writer = ClassWriter::new_class_writer(
    //     JavaVersion::V17,
    //     vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
    //     "Main",
    //     "java/lang/Object",
    //     vec![],
    // );
    //
    // {
    //     let mut field_writer = class_writer.visit_field(
    //         vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
    //         "main",
    //         "Ljava/lang/String;",
    //     )?;
    //
    //     field_writer.visit_constant("String")?;
    //     field_writer.visit_end();
    // }
    //
    // {
    //     let mut method_writer = class_writer.visit_method(
    //         vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
    //         "main",
    //         "([Ljava/lang/String;)V",
    //     )?;
    //
    //     method_writer.visit_return(Opcode::RETURN);
    //     method_writer.visit_end();
    // }
    //
    // {
    //     let mut method_writer =
    //         class_writer.visit_method(vec![MethodAccessFlag::Public], "getMain", "()I")?;
    //
    //     method_writer.visit_ldc(1);
    //     method_writer.visit_return(Opcode::IRETURN);
    //     method_writer.visit_end();
    // }
    //
    // let bytecode = class_writer.visit_end();
    //
    // fs::create_dir_all("./output").unwrap();
    //
    // let mut output = File::create("./output/Main.class").unwrap();
    // output.write_all(&bytecode[..]).unwrap();

    let mut class_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    class_path.push("compiled_source/Main.class");

    let class = read_class(class_path)?;

    println!("{class:#?}");

    let constant_pool = &class.constant_pool;
    let string_constant = constant_pool.get(13);

    if let Some(Constant::String(constant)) = string_constant {
        println!("{:?}", constant.string(constant_pool).unwrap());
    }

    Ok(())
}
