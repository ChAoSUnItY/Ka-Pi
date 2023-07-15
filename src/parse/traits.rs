use nom::ToUsize;

use crate::node::Node;

// External trait impls

impl<T> ToUsize for Node<T>
where
    T: ToUsize,
{
    fn to_usize(&self) -> usize {
        self.1.to_usize()
    }
}
