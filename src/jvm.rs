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

use jni::objects::{
    AsJArrayRaw, AutoLocal, GlobalRef, JObject, JObjectArray, JString, JValue, JValueOwned,
};
use jni::strings::{JNIString, JavaStr};
use jni::sys::jsize;
use jni::{objects::JClass, AttachGuard, InitArgsBuilder, JNIVersion, JavaVM};

use crate::class::{Class, RefClass};
use crate::error::{IntoKapiResult, KapiResult};
use crate::asm::types::canonical_to_descriptor;

pub type RefPseudoVM<'a> = Rc<RefCell<PseudoVM<'a>>>;

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

/// A [`PseudoVM`] represents an intermediate communication bridge between Rust and JVM, which
/// contains several useful functions and a class cache.
pub struct PseudoVM<'a> {
    pub(crate) attach_guard: AttachGuard<'a>,
    /// caches classes to prevent huge cost while retrieving class info from JVM.
    pub class_cache: HashMap<String, RefClass<'a>>,
}

impl<'a> PseudoVM<'a> {
    /// Initializes Java Virtual Machine and returns a pseudo VM state struct to represent an intermediate
    /// communication bridge between Rust and JVM.
    pub fn init_vm() -> KapiResult<RefPseudoVM<'a>> {
        let guard = jvm()?.attach_current_thread()?;

