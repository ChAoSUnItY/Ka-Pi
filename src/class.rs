use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::string::ToString;

use jni::objects::{GlobalRef, JClass, JObject, JValue};
use lazy_static::lazy_static;

use crate::class::LazyClassMember::{Failed, Initialized};
use crate::error::{IntoKapiResult, KapiError, KapiResult};
use crate::jvm::{as_global_ref, get_class, get_class_declared_methods, get_class_modifiers, get_class_name, get_obj_class, get_string, invoke_reflector_method, transform_object_array, FromObj, PseudoVMState, delete_local_ref};
use crate::types::{canonical_to_descriptor, canonical_to_internal};

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
    owner: Rc<RefCell<PseudoVMState<'a>>>,
    canonical_name: String,
    /// Represents class' full qualified path with a 'L' prefixed and ';' suffixed, if class is an
    /// object class; otherwise, represents primitive type's actual name which has only 1 characters.
    internal_name: String,
    class: GlobalRef,
    /// Represents array type's component class type.
    component_class: Option<Rc<RefCell<Class<'a>>>>,
    modifiers: LazyClassMember<u32>,
    declared_methods: LazyClassMember<Vec<Rc<RefCell<Method<'a>>>>>,
}

impl<'a> Class<'a> {
    /// Tries to fetch a class JVM by canonical_name
    pub fn get_class<S>(
        vm_state: Rc<RefCell<PseudoVMState<'a>>>,
        canonical_name: S,
    ) -> KapiResult<Rc<RefCell<Self>>>
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

        Self::resolve_class(&vm_state, &canonical_str, &internal_name)
    }

    fn resolve_class(
        vm_state: &Rc<RefCell<PseudoVMState<'a>>>,
        canonical_name: &String,
        internal_name: &String,
    ) -> KapiResult<Rc<RefCell<Self>>> {
        if let Ok(class_obj) = get_class(vm_state.clone(), canonical_to_descriptor(canonical_name))
        {
            let class = as_global_ref(vm_state.clone(), &class_obj)?;

            if internal_name.starts_with("[") {
                let component_class = Self::resolve_class(
                    vm_state,
                    &canonical_name,
                    &internal_name[1..].to_string(),
                )?;
                let class = Rc::new(RefCell::new(Self::new(
                    vm_state.clone(),
                    canonical_name.to_owned(),
                    internal_name.clone(),
                    class,
                    Some(component_class),
                )));

                Ok(Self::cache_class(
                    &vm_state,
                    class,
                    internal_name.to_owned(),
                ))
            } else {
                let class = Rc::new(RefCell::new(Self::new(
                    vm_state.clone(),
                    canonical_name.to_owned(),
                    internal_name.clone(),
                    class,
                    None,
                )));

                Ok(Self::cache_class(
                    &vm_state,
                    class,
                    internal_name.to_owned(),
                ))
            }
        } else {
            Err(KapiError::ClassResolveError(format!(
                "Unable to resolve class {}",
                canonical_name
            )))
        }
    }

    fn cache_class(
        vm_state: &Rc<RefCell<PseudoVMState<'a>>>,
        class: Rc<RefCell<Self>>,
        internal_name: String,
    ) -> Rc<RefCell<Self>> {
        let class_cache = &mut vm_state.deref().borrow_mut().class_cache;

        class_cache.insert(internal_name, class.clone());

        class
    }

    const fn new(
        owner: Rc<RefCell<PseudoVMState<'a>>>,
        canonical_name: String,
        internal_name: String,
        class: GlobalRef,
        component_class: Option<Rc<RefCell<Self>>>,
    ) -> Self {
        Self {
            owner,
            canonical_name,
            internal_name,
            class,
            component_class,
            modifiers: LazyClassMember::Uninitialized,
            declared_methods: LazyClassMember::Uninitialized,
        }
    }

    pub fn name(&self) -> KapiResult<String> {
        get_class_name(self.owner.clone(), &self.class)
    }

    /// Gets the belonging [`PseudoVMState`] owner of this class.
    pub fn owner(&self) -> Rc<RefCell<PseudoVMState<'a>>> {
        self.owner.clone()
    }

    /// Gets the internal name of class.
    pub fn internal_name(&self) -> &String {
        &self.internal_name
    }

    pub fn class(&self) -> &GlobalRef {
        &self.class
    }

    /// Returns true if class is an array class.
    pub fn is_array(&self) -> bool {
        self.component_class != None
    }

    /// Returns array type's component class type. If class is not an array type, then returns
    /// [`None`] instead.
    pub fn component_class(&self) -> &Option<Rc<RefCell<Class<'a>>>> {
        &self.component_class
    }

    /// Returns the modifiers of class.
    pub fn modifiers(&mut self) -> KapiResult<&u32> {
        self.modifiers
            .get_or_init(|| get_class_modifiers(self.owner.clone(), &self.class))
    }

    /// Returns methods declared by this class
    pub fn declared_methods(&mut self) -> KapiResult<&Vec<Rc<RefCell<Method<'a>>>>> {
        self.declared_methods
            .get_or_init(|| get_class_declared_methods(self.owner.clone(), &self.class))
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

impl<'a> FromObj<'a> for Class<'a> {
    fn from_obj(
        vm_state: Rc<RefCell<PseudoVMState<'a>>>,
        obj: &JObject<'a>,
    ) -> KapiResult<Rc<RefCell<Self>>>
    where
        Self: Sized,
    {
        let canonical_name_obj = invoke_reflector_method(
            vm_state.clone(),
            "getCanonicalName",
            "(Ljava/lang/Class;)Ljava/lang/String;",
            &[Into::<JValue>::into(get_obj_class(vm_state.clone(), obj)?)],
        )?
        .l()?;
        let canonical_name: String = vm_state
            .borrow()
            .attach_guard
            .get_string(canonical_name_obj.into())?
            .into();

        Class::get_class(vm_state, canonical_name)
    }
}

