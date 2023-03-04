pub mod signature;
pub mod types;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}
