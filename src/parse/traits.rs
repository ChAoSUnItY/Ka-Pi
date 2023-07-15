use crate::node::constant::{ConstantInfo, ConstantPool};
use crate::node::Node;
use nom::ToUsize;

pub(crate) trait Append {
    type Item;

    fn push(&mut self, item: Self::Item);
}

impl Append for ConstantPool {
    type Item = Node<ConstantInfo>;

    fn push(&mut self, item: Self::Item) {
        self.add(item);
    }
}

impl<T> Append for Vec<T> {
    type Item = T;

    fn push(&mut self, item: Self::Item) {
        self.push(item);
    }
}

impl<T> Append for Node<T>
where
    T: Append + Default,
{
    type Item = <T as Append>::Item;

    fn push(&mut self, item: Self::Item) {
        self.1.push(item)
    }
}

pub(crate) trait LengthConstraint {
    fn constraint(got: usize) -> usize;
}

impl LengthConstraint for ConstantPool {
    fn constraint(got: usize) -> usize {
        got - 1
    }
}

impl<T> LengthConstraint for Vec<T> {
    fn constraint(got: usize) -> usize {
        got
    }
}

// External trait impls

impl<T> ToUsize for Node<T>
where
    T: ToUsize,
{
    fn to_usize(&self) -> usize {
        self.1.to_usize()
    }
}
