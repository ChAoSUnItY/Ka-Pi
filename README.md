# Ka-Pi

![crates.io](https://img.shields.io/crates/v/ka-pi.svg)

### A JVM Bytecode Manipulation Framework inspired by ASM.

[Ka-Pi](https://en.wiktionary.org/wiki/ka-pi), known as coffee 
pronounced in Min Nan, which has multiple usages and an indispensable
place in not only modern society, but also [computer science](https://en.wikipedia.org/wiki/Java_(programming_language)).

### Features

Ka-Pi offers several essential modules relates to JVM ecosystem:

- `node` - Bytecode structure definition module, used by most of other modules.
- `visitor` (WIP) - Builtin implementation of visitor pattern based on `node` module.
- `parse` - Bytecode parsing module used to resolve bytecode (or classfile) into structs.
- `generate` (WIP) - Bytecode generation module used to generate bytecode.

### Basic usages

#### Parse class file

```rust
use std::fs;
use ka_pi::parse::{to_class, ParseResult};

fn main() -> ParseResult<()> {
  let class_path = "compiled_source/out/production/compiled_source/Main.class";
  let mut bytes = fs::read(class_path)?;
  let class_tree = read_class(Cursor::new(&mut bytes))?;

  println!("{:#?}", class_tree);

  Ok(())
}
```

### Implementation Status

#### `node` 
All nodes described in [The Java® Virtual Machine Specification Java SE 20 Edition](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf)
are implemented.

#### `visitor`
- [ ] Class visitor
- [ ] Field visitor
- [ ] Method visitor
- [ ] Module visitor
- [ ] Annotation visitor
- [ ] Record visitor
- [x] Signature visitor

#### `parse`
<details>
    <summary> Class structure parsing </summary>

- [x] Magic Number (0xCAFEBABE)
- [x] Constant Pool
  - [x] Utf8
  - [x] Integer
  - [x] Float
  - [x] Long
  - [x] Double
  - [x] Class
  - [x] String
  - [x] Fieldref
  - [x] Methodref
  - [x] InterfaceMethodref
  - [x] NameAndType
  - [x] MethodHandle
  - [x] MethodType
  - [x] InvokeDynamic
- [x] Access Flags (Class)
- [x] This Class
- [x] Super Class
- [x] Interfaces
- [x] Field
  - [x] Access Flags (Field)
  - [x] Name Index
  - [x] Descriptor Index
  - [x] Attributes (See Class#Attributes)
- [x] Method
  - [x] Access Flags (Method)
  - [x] Name Index
  - [x] Descriptor Index
  - [x] Attributes (See Class#Attributes)
- [x] Attributes
  - [x] Attribute Info
    - [x] Critical for JVM
      - [x] ConstantValue
      - [x] Code
      - [x] StackMapTable
      - [x] BootstrapMethods
      - [x] NestHost
      - [x] NestMembers
      - [x] PermittedSubclasses
    - [x] Critical for Java SE
      - [x] Exceptions
      - [x] InnerClasses
      - [x] EnclosingMethod
      - [x] Synthetic
      - [x] Signature
      - [x] Record
      - [x] SourceFile
      - [x] LineNumberTable
      - [x] LocalVariableTable
      - [x] LocalVariableTypeTable
    - [x] Not critical
      - [x] SourceDebugExtension
      - [x] Deprecated
      - [x] RuntimeVisibleAnnotations
      - [x] RuntimeInvisibleAnnotations
      - [x] RuntimeVisibleParameterAnnotations
      - [x] RuntimeInvisibleParameterAnnotations
      - [x] RuntimeVisibleTypeAnnotations
      - [x] RuntimeInvisibleTypeAnnotations
      - [x] AnnotationDefault
      - [x] MethodParameters
      - [x] Module
      - [x] ModulePackages
      - [x] ModuleMainClass
    - [x] Custom Attribute (Not described in specification)
</details>

#### `generate`

- [x] Basic symbol generation
  - [x] Class
  - [x] Field
  - [x] Method
- [ ] Class
  - [ ] Class hierarchy analysis
  - [ ] Accessibility check
- [x] Field
- [ ] Method
  - [ ] Method descriptor type check
  - [ ] Method stack frame analysis
  - [ ] `crate::generate::Instruction` based opcode generate functions

### See also

There are other related jvm projects developed by me may help the production of JVM projects along with Ka-Pi:
- [frape](https://github.com/ChAoSUnItY/frape) - A direct interop bridge between Rust and Java reflection library in low 
  communication cost. (No releases yet.)
- [jars](https://github.com/ChAoSUnItY/jars) - A simple jar extraction library.

### Author

**Kyle Lin (ChAoS-UnItY)**

* [github/ChAoSUnItY](https://github.com/ChAoSUnItY)
* [twitter/ChAoSUnItY](https://twitter.com/ChAoSUnItY_)

### License

Copyright © 2023, [Kyle Lin (ChAoS-UnItY)](https://github.com/ChAoSUnItY).
Released under the [MIT License](LICENSE).
