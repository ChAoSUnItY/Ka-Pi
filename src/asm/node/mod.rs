pub mod signature;
pub mod class;
mod utils;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}
