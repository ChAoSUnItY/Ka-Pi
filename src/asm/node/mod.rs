pub mod signature;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}
