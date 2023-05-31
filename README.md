# Ka-Pi

### A JVM Bytecode Manipulation Framework inspired by ASM.

[Ka-Pi](https://en.wiktionary.org/wiki/ka-pi), known as coffee 
pronounced in Min Nan, which has multiple usages and an indispensable
place in not only modern society, but also [computer science](https://en.wikipedia.org/wiki/Java_(programming_language)).

### Features

Ka-Pi offers several essential modules relates to JVM ecosystem:

- `asm` - A bytecode manipulation module used to visit JVM bytecode strucutre and generate class file via visitor pattern.
- ~~`reflect` - A directly interop bridge between Rust and Java reflection library in low communication cost.~~
- - (Update) `reflect` module has been moved to a separate module [`frape`](https://github.com/ChAoSUnItY/frape).
- `parse` - A bytecode parsing module used to resolve bytecode (or classfile) into structs.

### Implementation Status

#### `asm` 
- TODO

#### `parse`
<details>
    <summary> Class structure parsing </summary>

- [x] Magic Number (0xCAFEBABE)
- [x] Constant Pool
- - [x] Utf8
- - [x] Integer
- - [x] Float
- - [x] Long
- - [x] Double
- - [x] Class
- - [x] String
- - [x] Fieldref
- - [x] Methodref
- - [x] InterfaceMethodref
- - [x] NameAndType
- - [x] MethodHandle
- - [x] MethodType
- - [x] InvokeDynamic
- [x] Access Flags (Class)
- [x] This Class
- [x] Super Class
- [x] Interfaces
- [x] Field
- - [x] Access Flags (Field)
- - [x] Name Index
- - [x] Descriptor Index
- - [x] Attributes (See Class#Attributes)
- [x] Method
- - [x] Access Flags (Method)
- - [x] Name Index
- - [x] Descriptor Index
- - [x] Attributes (See Class#Attributes)
- [x] Attributes
- - [x] Attribute Info
- - - [ ] Critical for JVM
- - - - [x] ConstantValue
- - - - [x] Code
- - - - [x] StackMapTable
- - - - [x] BootstrapMethods
- - - - [ ] NestHost
- - - - [ ] NestMembers
- - - - [ ] PermittedSubclasses
- - - [ ] Critical for Java SE
- - - - [ ] Exceptions
- - - - [ ] InnerClasses
- - - - [ ] EnclosingMethod
- - - - [ ] Synthetic
- - - - [ ] Signature
- - - - [ ] Record
- - - - [x] SourceFile
- - - - [x] LineNumberTable
- - - - [ ] LocalVariableTable
- - - - [ ] LocalVariableTypeTable
- - - [ ] Not critical
- - - - [ ] SourceDebugExtension
- - - - [ ] Deprecated
- - - - [ ] RuntimeVisibleAnnotations
- - - - [ ] RuntimeInvisibleAnnotations
- - - - [ ] RuntimeVisibleParameterAnnotations
- - - - [ ] RuntimeInvisibleParameterAnnotations
- - - - [ ] RuntimeVisibleTypeAnnotations
- - - - [ ] RuntimeInvisibleTypeAnnotations
- - - - [ ] AnnotationDefault
- - - - [ ] MethodParameters
- - - - [ ] Module
- - - - [ ] ModulePackages
- - - - [ ] ModuleMainClass
- - - [x] Custom Attribute (Not described in specification)
</details>

### Author

**Kyle Lin (ChAoS-UnItY)**

* [github/ChAoSUnItY](https://github.com/ChAoSUnItY)
* [twitter/ChAoSUnItY](https://twitter.com/ChAoSUnItY_)

### License

Copyright © 2023, [Kyle Lin (ChAoS-UnItY)](https://github.com/ChAoSUnItY).
Released under the [MIT License](LICENSE) and [ASM License](ASM-LICENSE).
