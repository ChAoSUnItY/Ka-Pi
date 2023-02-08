use std::sync::{Arc, Once};

use jni::{AttachGuard, InitArgsBuilder, JNIVersion, JavaVM, errors::Result, objects::JClass, signature::{ReturnType, Primitive}};

pub fn jvm() -> &'static Arc<JavaVM> {
    static mut JVM: Option<Arc<JavaVM>> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .build()
            .unwrap_or_else(|e| panic!("{:#?}", e));

        let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

        unsafe {
            JVM = Some(Arc::new(jvm));
        }
    });

    unsafe { JVM.as_ref().unwrap() }
}

pub fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("Failed to attach jvm thread")
}

pub fn get_class<'a>() -> Result<JClass<'a>> {
    attach_current_thread().find_class("java/lang/Class")
}

pub fn get_class_modifiers<S>(class_name: S) -> Result<u32> where S: Into<String> {
    let class = get_class()?;
    let gaurd = attach_current_thread();
    let method_id = gaurd.get_method_id(class, "getModifiers", "()I")?;
    let modifiers = gaurd.call_method_unchecked(gaurd.find_class(class_name.into())?, method_id, ReturnType::Primitive(Primitive::Int), &[])?;

    modifiers.i().map(|m| m as u32)
}
