pub mod class;
pub mod field;
pub mod method;
pub mod module;
pub mod signature;

pub trait Visitable<V> {
    fn visit(&mut self, visitor: &mut V);
}
