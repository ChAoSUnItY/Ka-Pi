use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use jni::objects::GlobalRef;
use lazy_static::lazy_static;

use crate::class::LazyClassMember::Failed;
use crate::error::KapiError;
use crate::types::canonical_to_internal;
use crate::utils::jvm::{as_global_ref, get_class, get_class_modifiers};

lazy_static! {
    static ref CLASS_CACHE: Mutex<HashMap<String, Arc<Class>>> = Mutex::new(HashMap::new());
}

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
pub struct Class {
    /// Represents class' full qualified path with a 'L' prefixed and ';' suffixed, if class is an
    /// object class; otherwise, represents primitive type's actual name which has only 1 character.
    internal_name: String,
    class: GlobalRef,
    /// Represents array type's component class type.
    component_class: Option<Arc<Class>>,
    // descriptor: String,
    modifiers: LazyClassMember<u32>,
}

impl Class {
    pub fn get_class<S>(canonical_name: S) -> Result<Arc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let internal_name = canonical_to_internal(canonical_name);
        let class_cache = CLASS_CACHE.lock().unwrap();

        if let Some(class) = class_cache.get::<String>(&internal_name) {
            Ok(class.clone())
        } else {
            Self::resolve_class(internal_name)
        }
    }

    fn resolve_class<S>(internal_name: S) -> Result<Arc<Self>, KapiError>
    where
        S: Into<String>,
    {
        let internal_name_str = internal_name.into();

        if let Ok(global_ref) =
            get_class(&internal_name_str).and_then(|class| as_global_ref(class.into()))
        {
            if internal_name_str.ends_with("[]") {
                let component_class =
                    Self::resolve_class(internal_name_str.trim_end_matches("[]"))?;
                let class = Arc::new(Self::new(
                    internal_name_str.clone(),
                    global_ref,
                    Some(component_class),
                ));
                let mut cache = CLASS_CACHE.lock().unwrap();

                cache.insert(internal_name_str, class.clone());

                Ok(class)
            } else {
                let class = Arc::new(Self::new(internal_name_str.clone(), global_ref, None));
                let mut cache = CLASS_CACHE.lock().unwrap();

                cache.insert(internal_name_str.into(), class.clone());

                Ok(class)
            }
        } else {
            Err(KapiError::ClassResolveError(format!(
                "Unable to resolve class {}",
                internal_name_str
            )))
        }
    }

    const fn new(
        internal_name: String,
        class: GlobalRef,
        component_class: Option<Arc<Self>>,
    ) -> Self {
        Self {
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

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.internal_name == other.internal_name
            && self.component_class == other.component_class
            && self.modifiers == other.modifiers
    }
}

impl Eq for Class {}

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::utils::jvm::attach_current_thread;

    #[test]
    fn test_get_class() {
        let _ = attach_current_thread();

        let string_class = Class::get_class("java.lang.String");

        assert!(string_class.is_ok());

        let cached_string_class = Class::get_class("java.lang.String");

        assert!(cached_string_class.is_ok());
        assert_eq!(*string_class.unwrap(), *cached_string_class.unwrap());
    }
}
