pub mod byte_vec;
mod constants;
mod edge;
mod error;
mod frame;
mod label;
pub mod opcodes;
pub mod utils;

extern crate jni;

fn main() {
    let mut env = utils::jvm::attach_current_thread();
}
