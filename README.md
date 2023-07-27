# Ka-Pi

![crates.io](https://img.shields.io/crates/v/ka-pi.svg)

### A JVM Bytecode Manipulation Framework inspired by ASM.

[Ka-Pi](https://en.wiktionary.org/wiki/ka-pi), known as coffee 
pronounced in Min Nan, which has multiple usages and an indispensable
place in not only modern society, but also [computer science](https://en.wikipedia.org/wiki/Java_(programming_language)).

### Features

Ka-Pi offers several essential modules relates to JVM ecosystem:

- `classfile_parser` - A general purpose class file parser that transform class file into nodes described in 
[The Java® Virtual Machine Specification Java SE 20 Edition][spec]

### Basic usages

#### Parse class file into structural nodes

To parse a class file using `classfile_parser`, you'll need to enable feature `classfile_parser` first:

```toml
ka_pi = { version = "...", features = ["classfile_parser"] }
```

Then, you'll be able to use class file parser in your own project:

```rust
use std::fs::File;
use classfile_parser::parse::{to_class, ParsingOption};

fn main() {
  let mut file = File::open("Main.class").unwrap();
  let class = to_class(&mut file, ParsingOption::default().parse_attribute()).unwrap();
  
  println!("{:?}", class);
}

```

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

[spec]: https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf
