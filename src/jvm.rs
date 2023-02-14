//! JVM invocation module used to init and handle communication between Rust and JVM.
//!
//! To use it, you'll have to enable feature `interop`.
//!
//! Note: Before using any functions, you should call [`PseudoVMState::init_vm()`](PseudoVM::init_vm)
//! first in order to create a JVM.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::{Arc, Once};

use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::strings::JNIString;
use jni::{objects::JClass, AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};

use crate::class::{Method, RefClass};
use crate::error::{IntoKapiResult, KapiResult};
use crate::RefMethod;

pub type RefPseudoVM<'a> = Rc<RefCell<PseudoVM<'a>>>;

pub trait FromObj<'a> {
    fn from_obj(
        vm: RefPseudoVM<'a>,
        obj: &JObject<'a>,
    ) -> KapiResult<Rc<RefCell<Self>>>
    where
        Self: Sized;
}

/// A [`PseudoVM`] represents an intermediate communication bridge between Rust and JVM, which
/// contains several useful functions and a class cache.
pub struct PseudoVM<'a> {
    pub(crate) attach_guard: AttachGuard<'a>,
    /// caches classes to prevent huge cost while retrieving class info from JVM.
    pub class_cache: HashMap<String, RefClass<'a>>
}

impl<'a> PseudoVM<'a> {
    /// Initializes Java Virtual Machine and returns a pseudo VM state struct to represent an intermediate
    /// communication bridge between Rust and JVM.
    pub fn init_vm() -> KapiResult<Rc<RefCell<PseudoVM<'a>>>> {
        let guard = attach_current_thread()?;

        Ok(Rc::new(RefCell::new(PseudoVM {
            attach_guard: guard,
            class_cache: HashMap::new()
        })))
    }
    
    pub fn get_class()
}

impl<'a> Debug for PseudoVM<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PseudoVMState {{ attach_guard: ..., class_cache: ... }}")
    }
}

fn jvm() -> KapiResult<&'static Arc<JavaVM>> {
    static mut INIT_RESULT: Option<KapiResult<Arc<JavaVM>>> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let reflector_loading_result = || -> KapiResult<Arc<JavaVM>> {
            let jvm_args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .option("-Xcheck:jni")
                .build()
                .unwrap_or_else(|e| panic!("{:#?}", e));

            let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

            Ok(Arc::new(jvm))
        };

        unsafe {
            INIT_RESULT = Some(reflector_loading_result());
        }
    });

    unsafe {
        match INIT_RESULT.as_ref().unwrap() {
            Ok(vm) => Ok(vm),
            Err(err) => Err(err.clone()),
        }
    }
}
