//! JVM invocation module used to init and handle communication between Rust and JVM.
//!
//! To use it, you'll have to enable feature `interop`.
//!
//! Note: Before using any functions, you should call [`PseudoVMState::init_vm()`](PseudoVMState::init_vm)
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

use crate::class::{Class, Method};
use crate::error::{IntoKapiResult, KapiError, KapiResult};

pub trait FromObj<'a> {
    fn from_obj(
        vm_state: Rc<RefCell<PseudoVMState<'a>>>,
        obj: &JObject<'a>,
    ) -> KapiResult<Rc<RefCell<Self>>>
    where
        Self: Sized;
}

/// A [`PseudoVMState`] represents an intermediate communication bridge between Rust and JVM, which
/// contains several useful functions and a class cache.
pub struct PseudoVMState<'a> {
    pub(crate) attach_guard: AttachGuard<'a>,
    /// caches classes to prevent huge cost while retrieving class info from JVM.
    pub class_cache: HashMap<String, Rc<RefCell<Class<'a>>>>,
    reflector_class: JClass<'a>,
}

impl<'a> PseudoVMState<'a> {
    /// Initializes Java Virtual Machine and returns a pseudo VM state struct to represent an intermediate
    /// communication bridge between Rust and JVM.
    pub fn init_vm() -> KapiResult<Rc<RefCell<PseudoVMState<'a>>>> {
        let guard = attach_current_thread()?;
        let reflector_class = guard.find_class("Reflector")?;

        Ok(Rc::new(RefCell::new(PseudoVMState {
            attach_guard: guard,
            class_cache: HashMap::new(),
            reflector_class,
        })))
    }
}

impl<'a> Debug for PseudoVMState<'a> {
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

            {
                let guard = jvm.attach_current_thread()?;

                // Load lib/src/Reflector.class
                let class_class = guard.find_class("java/lang/Class")?;
                let class_loader = guard
                    .call_method(
                        class_class,
                        "getClassLoader",
                        "()Ljava/lang/ClassLoader;",
                        &[],
                    )?
                    .l()?;
                let class_bytes = include_bytes!("../lib/src/Reflector.class");
                let reflector_class = guard.define_class("Reflector", class_loader, class_bytes)?;
            }

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

fn env<'a>() -> KapiResult<JNIEnv<'a>> {
    jvm()?.get_env().into_kapi()
}

fn attach_current_thread() -> KapiResult<AttachGuard<'static>> {
    jvm()?.attach_current_thread().into_kapi()
}

pub(crate) fn get_class<S>(
    vm_state: Rc<RefCell<PseudoVMState>>,
    class_name: S,
) -> KapiResult<JObject>
where
    S: Into<String>,
{
    let class_name_string = {
        let vm = vm_state.borrow();

        vm.attach_guard.new_string(class_name.into())?
    };
    let class_obj_value = invoke_reflector_method(
        vm_state.clone(),
        "forName",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        &[Into::<JValue>::into(class_name_string)],
    )?;

    Ok(class_obj_value.l()?)
}

/// Get the [`JClass`](JClass) from current JVM based on given [`JObject`](JObject) instance.
pub(crate) fn get_obj_class<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    obj: &JObject<'a>,
) -> KapiResult<JClass<'a>> {
    vm_state
        .borrow()
        .attach_guard
        .get_object_class(*obj)
        .into_kapi()
}

/// Mark a [`JObject`](JObject) to ensure JVM won't gc the object.
pub(crate) fn as_global_ref<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    obj: &JObject<'a>,
) -> KapiResult<GlobalRef> {
    vm_state
        .borrow()
        .attach_guard
        .new_global_ref(*obj)
        .into_kapi()
}

/// Force drops a [`JObject`](JObject).
pub(crate) fn delete_local_ref<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    obj: JObject<'a>,
) -> KapiResult<()> {
    vm_state
        .borrow()
        .attach_guard
        .delete_local_ref(obj)
        .into_kapi()
}

