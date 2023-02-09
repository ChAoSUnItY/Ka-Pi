use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use jni::objects::JClass;

use crate::class::LazyClassMember::Failed;
use crate::error::KapiError;
use crate::types::{canonical_to_descriptor, canonical_to_internal};
use crate::utils::jvm::{get_class, get_class_modifiers, PseudoVMState};

/// Simple representation of lazy initialized class member, to avoid huge cost of communication between
/// Rust and JVM. See [`Class`].
#[derive(Debug, Eq, PartialEq)]
enum LazyClassMember<T>
where
    T: Eq + PartialEq,
{
    /// Represents the data had been successfully invoked and returned from JVM.
    Initialized(T),
    /// Represents the data request invocation is failed.
    Failed(KapiError),
    /// Represents the data hasn't been initialized.
    Uninitialized,
}

/// This is a lazy representation of Class<?>, to simplify the work of interop with [`Type`].
///
/// All class data are lazily initialized to avoid huge cost of communication between Rust and JVM.
#[derive(Debug)]
pub struct Class<'a> {
    owner: Rc<RefCell<PseudoVMState<'a>>>,
    /// Represents class' full qualified path with a 'L' prefixed and ';' suffixed, if class is an
    /// object class; otherwise, represents primitive type's actual name which has only 1 character.
    internal_name: String,
    class: JClass<'a>,
    /// Represents array type's component class type.
    component_class: Option<Rc<Class<'a>>>,
    // descriptor: String,
    modifiers: LazyClassMember<u32>,
}

impl<'a> Class<'a> {
    pub fn get_class<S>(vm_state: Rc<RefCell<PseudoVMState<'a>>>, canonical_name: S) -> Result<Rc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let canonical_str = canonical_name.into();
        let internal_name = canonical_to_internal(&canonical_str);

        {   // ugly block it ensure ownership unfortunately
            // Look up for the cached class first, if the requested class is cached, 
            // then returns it
            let vm = vm_state.deref().borrow();
            
            if let Some(class) = vm.class_cache.get(&internal_name) {
                return Ok(class.clone());
            }
        }

        Self::resolve_class(&vm_state, canonical_str)
    }

    fn resolve_class<S>(vm_state: &Rc<RefCell<PseudoVMState<'a>>>, canonical_name: S) -> Result<Rc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let canonical_str = canonical_name.into();
        let descriptor = canonical_to_descriptor(&canonical_str);
        let internal_name = canonical_to_internal(&canonical_str);

        if let Ok(class) = get_class(&descriptor) {
            if canonical_str.ends_with("[]") {
                let component_class = Self::resolve_class(vm_state, canonical_str.trim_end_matches("[]"))?;
                let class = Rc::new(Self::new(
                    vm_state.clone(),
                    internal_name.clone(),
                    class,
                    Some(component_class),
                ));
                let class_cache = &mut vm_state.deref().borrow_mut().class_cache;

                class_cache.insert(internal_name, class.clone());

                Ok(class)
            } else {
                let class = Rc::new(Self::new(vm_state.clone(), internal_name.clone(), class, None));
                let class_cache = &mut vm_state.deref().borrow_mut().class_cache;

                class_cache.insert(internal_name, class.clone());
                
                Ok(class)
            }
        } else {
            Err(KapiError::ClassResolveError(format!(
                "Unable to resolve class {}",
                canonical_str
            )))
        }
    }

    const fn new(
        owner: Rc<RefCell<PseudoVMState<'a>>>,
        internal_name: String,
        class: JClass<'a>,
        component_class: Option<Rc<Self>>,
    ) -> Self {
        Self {
            owner,
            internal_name,
            class,
            component_class,
            modifiers: LazyClassMember::Uninitialized,
        }
    }

    /// Returns array type's component class type. If class is not an array type, then returns
    /// [`None`] instead.
    // pub fn component_class(&self) -> &Option<Box<Class>> {
    //     &self.component_class
    // }

    /// Returns class type's modifiers.
    pub fn modifiers(&mut self) -> Result<u32, KapiError> {
        if let LazyClassMember::Initialized(modifiers) = self.modifiers {
            Ok(modifiers)
        } else if let Ok(modifiers) = get_class_modifiers(&self.internal_name) {
            self.modifiers = LazyClassMember::Initialized(modifiers);

            Ok(modifiers)
        } else {
            let err = KapiError::ClassResolveError(format!(
                "Unable to resolve modifiers of class {}",
                self.internal_name
            ));

            self.modifiers = Failed(err.clone());

            Err(err)
        }
    }
}

impl<'a> PartialEq for Class<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.internal_name == other.internal_name
            && self.component_class == other.component_class
            && self.modifiers == other.modifiers
    }
}

impl<'a> Eq for Class<'a> {}

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::utils::jvm::PseudoVMState;

    #[test]
    fn test_get_class() {
        let vm = PseudoVMState::initVM();

        let string_class = Class::get_class(vm.clone(), "java.lang.String");

        assert!(string_class.is_ok());

        let cached_string_class = Class::get_class(vm.clone(), "java.lang.String");

        assert!(cached_string_class.is_ok());
        assert_eq!(*string_class.unwrap(), *cached_string_class.unwrap());
    }
}
