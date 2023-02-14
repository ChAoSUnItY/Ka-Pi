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

use crate::class::{Class, Method, RefClass};
use crate::error::{IntoKapiResult, KapiResult};
use crate::types::canonical_to_descriptor;
use crate::RefMethod;

pub type RefPseudoVM<'a> = Rc<RefCell<PseudoVM<'a>>>;

/// A [`PseudoVM`] represents an intermediate communication bridge between Rust and JVM, which
/// contains several useful functions and a class cache.
pub struct PseudoVM<'a> {
    pub(crate) attach_guard: AttachGuard<'a>,
    /// caches classes to prevent huge cost while retrieving class info from JVM.
    pub class_cache: HashMap<String, RefClass<'a>>,
    pub(crate) class_clazz: JClass<'a>,
}

impl<'a> PseudoVM<'a> {
    /// Initializes Java Virtual Machine and returns a pseudo VM state struct to represent an intermediate
    /// communication bridge between Rust and JVM.
    pub fn init_vm() -> KapiResult<RefPseudoVM<'a>> {
        let guard = jvm()?.attach_current_thread()?;

        Ok(Rc::new(RefCell::new(Self::new(guard)?)))
    }

    pub fn new(attach_guard: AttachGuard<'a>) -> KapiResult<Self> {
        let class_clazz = attach_guard.find_class("java/lang/Class")?;

        Ok(Self {
            attach_guard,
            class_cache: HashMap::new(),
            class_clazz,
        })
    }

    pub fn get_or_init_class<S>(vm: RefPseudoVM<'a>, class_name: S) -> KapiResult<RefClass<'a>>
    where
        S: Into<String>,
    {
        let class_name = class_name.into();
        let class_cache = &mut vm.borrow_mut().class_cache;

        if let Some(class) = class_cache.get(&class_name) {
            Ok(class.clone())
        } else {
            Self::get_class(vm.clone(), class_name)
        }
    }

    pub fn get_class<S>(vm: RefPseudoVM<'a>, class_name: S) -> KapiResult<RefClass<'a>>
    where
        S: Into<String>,
    {
        let class_clazz = vm.clone().borrow().class_clazz;
        let class_name = class_name.into();
        let descriptor = Self::new_string(vm.clone(), canonical_to_descriptor(&class_name))?;
        let class = Self::call_static_method(
            vm.clone(),
            class_clazz,
            "forName",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[descriptor.into()],
        )?
        .l()?;
        
        Self::delete_local_ref(vm.clone(), descriptor.into())?;
        
        let class_ref = Self::new_global_ref(vm.clone(), class)?;
        let is_array =
            Self::call_method(vm.clone(), class, "isArray", "()Z", &[])?.z()?;
        
        Self::delete_local_ref(vm.clone(), class)?;
        
        let component_class = if is_array {
            Some(Self::get_or_init_class(
                vm.clone(),
                &class_name[..class_name.len() - 2],
            )?)
        } else {
            None
        };
        
        Ok(Class::new_class_ref(vm.clone(), class_ref, component_class))
    }

    pub fn call_static_method<S1, S2>(
        vm: RefPseudoVM<'a>,
        class: JClass<'a>,
        name: S1,
        sig: S2,
        args: &[JValue<'a>],
    ) -> KapiResult<JValue<'a>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        vm.borrow()
            .attach_guard
            .call_static_method(class, name.into(), sig.into(), args)
            .into_kapi()
    }

    pub fn call_method<O, S1, S2>(
        vm: RefPseudoVM<'a>,
        object: O,
        name: S1,
        sig: S2,
        args: &[JValue<'a>],
    ) -> KapiResult<JValue<'a>>
    where
        O: Into<JObject<'a>>,
        S1: Into<String>,
        S2: Into<String>,
    {
        vm.borrow()
            .attach_guard
            .call_method(object, name.into(), sig.into(), args)
            .into_kapi()
    }

    pub fn new_global_ref<O>(vm: RefPseudoVM<'a>, object: O) -> KapiResult<GlobalRef>
    where
        O: Into<JObject<'a>>,
    {
        vm.borrow().attach_guard.new_global_ref(object).into_kapi()
    }

    pub fn delete_local_ref(vm: RefPseudoVM<'a>, object: JObject<'a>) -> KapiResult<()> {
        vm.borrow()
            .attach_guard
            .delete_local_ref(object)
            .into_kapi()
    }

    pub fn new_string<S>(vm: RefPseudoVM<'a>, string: S) -> KapiResult<JString<'a>>
    where
        S: Into<String>,
    {
        vm.borrow()
            .attach_guard
            .new_string(string.into())
            .into_kapi()
    }
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
