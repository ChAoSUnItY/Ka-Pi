use crate::asm::node::signature::BaseType;
use crate::asm::node::signature::WildcardIndicator;

/// A visitor to visit class generic signature. This trait requires struct also implements
/// [FormalTypeParameterVisitable].
///
/// # Implemented Examples
///
/// See [ClassSignatureWriter] for more info.
pub trait ClassSignatureVisitor: FormalTypeParameterVisitable {
    type TV: TypeVisitor + Sized;

    /// Visits class generic signature's super class type. This would be called on every classes
    /// expect `java.lang.Object`.
    fn visit_super_class(&mut self) -> Self::TV;

    /// Visits class generic signature's interface type. This could be called by multiple times
    /// when there's more than 1 interfaces implemented.
    fn visit_interface(&mut self) -> Self::TV;
}

/// A visitor to visit field generic signature.
///
/// # Implemented Examples
///
/// See [FieldSignatureWriter] for more info.
pub trait FieldSignatureVisitor {
    type TV: TypeVisitor + Sized;

    /// Visits field generic signature's type.
    fn visit_field_type(&mut self) -> Self::TV;
}

/// A visitor to visit method generic signature. This trait requires struct also implements
/// [FormalTypeParameterVisitable].
///
/// # Implemented Examples
///
/// See [MethodSignatureWriter] for more info.
pub trait MethodSignatureVisitor: FormalTypeParameterVisitable {
    type TV: TypeVisitor + Sized;

    /// Visits method generic signature's parameter type. This could be called by multiple times
    /// when there's more than 1 parameters declared.
    fn visit_parameter_type(&mut self) -> Self::TV;

    /// Visits method generic signature's return type. This would be only called once per method.
    fn visit_return_type(&mut self) -> Self::TV;

    /// Visits method generic signature's exception type. This could be called by multiple times
    /// when there's more than 1 exception types declared.
    fn visit_exception_type(&mut self) -> Self::TV;
}

/// A trait indicates super-trait visitor has formal type parameter section to be visited, which are
/// [ClassSignatureVisitor] and [MethodSignatureVisitor].
///
/// # Implemented Examples
///
/// See [ClassSignatureWriter] and [MethodSignatureWriter] for more info.
#[allow(unused_variables)]
pub trait FormalTypeParameterVisitable {
    type FTPV: FormalTypeParameterVisitor + Sized;

    /// Visits generic signature's formal type parameter. This could be called by multiple times
    /// when there's more than 1 formal type parameters declared.
    fn visit_formal_type_parameter(&mut self, name: &str) -> Self::FTPV;
}

/// A visitor to visit formal type parameters in generic signature.
///
/// # Implemented Examples
///
/// See [FormalTypeParameterWriter] for more info.
#[allow(unused_variables)]
pub trait FormalTypeParameterVisitor {
    type TV: TypeVisitor + Sized;

    /// Visits class bound in formal type parameter. This would be only called up to once per parameter.
    fn visit_class_bound(&mut self) -> Self::TV;

    /// Visits interface bound in formal type parameter. This could be called by multiple times when
    /// there's more than 1 interface bounds declared.
    fn visit_interface_bound(&mut self) -> Self::TV;
}

/// A visitor to visit types in generic signature.
///
/// # Implemented Examples
///
/// See [TypeWriter] for more info.
#[allow(unused_variables)]
pub trait TypeVisitor {
    /// Visits base type in signature. This could be any type defined by
    /// [`BaseType`].
    fn visit_base_type(&mut self, base_type: BaseType) {}

    /// Visits array type in signature. Further type visiting is required after
    /// [`visit_array_type`](TypeVisitor::visit_array_type) called. For example: you can call this
    /// [`visit_array_type`](TypeVisitor::visit_array_type) then call
    /// [`visit_base_type`](TypeVisitor::visit_base_type) to construct a base type array.
    fn visit_array_type(&mut self) {}

    /// Visits class type in signature.
    fn visit_class_type(&mut self, name: &str) {}

    /// Visits inner class type in signature. Required calling [`visit_class_type`](TypeVisitor::visit_class_type)
    /// before calling [`visit_inner_class_type`](TypeVisitor::visit_inner_class_type).
    fn visit_inner_class_type(&mut self, name: &str) {}

    /// Visits type variable in signature.
    fn visit_type_variable(&mut self, name: &str) {}

    /// Visits type argument in signature. Required calling [visit_class_type](TypeVisitor::visit_class_type)
    /// before calling [`visit_type_argument`](TypeVisitor::visit_type_argument).
    ///
    /// This function will be called when the following type is unbounded. For type argument with
    /// Wildcard, see [`visit_type_argument_wildcard`](TypeVisitor::visit_type_argument_wildcard) for
    /// more info.
    fn visit_type_argument(&mut self) {}

    /// Visits type type argument with Wildcard indicator in signature. Required calling
    /// [`visit_class_type`](TypeVisitor::visit_class_type) before calling
    /// [`visit_type_argument`](TypeVisitor::visit_type_argument).
    fn visit_type_argument_wildcard(&mut self, wildcard: WildcardIndicator) {}
}

/// Default signature visitor for internal usage only. This visitor does not have any effect on visiting
/// signatures.
#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl ClassSignatureVisitor for SignatureVisitorImpl {
    type TV = Self;

    fn visit_super_class(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_interface(&mut self) -> Self::TV {
        Self::default()
    }
}

impl FieldSignatureVisitor for SignatureVisitorImpl {
    type TV = Self;

    fn visit_field_type(&mut self) -> Self::TV {
        Self::default()
    }
}

impl MethodSignatureVisitor for SignatureVisitorImpl {
    type TV = Self;

    fn visit_parameter_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_return_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_exception_type(&mut self) -> Self::TV {
        Self::default()
    }
}

impl FormalTypeParameterVisitable for SignatureVisitorImpl {
    type FTPV = Self;

    fn visit_formal_type_parameter(&mut self, _: &str) -> Self::FTPV {
        Self::default()
    }
}

impl FormalTypeParameterVisitor for SignatureVisitorImpl {
    type TV = Self;

    fn visit_class_bound(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_interface_bound(&mut self) -> Self::TV {
        Self::default()
    }
}

impl TypeVisitor for SignatureVisitorImpl {}
