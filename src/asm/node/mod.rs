pub mod signature;
pub mod class;
pub mod constant;
pub mod attribute;
pub mod access_flag;
pub mod opcode;
pub mod handle;
pub mod field;
pub mod method;

trait Node<'a, T> {
    fn parent(&mut self) -> Option<&'a mut T>;
}


