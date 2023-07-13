//! `node` module contains all specification-described data structures from
//! [The JVM Specification - Java SE 20 Edition](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf).
//!
//! Some of the structures are made to be user-friendly, at the same time makes it much more
//! straightforward to use.

use std::ops::{Deref, DerefMut, Range};

use serde::{Deserialize, Serialize};

pub mod access_flag;
pub mod attribute;
pub mod class;
pub mod constant;
pub mod field;
pub mod method;
pub mod opcode;
pub mod signature;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node<T>(pub Span, pub T);
pub type Span = Range<usize>;

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
