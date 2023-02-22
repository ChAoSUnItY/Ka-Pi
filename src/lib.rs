//! # Ka-Pi
//! ### A JVM Bytecode Manipulation Framework inspired by ASM.
//!
#[cfg_attr(not(feature = "reflection"), no_std)] // This would very likely break the whole project, we'll enable this after first stable version 

#[allow(unused)]
pub mod asm;
#[cfg(feature = "reflection")]
pub mod reflection;

pub mod error;
mod utils;
