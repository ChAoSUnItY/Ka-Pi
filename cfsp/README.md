# ClassFile Structural Parser (CFSP)

A general usage parser used to parse class file into structural nodes.

This class file parser implementation is support up to Java SE 20 based on 
[The Java® Virtual Machine Specification Java SE 20 Edition][spec].

## About Performance

Currently, classfile_parser's performance is guaranteed to be parsing whole class file in < 1 ms on average-sized class 
file. However, the timing might be varied based on the instruction control flow and attributes. (For example, take a look
at the `~/compiled_source/src/MegaSized.java`, this can produce class file that weighs over 6 MB, which would take 50 ms 
to parse).

The performance currently is not very ideal compare to [classfile-rs](https://github.com/x4e/classfile-rs)'s implementation.

(Note: Though there's possibility that this performance is lead by full implementation.)

[spec]: https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf
