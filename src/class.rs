use jni::objects::JClass;

use crate::class::LazyClassMember::Failed;
use crate::error::KapiError;
use crate::types::Type;
use crate::utils::jvm::{get_class, get_class_modifiers};

/// Simple representation of lazy initialized class member, to avoid huge cost of communication between
/// Rust and JVM. See [`Class`].
enum LazyClassMember<T> {
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
pub struct Class<'a> {
    pub class_path: String,
    pub class: JClass<'a>,
    modifiers: LazyClassMember<u32>,
}

impl<'a> Class<'a> {
    pub fn get_class<S>(class_path: S) -> Result<Self, KapiError> where S: Into<String> {
        let path_str = class_path.into();

        if let Ok(jclass) = get_class(&path_str) {
            let class = Self::new(path_str.clone(), jclass);

            Ok(class)
        } else {
            Err(KapiError::ClassResolveError(format!("Unable to resolve class {}", path_str)))
        }
    }

    const fn new(class_path: String, class: JClass<'a>) -> Self {
        Self {
            class_path,
            class,
            modifiers: LazyClassMember::Uninitialized,
        }
    }

    pub fn modifiers(&mut self) -> Result<u32, KapiError> {
        if let LazyClassMember::Initialized(modifiers) = self.modifiers {
            Ok(modifiers)
        } else if let Ok(modifiers) = get_class_modifiers(&self.class_path) {
            self.modifiers = LazyClassMember::Initialized(modifiers);

            Ok(modifiers)
        } else {
            let err = KapiError::ClassResolveError(format!("Unable to resolve modifiers of class {}", self.class_path));

            self.modifiers = Failed(err.clone());

            Err(err)
        }
    }
}
