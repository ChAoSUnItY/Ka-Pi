pub mod signature;
pub mod class;
pub mod constant;
mod utils;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}
