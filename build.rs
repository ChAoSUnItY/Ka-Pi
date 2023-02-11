use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=./lib/src/Reflector.java");
    let _ = std::env::set_current_dir("lib/src")
        .expect("Failed to set current compilation directory to ./lib/src");
    let output = Command::new("javac")
        .arg("Reflector.java")
        .output()
        .expect("Failed to compile");
    
    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
    println!("Reflector.java has been recompiled")
}
