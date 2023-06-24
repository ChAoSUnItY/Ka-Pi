use crate::asm::node::signature::BaseType;
use crate::asm::node::signature::WildcardIndicator;

pub trait SignatureVisitor {
    type TV: TypeVisitor;
    type FTPV: FormalTypeParameterVisitor;

    /// Visits class generic signature's super class type. This would be called on every classes
    /// expect `java.lang.Object`.
    fn visit_super_class(&mut self) -> Self::TV;

    /// Visits class generic signature's interface type. This could be called by multiple times
    /// when there's more than 1 interfaces implemented.
    fn visit_interface(&mut self) -> Self::TV;

    /// Visits field generic signature's type.
    fn visit_field_type(&mut self) -> Self::TV;

    /// Visits method generic signature's parameter type. This could be called by multiple times
    /// when there's more than 1 parameters declared.
    fn visit_parameter_type(&mut self) -> Self::TV;

    /// Visits method generic signature's return type. This would be only called once per method.
    fn visit_return_type(&mut self) -> Self::TV;

    /// Visits method generic signature's exception type. This could be called by multiple times
    /// when there's more than 1 exception types declared.
    fn visit_exception_type(&mut self) -> Self::TV;

    /// Visits generic signature's formal type parameter. This could be called by multiple times
    /// when there's more than 1 formal type parameters declared.
    fn visit_formal_type_parameter(&mut self, name: &mut String) -> Self::FTPV;
}

/// A visitor to visit formal type parameters in generic signature.
#[allow(unused_variables)]
pub trait FormalTypeParameterVisitor {
    type TV: TypeVisitor;

    fn visit_class_bound(&mut self) -> Self::TV;

    fn visit_interface_bound(&mut self) -> Self::TV;
}

/// A visitor to visit types in generic signature.
#[allow(unused_variables)]
pub trait TypeVisitor {
    fn visit_base_type(&mut self, base_type: &mut BaseType) {}

    fn visit_array_type(&mut self) {}

    fn visit_class_type(&mut self, package_path: &mut String, class_name: &mut String) {}

    fn visit_inner_class_type(&mut self, name: &mut String) {}

    fn visit_type_variable(&mut self, name: &mut String) {}

    fn visit_type_argument_bounded(&mut self, wildcard: &mut WildcardIndicator) {}

    fn visit_type_argument_wildcard(&mut self) {}
}

/// Default signature visitor for internal usage only. This visitor does not have any effect on visiting
/// signatures.
#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl SignatureVisitor for SignatureVisitorImpl {
    type TV = Self;
    type FTPV = Self;

    fn visit_super_class(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_interface(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_field_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_parameter_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_return_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_exception_type(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_formal_type_parameter(&mut self, _: &mut String) -> Self::FTPV {
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
