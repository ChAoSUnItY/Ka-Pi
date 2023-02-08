pub mod utils;
mod label;
mod edge;
mod frame;
mod error;
pub mod opcodes;
mod constants;

extern crate jni;

fn main() {
    let mut env = utils::jvm::attach_current_thread();
}
