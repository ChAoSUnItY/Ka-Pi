pub mod class;
pub mod constant;
pub mod field;
pub mod method;
pub mod module;
pub mod signature;

/// A marker trait marks data structure visitable by visitors.
pub trait Visitable<V> {
    /// Visits the data structure with visitor.
    fn visit(&mut self, visitor: &mut V);
}
