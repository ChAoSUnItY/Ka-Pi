//! `node` module contains all specification-described data structures from
//! [The JVM Specification - Java SE 20 Edition](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf).
//!
//! Some of the structures are made to be user-friendly, at the same time makes it much more
//! straightforward to use.

use std::ops::{Deref, DerefMut, Range};

use byte_span::BytesSpan;
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
pub struct Node<T>(
    #[cfg_attr(not(feature = "serde_span"), serde(skip))] pub Span,
    pub T,
);
pub type Span = Range<usize>;

pub type Nodes<T> = Node<Vec<Node<T>>>;

impl<T> Node<T> {
    pub fn map<R>(self, mut mapper: impl FnMut(T) -> R) -> Node<R> {
        let Self(span, t) = self;

        Node(span, mapper(t))
    }
}

impl<'fragment> From<BytesSpan<'fragment>> for Node<&'fragment [u8]> {
    fn from(value: BytesSpan<'fragment>) -> Self {
        Node(value.range(), value.fragment)
    }
}

impl<'fragment> Into<BytesSpan<'fragment>> for Node<&'fragment [u8]> {
    fn into(self) -> BytesSpan<'fragment> {
        BytesSpan {
            offset: self.0.start,
            fragment: self.1,
        }
    }
}

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
