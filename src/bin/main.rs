use std::fs;
use std::fs::File;
use std::io::{Error, Write};

use ka_pi::asm::class::{ClassVisitor, ClassWriter};
use ka_pi::asm::method::MethodVisitor;
use ka_pi::asm::opcodes::{ClassAccessFlag, Instruction, JavaVersion, MethodAccessFlag, Opcode};

fn main() -> Result<(), Error> {
    let mut class_writer = ClassWriter::new_class_writer(
        JavaVersion::V20,
        vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
        "Main",
        "java/lang/Object",
        vec![],
    );

    {
        let mut method_writer = class_writer.visit_method(
            &[MethodAccessFlag::Public, MethodAccessFlag::Static],
            "main",
            "([Ljava/lang/String;)V",
        );
        
        method_writer.visit_return(Opcode::RETURN);
        method_writer.visit_end();
    }

    class_writer.visit_end();

    let bytecode = class_writer.bytecode();

    fs::create_dir_all("./output")?;

    let mut output = File::create("./output/Main.class")?;
    output.write_all(&bytecode[..])?;

    Ok(())
}
