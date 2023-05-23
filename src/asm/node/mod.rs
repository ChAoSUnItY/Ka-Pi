pub mod access_flag;
pub mod attribute;
pub mod class;
pub mod constant;
pub mod field;
pub mod handle;
pub mod method;
pub mod opcode;
pub mod signature;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}
