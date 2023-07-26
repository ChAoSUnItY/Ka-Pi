//! `node` module contains all specification-described data structures from
//! [The JVM Specification - Java SE 20 Edition](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf).
//!
//! Some of the structures are made to be user-friendly, at the same time makes it much more
//! straightforward to use.

pub mod access_flag;
pub mod attribute;
pub mod class;
pub mod constant;
mod error;
pub mod field;
pub mod method;
pub mod opcode;
pub mod signature;
