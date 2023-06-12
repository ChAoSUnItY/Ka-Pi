# Ka-Pi

### A JVM Bytecode Manipulation Framework inspired by ASM.

[Ka-Pi](https://en.wiktionary.org/wiki/ka-pi), known as coffee 
pronounced in Min Nan, which has multiple usages and an indispensable
place in not only modern society, but also [computer science](https://en.wikipedia.org/wiki/Java_(programming_language)).

### Features

Ka-Pi offers several essential modules relates to JVM ecosystem:

- `asm`
  - `node` - Bytecode structure definition module, used by most of other modules.
  - `parse` - Bytecode parsing module used to resolve bytecode (or classfile) into structs.
  - `generate` (WIP) - Bytecode generation module used to generate bytecode.

### Basic usages

#### Parse class file

```rust
use ka_pi::asm::parse::read_class;
use ka_pi::error::KapiResult;

fn main() -> KapiResult<()> {
    let class_path = "path/to/class/file";
    let class_tree = read_class(class_path)?;
    
    println!("{:#?}", class_tree);
    
    Ok(())
}
```

### Implementation Status

#### `asm` 
- TODO

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
Released under the [MIT License](LICENSE) and [ASM License](ASM-LICENSE).