        Ok(Rc::new(RefCell::new(Self::new(guard)?)))
    }

    fn new(attach_guard: AttachGuard<'a>) -> KapiResult<Self> {
        Ok(Self {
            attach_guard,
            class_cache: HashMap::new(),
        })
    }

    fn get_cached_class<S>(vm: RefPseudoVM<'a>, class_name: S) -> Option<RefClass<'a>>
    where
        S: Into<String>,
    {
        vm.borrow_mut()
            .class_cache
            .get(&class_name.into())
            .map(|class_ref| class_ref.clone())
    }

    fn cache_class<S>(vm: RefPseudoVM<'a>, class_name: S, class_ref: RefClass<'a>) -> RefClass<'a>
    where
        S: Into<String>,
    {
        vm.borrow_mut()
            .class_cache
            .insert(class_name.into(), class_ref.clone())
            .unwrap_or(class_ref)
    }

    fn get_or_init_class<S>(vm: RefPseudoVM<'a>, class_name: S) -> KapiResult<RefClass<'a>>
    where
        S: Into<String>,
    {
        let class_name = class_name.into();

        if let Some(class) = Self::get_cached_class(vm.clone(), &class_name) {
            Ok(class.clone())
        } else {
            let class = Self::get_class(vm.clone(), class_name.clone())?;
            let class = Self::cache_class(vm.clone(), class_name, class);

            Ok(class)
        }
    }

    pub fn find_class<S>(vm: RefPseudoVM<'a>, name: S) -> KapiResult<JClass<'a>>
    where
        S: Into<JNIString>,
    {
        vm.borrow_mut().attach_guard.find_class(name).into_kapi()
    }

    pub fn get_class<S>(vm: RefPseudoVM<'a>, class_name: S) -> KapiResult<RefClass<'a>>
    where
        S: Into<String>,
    {
        let class_clazz = Self::find_class(vm.clone(), "java/lang/Class")?;
        let class_name = class_name.into();
        let descriptor = Self::new_string(vm.clone(), canonical_to_descriptor(&class_name))?;
        let class = Self::call_static_method(
            vm.clone(),
            &class_clazz,
            "forName",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[(&descriptor).into()],
        )?
        .l()?;

        Self::delete_local_ref(vm.clone(), class_clazz)?;
        Self::delete_local_ref(vm.clone(), descriptor)?;

        let class_ref = Self::new_global_ref(vm.clone(), &class)?;

        let is_array = Self::call_method(vm.clone(), &class, "isArray", "()Z", &[])?.z()?;

        Self::delete_local_ref(vm.clone(), class)?;

        let component_class = if is_array {
            let component_class_name = if class_name.matches("[").count() > 1 {
                // Remove "["
                &class_name[1..]
            } else {
                // Remove "[L" and ";"
                &class_name[2..class_name.len() - 1]
            };

            Some(Self::get_or_init_class(vm.clone(), component_class_name)?)
        } else {
            None
        };

        Ok(Class::new_class_ref(
            vm.clone(),
            class_name,
            class_ref,
            component_class,
        ))
    }

    pub fn auto_local<O>(vm: RefPseudoVM<'a>, obj: O) -> AutoLocal<'a, O>
    where
        O: Into<JObject<'a>>,
    {
        vm.borrow().attach_guard.auto_local(obj)
    }

    pub fn get_string<'other_local: 'obj_ref, 'obj_ref>(
        vm: RefPseudoVM<'a>,
        obj: &'obj_ref JObject<'other_local>,
    ) -> KapiResult<String> {
        vm.borrow_mut()
            .attach_guard
            .get_string(obj.into())
            .map(JavaStr::into)
            .into_kapi()
    }

    pub fn call_static_method<S1, S2>(
        vm: RefPseudoVM<'a>,
        class: &JClass<'a>,
        name: S1,
        sig: S2,
        args: &[JValue],
    ) -> KapiResult<JValueOwned<'a>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        vm.borrow_mut()
            .attach_guard
            .call_static_method(class, name.into(), sig.into(), args)
            .into_kapi()
    }

    pub fn call_method<'local_ref, O, S1, S2>(
        vm: RefPseudoVM<'a>,
        object: O,
        name: S1,
        sig: S2,
        args: &[JValue],
    ) -> KapiResult<JValueOwned<'a>>
    where
        O: AsRef<JObject<'local_ref>>,
        S1: Into<String>,
        S2: Into<String>,
    {
        vm.borrow_mut()
            .attach_guard
            .call_method(object, name.into(), sig.into(), args)
            .into_kapi()
    }

    pub fn new_global_ref<'other_local, O>(vm: RefPseudoVM<'a>, object: O) -> KapiResult<GlobalRef>
    where
        O: AsRef<JObject<'other_local>>,
    {
        vm.borrow().attach_guard.new_global_ref(object).into_kapi()
    }

    pub fn new_local_ref<'other_local, O>(
        vm: RefPseudoVM<'a>,
        global_ref: O,
    ) -> KapiResult<JObject<'a>>
    where
        O: AsRef<JObject<'other_local>>,
    {
        vm.borrow()
            .attach_guard
            .new_local_ref(global_ref)
            .into_kapi()
    }

    pub fn delete_local_ref<'other_local, O>(vm: RefPseudoVM<'a>, object: O) -> KapiResult<()>
    where
        O: Into<JObject<'other_local>>,
    {
        vm.borrow()
            .attach_guard
            .delete_local_ref(object)
            .into_kapi()
    }

    pub fn new_string<S>(vm: RefPseudoVM<'a>, string: S) -> KapiResult<JString<'a>>
    where
        S: Into<JNIString>,
    {
        vm.borrow()
            .attach_guard
            .new_string(string.into())
            .into_kapi()
    }

    pub fn get_array_length<'other_local, 'array>(
        vm: RefPseudoVM<'a>,
        array: &'array impl AsJArrayRaw<'other_local>,
    ) -> KapiResult<usize> {
        vm.borrow()
            .attach_guard
            .get_array_length(array)
            .map(|size| size as _)
            .into_kapi()
    }

    pub fn get_obj_element<'other_local>(
        vm: RefPseudoVM<'a>,
        array: impl AsRef<JObjectArray<'other_local>>,
        index: usize,
    ) -> KapiResult<JObject<'a>> {
        vm.borrow_mut()
            .attach_guard
            .get_object_array_element(array, index as jsize)
            .into_kapi()
    }

    pub fn map_obj_array<'other_local, 'array, F, T>(
        vm: RefPseudoVM<'a>,
        array: &'array impl AsRef<JObjectArray<'other_local>>,
        element_mapper: F,
    ) -> KapiResult<Vec<T>>
    where
        F: Fn(&JObject<'a>) -> KapiResult<T>,
    {
        let array = array.as_ref();
        let len = Self::get_array_length(vm.clone(), array)?;
        let mut result = Vec::with_capacity(len);

        for i in 0..len {
            let obj = Self::get_obj_element(vm.clone(), array, i)?;

            result.push(element_mapper(&obj)?);

            Self::delete_local_ref(vm.clone(), obj)?;
        }

        Ok(result)
    }
}

impl<'a> Debug for PseudoVM<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PseudoVMState {{ attach_guard: ..., class_cache: ... }}")
    }
}

#[cfg(test)]
mod test {
    use crate::jvm::PseudoVM;

    #[test]
    fn test_init_vm() {
        let vm = PseudoVM::init_vm();

        assert!(vm.is_ok());
    }
}
