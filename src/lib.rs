//! # Rasm
//! ### A JVM Bytecode Manipulation Framework inspired by ASM.
//! 
pub mod byte_vec;
mod constants;
mod edge;
mod error;
mod frame;
mod label;
pub mod opcodes;
mod symbol;
pub mod utils;
pub mod types;
pub mod macros;

extern crate jni;
