use std::cell::RefCell;
use std::convert::Into;
use std::rc::Rc;

use jni::objects::GlobalRef;

use crate::class::LazyClassMember::{Failed, Initialized, Uninitialized};
use crate::error::{KapiError, KapiResult};
use crate::jvm::{PseudoVM, RefPseudoVM};

pub type RefClass<'a> = Rc<RefCell<Class<'a>>>;
pub type RefMethod<'a> = Rc<RefCell<Method<'a>>>;

/// Simple representation of lazy initialized class member, to avoid heavy cost of communication between
/// Rust and JVM. See [`Class`].
#[derive(Debug, Clone, Eq, PartialEq)]
enum LazyClassMember<T> {
    /// Represents the data had been successfully invoked and returned from JVM.
    Initialized(T),
    /// Represents the data request invocation is failed.
    Failed(KapiError),
    /// Represents the data hasn't been initialized.
    Uninitialized,
}

impl<T> LazyClassMember<T>
where
    T: Eq + PartialEq,
{
    const UNINITIALIZED_ERROR: KapiError = KapiError::StateError("Member hasn't been initialized yet.");
    
    const fn new() -> Self {
        Uninitialized
    }
    
    fn get_or_init<F, TR>(&mut self, initializer: F) -> KapiResult<&T>
    where
        F: FnOnce() -> KapiResult<TR>,
        TR: Into<T>,
    {
        match self {
            Initialized(value) => Ok(value),
            Failed(err) => Err(err.clone()),
            Uninitialized => {
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
            Uninitialized => Err(Self::UNINITIALIZED_ERROR),
        }
    }
}

/// This is a lazy representation of Class<?>, to simplify the work of interop with [`Type`].
///
/// All class data are lazily initialized to avoid heavy cost of communication between Rust and JVM.
#[derive(Debug, Clone)]
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
            modifiers: LazyClassMember::new(),
            declared_methods: LazyClassMember::new(),
        }
    }

    pub fn component_class(class: RefClass<'a>) -> Option<RefClass<'a>> {
        let class_ref = class.borrow();

        class_ref.component_class.clone()
    }

    pub fn modifiers(&mut self) -> KapiResult<&u32> {
        self.modifiers.get_or_init(|| {
            let self_obj = PseudoVM::new_local_ref(self.vm.clone(), &self.class)?;
            let modifiers =
                PseudoVM::call_method(self.vm.clone(), self_obj, "getModifiers", "()I", &[])?
                    .i()?;
            PseudoVM::delete_local_ref(self.vm.clone(), self_obj)?;

            Ok(modifiers as u32)
        })
    }
}

impl PartialEq for Class<'_> {
    fn eq(&self, other: &Self) -> bool {
        let self_obj = PseudoVM::new_local_ref(self.vm.clone(), &self.class);
        let other_obj = PseudoVM::new_local_ref(self.vm.clone(), &other.class);

        if self_obj.is_err() || other_obj.is_err() {
            return false;
        }

        let self_obj = self_obj.unwrap();
        let other_obj = other_obj.unwrap();

        let eq = PseudoVM::call_method(
            self.vm.clone(),
            self_obj,
            "equals",
            "(Ljava/lang/Object;)Z",
            &[other_obj.into()],
        )
        .map_or(false, |value| value.z().unwrap_or(false));

        let _ = PseudoVM::delete_local_ref(self.vm.clone(), self_obj);
        let _ = PseudoVM::delete_local_ref(self.vm.clone(), other_obj);

        eq
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

    #[test]
    fn test_get_array_class() {
        let vm = PseudoVM::init_vm().unwrap();

        let array_class = PseudoVM::get_class(vm.clone(), "[Ljava.lang.String;");

        assert!(array_class.is_ok());

        let array_class = array_class.unwrap();
        let component_class = Class::component_class(array_class.clone());

        assert!(component_class.is_some());

        let component_class = component_class.unwrap();
        let string_class = PseudoVM::get_class(vm.clone(), "java.lang.String");

        assert!(string_class.is_ok());

        let string_class = string_class.unwrap();

        assert_eq!(component_class, string_class);
    }

    #[test]
    fn test_class_modifiers() {
        let vm = PseudoVM::init_vm().unwrap();
    
        let string_class = PseudoVM::get_class(vm.clone(), "java.lang.String");
        
        assert!(string_class.is_ok());
        
        let string_class = string_class.unwrap();
        let mut string_class = string_class.borrow_mut();
        let modifiers = string_class.modifiers();
        
        assert!(modifiers.is_ok());
        
        let modifiers = modifiers.unwrap();
    
        assert_eq!(17, *modifiers)
    }
    
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
