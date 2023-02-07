pub mod util;
mod label;

extern crate jni;

fn main() {
    let mut env = util::attach_current_thread();
}
