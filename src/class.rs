use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use jni::objects::JClass;

use crate::class::LazyClassMember::{Failed, Initialized};
use crate::error::KapiError;
use crate::types::canonical_to_internal;
use crate::utils::jvm::{get_class, get_class_modifiers, PseudoVMState};

/// Simple representation of lazy initialized class member, to avoid heavy cost of communication between
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

impl<T> LazyClassMember<T>
where
    T: Eq + PartialEq,
{
    fn get_or_init<F>(&mut self, initializer: F) -> Result<&T, KapiError>
    where
        F: FnOnce() -> Result<T, KapiError>,
    {
        match self {
            Initialized(value) => Ok(value),
            Failed(err) => Err(err.clone()),
            LazyClassMember::Uninitialized => {
                match initializer() {
                    Ok(value) => *self = Initialized(value),
                    Err(err) => *self = Failed(err),
                }

                self.get()
            }
        }
    }

    fn get(&self) -> Result<&T, KapiError> {
        match self {
            Initialized(value) => Ok(value),
            Failed(err) => Err(err.clone()),
            LazyClassMember::Uninitialized => Err(KapiError::StateError(String::from(
                "Lazy value is initialized, try call `get_or_init` first",
            ))),
        }
    }
}

/// This is a lazy representation of Class<?>, to simplify the work of interop with [`Type`].
///
/// All class data are lazily initialized to avoid heavy cost of communication between Rust and JVM.
#[derive(Debug)]
pub struct Class<'a> {
    owner: Rc<RefCell<PseudoVMState<'a>>>,
    /// Represents class' full qualified path with a 'L' prefixed and ';' suffixed, if class is an
    /// object class; otherwise, represents primitive type's actual name which has only 1 character.
    internal_name: String,
    class: JClass<'a>,
    /// Represents array type's component class type.
    component_class: Option<Rc<Class<'a>>>,
    modifiers: LazyClassMember<u32>,
}

impl<'a> Class<'a> {
    /// Tries to fetch a class JVM by canonical_name
    pub fn get_class<S>(
        vm_state: Rc<RefCell<PseudoVMState<'a>>>,
        canonical_name: S,
    ) -> Result<Rc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let canonical_str = canonical_name.into();
        let internal_name = canonical_to_internal(&canonical_str);

        {
            // ugly block it ensure ownership unfortunately
            // Look up for the cached class first, if the requested class is cached,
            // then returns it
            let vm = vm_state.deref().borrow();

            if let Some(class) = vm.class_cache.get(&internal_name) {
                return Ok(class.clone());
            }
        }

        Self::resolve_class(&vm_state, canonical_str)
    }

    fn resolve_class<S>(
        vm_state: &Rc<RefCell<PseudoVMState<'a>>>,
        canonical_name: S,
    ) -> Result<Rc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let canonical_str = canonical_name.into();
        let internal_name = canonical_to_internal(&canonical_str);

        if let Ok(class) = get_class(&internal_name) {
            if canonical_str.ends_with("[]") {
                let component_class =
                    Self::resolve_class(vm_state, canonical_str.trim_end_matches("[]"))?;
                let class = Rc::new(Self::new(
                    vm_state.clone(),
                    internal_name.clone(),
                    class,
                    Some(component_class),
                ));

                Ok(class.cache_class(internal_name, &vm_state))
            } else {
                let class = Rc::new(Self::new(
                    vm_state.clone(),
                    internal_name.clone(),
                    class,
                    None,
                ));

                Ok(class.cache_class(internal_name, &vm_state))
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

    fn cache_class(
        self: Rc<Self>,
        internal_name: String,
        vm_state: &Rc<RefCell<PseudoVMState<'a>>>,
    ) -> Rc<Self> {
        let class_cache = &mut vm_state.deref().borrow_mut().class_cache;

        class_cache.insert(internal_name, self.clone());

        self
    }

    /// Gets the belonging [`PseudoVMState`] owner of this class.
    pub fn owner(&self) -> Rc<RefCell<PseudoVMState<'a>>> {
        self.owner.clone()
    }

    /// Gets the internal name of class.
    pub fn internal_name(&self) -> &String {
        &self.internal_name
    }

    pub fn class(&self) -> &JClass<'a> {
        &self.class
    }

    /// Returns true if class is an array class.
    pub fn is_array(&self) -> bool {
        self.component_class != None
    }

    /// Returns array type's component class type. If class is not an array type, then returns
    /// [`None`] instead.
    pub fn component_class(&self) -> &Option<Rc<Class<'a>>> {
        &self.component_class
    }

    /// Returns the modifiers of class.
    pub fn modifiers(&mut self) -> Result<&u32, KapiError> {
        self.modifiers.get_or_init(|| {
            get_class_modifiers(&self.internal_name).map_err(|_| {
                KapiError::ClassResolveError(format!(
                    "Unable to resolve modifiers of class {}",
                    self.internal_name
                ))
            })
        })
    }
}

// TODO: A better way to check class's equality?
impl<'a> PartialEq for Class<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.internal_name == other.internal_name
            && self.component_class == other.component_class
            && self.modifiers == other.modifiers
    }
}

impl<'a> Eq for Class<'a> {}

#[derive(Debug)]
pub struct Method {}

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::utils::jvm::PseudoVMState;

    #[test]
    fn test_cache_class() {
        let vm = PseudoVMState::initVM();

        let string_class = Class::get_class(vm.clone(), "java.lang.String");

        assert!(string_class.is_ok());

        let cached_string_class = Class::get_class(vm.clone(), "java.lang.String");

        assert!(cached_string_class.is_ok());
        assert_eq!(*string_class.unwrap(), *cached_string_class.unwrap());
    }

    #[test]
    fn test_get_array_class() {
        let vm = PseudoVMState::initVM();

        let string_array_class_result = Class::get_class(vm.clone(), "java.lang.String[]");

        assert!(string_array_class_result.is_ok());

        let string_array_class = string_array_class_result.unwrap();

        assert!(string_array_class.is_array());

        let string_class_option = &string_array_class.component_class;

        assert!(string_class_option.is_some());

        let string_class = string_class_option.as_ref().unwrap();

        assert!(!string_class.is_array());
    }
}
