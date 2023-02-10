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

use jni::objects::{GlobalRef, JObject, JValue};
use jni::sys::jvalue;
use jni::{
    errors::Result,
    objects::JClass,
    signature::{Primitive, ReturnType},
    AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM,
};

use crate::class::{Class, Method};

pub trait FromVMState<'a> {
    fn from(vm_state: Rc<RefCell<PseudoVMState<'a>>>, j_object: JObject<'a>) -> Result<Self>
    where
        Self: Sized;
}

/// A [`PseudoVMState`] represents an intermediate communication bridge between Rust and JVM, which
/// contains several useful functions and a class cache.
pub struct PseudoVMState<'a> {
    pub(crate) attach_guard: AttachGuard<'a>,
    /// caches classes to prevent huge cost while retrieving class info from JVM.
    pub class_cache: HashMap<String, Rc<Class<'a>>>,
}

impl<'a> PseudoVMState<'a> {
    /// Initializes Java Virtual Machine and returns a pseudo VM state struct to represent an intermediate
    /// communication bridge between Rust and JVM.
    pub fn init_vm() -> Rc<RefCell<PseudoVMState<'a>>> {
        Rc::new(RefCell::new(PseudoVMState {
            attach_guard: attach_current_thread(),
            class_cache: HashMap::new(),
        }))
    }
}

impl<'a> Debug for PseudoVMState<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PseudoVMState {{ attach_guard: ..., class_cache: ... }}")
    }
}

fn jvm() -> &'static Arc<JavaVM> {
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

fn env<'a>() -> Result<JNIEnv<'a>> {
    jvm().get_env()
}

fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("Failed to attach jvm thread")
}

fn get_clazz<'a>() -> Result<JClass<'a>> {
    attach_current_thread().find_class("java/lang/Class")
}

/// Get a [`JClass`](JClass) from current JVM environment.
pub(crate) fn get_class<'a>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    class_name: &String,
) -> Result<JClass<'a>> {
    vm_state.borrow().attach_guard.find_class(class_name)
}

pub(crate) fn as_global_ref(
    vm_state: Rc<RefCell<PseudoVMState>>,
    obj: JObject,
) -> Result<GlobalRef> {
    vm_state.borrow().attach_guard.new_global_ref(obj)
}

pub(crate) fn invoke_method<'a, S1, S2>(
    vm_state: Rc<RefCell<PseudoVMState<'a>>>,
    class: &JClass,
    name: S1,
    sig: S2,
    args: &[jvalue],
    return_type: ReturnType,
) -> Result<JValue<'a>>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let guard = &vm_state.borrow().attach_guard;
    let method_id = guard.get_method_id(*class, name.into(), sig.into())?;
    guard.call_method_unchecked(*class, method_id, return_type, args)
}

/// Get a [`u32`] represents modifiers applied on class.
///
/// The returned [`u32`] is a bitset to represents combination of access modifiers.
///
/// ## See [JVM 4.1 Table 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1)
pub(crate) fn get_class_modifiers(
    vm_state: Rc<RefCell<PseudoVMState>>,
    class: &JClass,
) -> Result<u32> {
    invoke_method(
        vm_state,
        class,
        "getModifiers",
        "()I",
        &[],
        ReturnType::Primitive(Primitive::Int),
    )?
    .i()
    .map(|i| i as u32)
}

pub(crate) fn get_class_declared_methods<'a>(
    vm_state: Rc<RefCell<PseudoVMState>>,
    class: &JClass,
) -> Result<Vec<Rc<Method<'a>>>> {
    let declared_methods_obj = invoke_method(
        vm_state,
        class,
        "getDeclaredMethods",
        "()[java/lang/reflect/Method;",
        &[],
        ReturnType::Array,
    )?
    .l()?;
    let guard = &vm_state.borrow().attach_guard;
    let len = guard.get_array_length(declared_methods_obj.cast())?;
    let mut declared_methods = Vec::with_capacity(len as usize);

    for i in 0..len {
        declared_methods.push(guard.get_object_array_element(declared_methods_obj.cast(), i)?);
    }

    todo!()
}