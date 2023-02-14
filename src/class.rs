use std::cell::RefCell;
use std::rc::Rc;

use jni::objects::GlobalRef;
use lazy_static::lazy_static;

use crate::class::LazyClassMember::{Failed, Initialized};
use crate::error::{KapiError, KapiResult};
use crate::jvm::{PseudoVM, RefPseudoVM};

pub type RefClass<'a> = Rc<RefCell<Class<'a>>>;
pub type RefMethod<'a> = Rc<RefCell<Method<'a>>>;

/// Simple representation of lazy initialized class member, to avoid heavy cost of communication between
/// Rust and JVM. See [`Class`].
#[derive(Debug, Eq, PartialEq)]
enum LazyClassMember<T> {
    /// Represents the data had been successfully invoked and returned from JVM.
    Initialized(T),
    /// Represents the data request invocation is failed.
    Failed(KapiError),
    /// Represents the data hasn't been initialized.
    Uninitialized,
}

lazy_static! {
    static ref UNINITIALIZED_ERROR: KapiError = KapiError::StateError(String::from(
        "Lazy value is initialized, try call `get_or_init` first"
    ));
}

impl<T> LazyClassMember<T>
where
    T: Eq + PartialEq,
{
    fn get_or_init<F, TR>(&mut self, initializer: F) -> KapiResult<&T>
    where
        F: FnOnce() -> KapiResult<TR>,
        TR: Into<T>,
    {
        match self {
            Initialized(value) => Ok(value),
            Failed(err) => Err(err.clone()),
            LazyClassMember::Uninitialized => {
                match initializer() {
                    Ok(value) => *self = Initialized(value.into()),
                    Err(err) => *self = Failed(err.into()),
                }

                self.get()
            }
        }
    }

    fn get(&self) -> KapiResult<&T> {
        match self {
            Initialized(value) => Ok(value),
            Failed(err) => Err(err.clone()),
            LazyClassMember::Uninitialized => Err(UNINITIALIZED_ERROR.clone()),
        }
    }
}

/// This is a lazy representation of Class<?>, to simplify the work of interop with [`Type`].
///
/// All class data are lazily initialized to avoid heavy cost of communication between Rust and JVM.
#[derive(Debug)]
pub struct Class<'a> {
    vm: RefPseudoVM<'a>,
    class: GlobalRef,
    component_class: Option<RefClass<'a>>,
    modifiers: LazyClassMember<u32>,
    declared_methods: LazyClassMember<Vec<RefMethod<'a>>>,
}

impl<'a> Class<'a> {
    pub(crate) fn new_class_ref(
        vm: RefPseudoVM<'a>,
        class: GlobalRef,
        component_class: Option<RefClass<'a>>,
    ) -> RefClass<'a> {
        Rc::new(RefCell::new(Self::new(vm, class, component_class)))
    }

    const fn new(
        vm: RefPseudoVM<'a>,
        class: GlobalRef,
        component_class: Option<RefClass<'a>>,
    ) -> Self {
        Self {
            vm,
            class,
            component_class,
            modifiers: LazyClassMember::Uninitialized,
            declared_methods: LazyClassMember::Uninitialized,
        }
    }
}

// TODO: A better way to check class's equality?
impl<'a> PartialEq for Class<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a> Eq for Class<'a> {}

#[derive(Debug)]
pub struct Method<'a> {
    owner: Rc<RefCell<PseudoVM<'a>>>,
    owner_class: LazyClassMember<RefClass<'a>>,
    object: GlobalRef,
    name: LazyClassMember<String>,
    parameter_types: LazyClassMember<Vec<RefClass<'a>>>,
    return_type: LazyClassMember<RefClass<'a>>,
}

impl<'a> Method<'a> {}

impl<'a> PartialEq<Self> for Method<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // TODO: Check method id should be enough
    }
}

impl<'a> Eq for Method<'a> {}

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::jvm::PseudoVM;
    
    #[test]
    fn test_get_class() {
        let vm = PseudoVM::init_vm().unwrap();
    
        let string_class = PseudoVM::get_class(vm.clone(), "java.lang.String");
    
        assert!(string_class.is_ok());
    }
    
    // #[test]
    // fn test_get_array_class() {
    //     let vm = PseudoVM::init_vm().unwrap();
    //
    //     let string_array_class_result = Class::get_class(vm.clone(), "java.lang.String[]");
    //
    //     assert!(string_array_class_result.is_ok());
    //
    //     let string_array_class_rfc = string_array_class_result.unwrap();
    //     let string_array_class = string_array_class_rfc.borrow();
    //
    //     assert!(string_array_class.is_array());
    //
    //     let string_class_option = &string_array_class.component_class;
    //
    //     assert!(string_class_option.is_some());
    //
    //     let string_class = string_class_option.as_ref().unwrap().borrow();
    //
    //     assert!(!string_class.is_array());
    // }
    //
    // #[test]
    // fn test_class_from_obj() {
    //     let vm = PseudoVM::init_vm().unwrap();
    //
    //     let string = new_string(vm.clone(), "").unwrap();
    //     let _ = Class::from_obj(vm.clone(), &string).unwrap();
    // }
    //
    // #[test]
    // fn test_class_modifiers() {
    //     let vm = PseudoVM::init_vm().unwrap();
    //
    //     let string_class_rfc = Class::get_class(vm.clone(), "java.lang.String").unwrap();
    //     let mut string_class = string_class_rfc.borrow_mut();
    //     let modifiers = string_class.modifiers().unwrap();
    //
    //     assert_eq!(17, *modifiers)
    // }
    //
    // #[test]
    // fn test_class_declared_methods() {
    //     let vm = PseudoVM::init_vm().unwrap();
    //
    //     let string_class_result = Class::get_class(vm.clone(), "java.lang.String");
    //
    //     assert!(string_class_result.is_ok());
    //
    //     let string_class_rfc = string_class_result.unwrap();
    //     let mut string_class = string_class_rfc.borrow_mut();
    //     let methods_result = string_class.declared_methods();
    //
    //     assert!(methods_result.is_ok());
    // }
    //
    // #[test]
    // fn test_method_name() {
    //     let vm = PseudoVM::init_vm().unwrap();
    //
    //     let string_class_rfc = Class::get_class(vm.clone(), "java.lang.String").unwrap();
    //     let mut string_class = string_class_rfc.borrow_mut();
    //     let mut methods = string_class.declared_methods().unwrap().to_owned();
    //
    //     let mut names = Vec::new();
    //
    //     for method_rfc in methods.iter_mut() {
    //         let mut method = method_rfc.borrow_mut();
    //         let name = method.name().unwrap();
    //
    //         names.push(name.clone());
    //     }
    // }
}
