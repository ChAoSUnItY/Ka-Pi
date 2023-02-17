//! # Ka-Pi
//! ### A JVM Bytecode Manipulation Framework inspired by ASM.
//!

extern crate jni;

#[cfg(feature = "interop")]
pub use class::{RefClass, RefMethod};


#[cfg(feature = "interop")]
pub mod class;
#[cfg(feature = "interop")]
pub mod jvm;
pub mod asm;

pub mod error;
mod utils;
