use std::fs;
use std::fs::File;
use std::io::Write;

use ka_pi::asm::class::{ClassVisitor, ClassWriter};
use ka_pi::asm::field::FieldVisitor;
use ka_pi::asm::method::MethodVisitor;
use ka_pi::asm::opcodes::{
    ClassAccessFlag, FieldAccessFlag, JavaVersion, MethodAccessFlag, Opcode,
};
use ka_pi::error::KapiResult;

fn main() -> KapiResult<()> {
    let mut class_writer = ClassWriter::new_class_writer(
        JavaVersion::V17,
        vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
        "Main",
        "java/lang/Object",
        vec![],
    );

    {
        let mut field_writer = class_writer.visit_field(
            vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
            "main",
            "Ljava/lang/String;",
        )?;

        field_writer.visit_constant("String")?;
        field_writer.visit_end();
    }

    {
        let mut method_writer = class_writer.visit_method(
            vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
            "main",
            "([Ljava/lang/String;)V",
        )?;

        method_writer.visit_return(Opcode::RETURN);
        method_writer.visit_end();
    }

    {
        let mut method_writer =
            class_writer.visit_method(vec![MethodAccessFlag::Public], "getMain", "()I")?;

        method_writer.visit_ldc(1);
        method_writer.visit_return(Opcode::IRETURN);
        method_writer.visit_end();
    }

    class_writer.visit_end();

    let bytecode = class_writer.bytecode();

    fs::create_dir_all("./output").unwrap();

    let mut output = File::create("./output/Main.class").unwrap();
    output.write_all(&bytecode[..]).unwrap();

    Ok(())
}