#[derive(Debug)]
pub struct Method<'a> {
    owner: Rc<RefCell<PseudoVMState<'a>>>,
    owner_class: LazyClassMember<Rc<RefCell<Class<'a>>>>,
    object: GlobalRef,
    name: LazyClassMember<String>,
    parameter_types: LazyClassMember<Vec<Rc<RefCell<Class<'a>>>>>,
    return_type: LazyClassMember<Rc<RefCell<Class<'a>>>>,
}

impl<'a> Method<'a> {
    fn get_method_clazz(vm_state: Rc<RefCell<PseudoVMState<'a>>>) -> KapiResult<JClass<'a>> {
        vm_state
            .clone()
            .borrow()
            .attach_guard
            .find_class("java/lang/reflect/Method")
            .into_kapi()
    }

    pub fn name(&mut self) -> KapiResult<&String> {
        self.name.get_or_init(|| {
            let name_obj = invoke_reflector_method(
                self.owner.clone(),
                "getName",
                "(Ljava/lang/reflect/Method;)Ljava/lang/String;",
                &[Into::<JValue>::into(self.object.as_obj())],
            )?
            .l()?;

            let string = get_string(self.owner.clone(), &name_obj);
            
            delete_local_ref(self.owner.clone(), name_obj)?;
            
            string
        })
    }

    pub fn parameter_types(&mut self) -> KapiResult<&Vec<Rc<RefCell<Class<'a>>>>> {
        self.parameter_types.get_or_init(|| {
            let parameter_types_obj_arr = as_global_ref(
                self.owner.clone(),
                &invoke_reflector_method(
                    self.owner.clone(),
                    "getParameterTypes",
                    "()[Ljava/lang/Class;",
                    &[Into::<JValue>::into(self.object.as_obj())],
                )?
                .l()?,
            )?;

            transform_object_array(self.owner.clone(), &parameter_types_obj_arr, |obj| {
                Class::from_obj(self.owner.clone(), &obj)
            })
        })
    }

    pub fn return_type(&mut self) -> KapiResult<&Rc<RefCell<Class<'a>>>> {
        self.return_type.get_or_init(|| {
            let return_type_obj = invoke_reflector_method(
                self.owner.clone(),
                "getReturnType",
                "()Ljava/lang/Class;",
                &[Into::<JValue>::into(self.object.as_obj())],
            )?
            .l()?;

            Class::from_obj(self.owner.clone(), &return_type_obj)
        })
    }
}

impl<'a> PartialEq<Self> for Method<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // TODO: Check method id should be enough
    }
}

impl<'a> Eq for Method<'a> {}

impl<'a> FromObj<'a> for Method<'a> {
    fn from_obj(
        vm_state: Rc<RefCell<PseudoVMState<'a>>>,
        obj: &JObject<'a>,
    ) -> KapiResult<Rc<RefCell<Self>>>
    where
        Self: Sized,
    {
        Ok(Rc::new(RefCell::new(Self {
            owner: vm_state.clone(),
            owner_class: LazyClassMember::Uninitialized,
            object: as_global_ref(vm_state.clone(), obj)?,
            name: LazyClassMember::Uninitialized,
            parameter_types: LazyClassMember::Uninitialized,
            return_type: LazyClassMember::Uninitialized,
        })))
    }
}

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::jvm::{new_string, FromObj, PseudoVMState};

    #[test]
    fn test_cache_class() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string_class_result = Class::get_class(vm.clone(), "java.lang.String").unwrap();

        let cached_string_class_result = Class::get_class(vm.clone(), "java.lang.String");

        assert!(cached_string_class_result.is_ok());

        assert_eq!(*string_class_result, *cached_string_class_result.unwrap());
    }

    #[test]
    fn test_get_array_class() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string_array_class_result = Class::get_class(vm.clone(), "java.lang.String[]");

        assert!(string_array_class_result.is_ok());

        let string_array_class_rfc = string_array_class_result.unwrap();
        let string_array_class = string_array_class_rfc.borrow();

        assert!(string_array_class.is_array());

        let string_class_option = &string_array_class.component_class;

        assert!(string_class_option.is_some());

        let string_class = string_class_option.as_ref().unwrap().borrow();

        assert!(!string_class.is_array());
    }

    #[test]
    fn test_class_from_obj() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string = new_string(vm.clone(), "").unwrap();
        let _ = Class::from_obj(vm.clone(), &string).unwrap();
    }

    #[test]
    fn test_class_modifiers() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string_class_rfc = Class::get_class(vm.clone(), "java.lang.String").unwrap();
        let mut string_class = string_class_rfc.borrow_mut();
        let modifiers = string_class.modifiers().unwrap();

        assert_eq!(17, *modifiers)
    }

    #[test]
    fn test_class_declared_methods() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string_class_result = Class::get_class(vm.clone(), "java.lang.String");

        assert!(string_class_result.is_ok());

        let string_class_rfc = string_class_result.unwrap();
        let mut string_class = string_class_rfc.borrow_mut();
        let methods_result = string_class.declared_methods();

        assert!(methods_result.is_ok());
    }

    #[test]
    fn test_method_name() {
        let vm = PseudoVMState::init_vm().unwrap();

        let string_class_rfc = Class::get_class(vm.clone(), "java.lang.String").unwrap();
        let mut string_class = string_class_rfc.borrow_mut();
        let mut methods = string_class.declared_methods().unwrap().to_owned();

        let mut names = Vec::new();

        for method_rfc in methods.iter_mut() {
            let mut method = method_rfc.borrow_mut();
            let name = method.name().unwrap();

            names.push(name.clone());
        }
    }
}