/// Invokes an instance function based on the given class method from [`class`](JClass) argument,
/// e.g. to get class `java.lang.String`'s name through `Class<?>#getName()`, you'll have to pass
/// first [`JClass`](JClass) argument with an `Class<?>`, notice that you can directly call
/// [`get_clazz`](get_clazz) to retrieve, and you'll have to pass second [`JClass`](JClass) argument
/// with class `Class<String>`, and by supplying the remaining arguments, the result would be expected.
pub(crate) fn invoke_reflector_method<'a, S1, S2>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    name: S1,
    sig: S2,
    args: &[JValue],
) -> KapiResult<JValue<'a>>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let vm = vm_state.borrow();
    vm.attach_guard
        .call_static_method(vm.reflector_class, name.into(), sig.into(), args)
        .into_kapi()
}

pub(crate) fn transform_object_array<'a, F, T>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    obj: &GlobalRef,
    mapper: F,
) -> KapiResult<Vec<T>>
where
    F: Fn(&JObject<'a>) -> KapiResult<T>,
{
    let guard = &vm_state.borrow().attach_guard;
    let arr = obj.as_obj().cast();
    let len = guard.get_array_length(arr)?;
    let mut objs = Vec::with_capacity(len as usize);

    for i in 0..len {
        let obj = guard.get_object_array_element(arr, i)?;

        objs.push(mapper(&obj)?);
        
        guard.delete_local_ref(obj)?;
    }

    Ok(objs)
}

pub(crate) fn new_string<S>(vm_state: Rc<RefCell<PseudoVMState>>, str: S) -> KapiResult<JString>
where
    S: Into<JNIString>,
{
    let vm = vm_state.borrow();

    vm.attach_guard.new_string(str).into_kapi()
}

pub(crate) fn get_string<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    obj: &JObject<'a>,
) -> KapiResult<String> {
    let guard = &vm_state.borrow().attach_guard;

    guard
        .get_string(Into::<JString>::into(*obj))
        .into_kapi()
        .map(|s| s.into())
}

pub(crate) fn get_class_name(
    vm_state: Rc<RefCell<PseudoVMState>>,
    class: &GlobalRef,
) -> KapiResult<String> {
    let name_obj = invoke_reflector_method(
        vm_state.clone(),
        "getName",
        "(Ljava/lang/Class;)Ljava/lang/String;",
        &[Into::<JValue>::into(class.as_obj())],
    )?
    .l()?;

    get_string(vm_state.clone(), &name_obj).into_kapi()
}

/// Get a [`u32`] represents modifiers applied on class.
///
/// The returned [`u32`] is a bitset to represents combination of access modifiers.
///
/// ## See [JVM 4.1 Table 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1)
pub(crate) fn get_class_modifiers(
    vm_state: Rc<RefCell<PseudoVMState>>,
    class: &GlobalRef,
) -> KapiResult<u32> {
    invoke_reflector_method(
        vm_state,
        "getModifiers",
        "(Ljava/lang/Class;)I",
        &[Into::<JValue>::into(class.as_obj())],
    )?
    .i()
    .map(|i| i as u32)
    .into_kapi()
}

pub(crate) fn get_class_declared_methods<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    class: &GlobalRef,
) -> KapiResult<Vec<Rc<RefCell<Method<'a>>>>> {
    let declared_methods_obj_arr = as_global_ref(
        vm_state.clone(),
        &invoke_reflector_method(
            vm_state.clone(),
            "getDeclaredMethods",
            "(Ljava/lang/Class;)[Ljava/lang/reflect/Method;",
            &[Into::<JValue>::into(class.as_obj())],
        )?
        .l()?,
    )?;

    transform_object_array(vm_state.clone(), &declared_methods_obj_arr, |obj| {
        Method::from_obj(vm_state.clone(), obj)
    })
}
