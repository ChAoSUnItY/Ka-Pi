#[cfg(target_feature = "invocation")]

use ka_pi::utils::jvm::{attach_current_thread, get_class_modifiers};
use ka_pi::cp;

fn main() {
    let _ = attach_current_thread();

    let modifiers = get_class_modifiers(cp!(java));

    println!("{:?}", modifiers)
}
