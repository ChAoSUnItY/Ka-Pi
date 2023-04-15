use std::fs;
use std::fs::File;
use std::io::{Error, Write};

use ka_pi::asm::class::{ClassVisitor, ClassWriter};
use ka_pi::asm::opcodes::ClassAccessFlag::{Public, Super};
use ka_pi::asm::opcodes::JavaVersion;

fn main() -> Result<(), Error> {
    let mut bytecode = Vec::new();
    let class_writer = ClassWriter::new_class_writer(
        &mut bytecode,
        JavaVersion::V20,
        &[Super, Public],
        "Main",
        "java/lang/Object",
        &[],
    );

    class_writer.visit_end();

    fs::create_dir_all("./output")?;
    
    let mut output = File::create("./output/Main.class")?;
    output.write_all(&bytecode[..])?;

    Ok(())
}
