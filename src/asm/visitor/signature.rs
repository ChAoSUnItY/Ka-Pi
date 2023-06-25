use crate::asm::node::signature::{FormalTypeParameter, ReferenceType, SignatureType, ThrowsType, WildcardIndicator};
use crate::asm::node::signature::{ArrayType, BaseType, ClassType, TypeArgument};

pub trait SignatureVisitor {
    type TV: TypeVisitor;
    type FTPV: FormalTypeParameterVisitor;

    /// Visits class generic signature's super class type. 
    /// 
    /// # Visit rule
    /// 
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `super_class` will be `java/lang/Object`, then super class type will be visited by
    /// [TypeVisitor].
    /// 
    /// This would be called on every class type except `java/lang/Object` which does not have any
    /// super type.
    fn visit_super_class(&mut self, super_class: &mut ClassType) -> Self::TV;

    /// Visits class generic signature's interface types. 
    ///
    /// # Visit rule
    ///
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `interfaces` will be `[java/lang/Runnable, java/lang/AutoCloseable]`, then 
    /// interface types will be visited by [Self::visit_interface].
    fn visit_interfaces(&mut self, interfaces: &mut Vec<ClassType>);
    
    /// Visits class generic signature's interface type. 
    /// 
    /// # Visit rule
    /// 
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `interface` will be `java/lang/Runnable` and `java/lang/AutoCloseable`, then 
    /// interface types will be visited by [TypeVisitor].
    fn visit_interface(&mut self, interface: &mut ClassType) -> Self::TV;

    /// Visits field generic signature's type.
    /// 
    /// # Visit rule
    /// 
    /// Consider field signature `Ljava/lang/String;`, parameter `field_type` will be [ClassType] 
    /// `java/lang/String`, then the type will be visited by [TypeVisitor].
    fn visit_field_type(&mut self, field_type: &mut ReferenceType) -> Self::TV;

    /// Visits method generic signature's parameter types.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `parameter_types` will be
    /// `[java/lang/Object, I]`, then parameter types will be visited by [Self::visit_parameter_type].
    fn visit_parameter_types(&mut self, parameter_types: &mut Vec<SignatureType>);

    /// Visits method generic signature's parameter type.
    /// 
    /// # Visit rule
    /// 
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `parameter_type` will be
    /// [ClassType] `java/lang/Object` and [BaseType] `I`, then parameter types will be visited by
    /// [TypeVisitor].
    fn visit_parameter_type(&mut self, parameter_type: &mut SignatureType) -> Self::TV;

    /// Visits method generic signature's return type.
    /// 
    /// # Visit rule
    /// 
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `return_type` will be
    /// [BaseType] `I`, then return type will be visited by [TypeVisitor].
    fn visit_return_type(&mut self, return_type: &mut SignatureType) -> Self::TV;

    /// Visits method generic signature's exception types.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `()V^TT1;^TT2;`, parameter `throws_types` will be `[T1, T2]`, then 
    /// throws types will be visited by [Self::visit_exception_type].
    fn visit_exception_types(&mut self, throws_types: &mut Vec<ThrowsType>);
    
    /// Visits method generic signature's exception type.
    /// 
    /// # Visit rule
    /// 
    /// Consider method signature `()V^TT;`, parameter `throws_type` will be
    /// [TypeVariable](crate::asm::node::signature::TypeVariable) `T`, then throws type will be
    /// visited by [TypeVisitor].
    fn visit_exception_type(&mut self, throws_type: &mut ThrowsType) -> Self::TV;

    /// Visits generic signature's formal type parameters.
    ///
    /// # Visit rule
    fn visit_formal_type_parameters(&mut self, formal_type_parameters: &mut Vec<FormalTypeParameter>);

    /// Visits generic signature's formal type parameter.
    /// 
    /// # Visit rule
    fn visit_formal_type_parameter(&mut self, formal_type_parameter: &mut FormalTypeParameter) -> Self::FTPV;
}

/// A visitor to visit formal type parameters in generic signature.
#[allow(unused_variables)]
pub trait FormalTypeParameterVisitor {
    type TV: TypeVisitor;

    fn visit_class_bound(&mut self) -> Self::TV;

    fn visit_interface_bound(&mut self) -> Self::TV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A visitor to visit types in generic signature.
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Start Visit)]
///     A1[visit_base_type]
///     A2[visit_class_type]
///     A3[visit_array_type]
///     A4[visit_type_variable]
///     B0[visit_type_arguments]
///     B1[visit_type_argument]
///     B2[visit_type_argument_bound]
///     B3[visit_type_argument_wildcard]
///     C0[visit_inner_classes]
///     C1[visit_inner_class]
///     
///     A0 --> A1
///     A0 --> A2
///     A0 --> A3
///     A0 --> A4
///
///     A3 --> A2
///     A3 --> A1
///     A3 --> A3
///
///     A2 --> B0
///     B0 --> B1
///     B1 --> B2
///     B1 --> B3
///     B2 --> A2
///     B2 --> A3
///     B2 --> A4
///
///     A2 --> C0
///     C0 --> C1
///     C1 --> B0
///     C1 --> C0
/// ```
#[allow(unused_variables)]
pub trait TypeVisitor {
    /// Visits [BaseType].
    ///
    /// # Visit rule
    ///
    /// Consider primitive type `I`, parameter `base_type` will be [BaseType::Int], since [BaseType]
    /// is guaranteed to be an terminal node of the signature, no more visiting will be occurred on
    /// this type.
    fn visit_base_type(&mut self, base_type: &mut BaseType) {}

    /// Visits [ArrayType].
    ///
    /// # Visit rule
    ///
    /// Consider type `[[example/Class[TT1;].InnerClass[TT2;]`, parameter `array_type` will be same
    /// as the type given, and the base type will be visited by other visit functions if satisfied.
    fn visit_array_type(&mut self, array_type: &mut ArrayType) {}

    /// Visits [ClassType].
    ///
    /// # Visit rule
    ///
    /// Consider class path `example/Class.InnerClass[TT1;].InnerMostClass[TT2;]`, parameter `class_type` 
    /// will be `example/Class`, the type arguments of the class type will be visited by 
    /// [Self::visit_type_arguments] and [Self::visit_type_argument]. And inner classes will be visited 
    /// by [Self::visit_inner_class_types] and [Self::visit_inner_class_type].
    fn visit_class_type(&mut self, class_type: &mut ClassType) {}
    
    /// Visits type variable [TypeVariable](crate::asm::node::signature::TypeVariable).
    /// 
    /// # Visit rule
    /// 
    /// Consider type variable `TT;` parameter `name` will be `"T"`, since 
    /// [TypeVariable](crate::asm::node::signature::TypeVariable) is guaranteed to be an terminal node 
    /// of the signature, no more visiting will be occurred on this type.
    fn visit_type_variable(&mut self, name: &mut String) {}
    
    /// Visits inner classes of [ClassType].
    ///
    /// # Visit rule
    ///
    /// Consider class path `example/Class.InnerClass[TT1;].InnerMostClass[TT2;]`, parameter
    /// `inner_classes` will be `[InnerClass[TT1;], InnerMostClass[TT2;]]`, and type arguments of both 
    /// inner classes will be visited by [Self::visit_type_arguments] in order.
    fn visit_inner_class_types(&mut self, inner_classes: &mut Vec<(String, Vec<TypeArgument>)>) {}

    /// Visits inner class of [ClassType].
    ///
    /// # Visit rule
    ///
    /// Consider class path `example/Class.InnerClass[TT1;].InnerMostClass[TT2;]`, parameter
    /// `name` will be `InnerClass` and `InnerMostClass` in order in different calls. and their type
    /// parameters of both inner classes will be visited by [Self::visit_type_arguments] in order.
    fn visit_inner_class_type(&mut self, name: &mut String) {}

    /// Visits all [TypeArgument] of [ClassType] or its inner class type.
    ///
    /// # Visit rule
    /// 
    /// Consider type argument signature `[TT1;TT2;]`, parameter `type_arguments` will be `[T1, T2]`,
    /// then each type arguments will be visited by [Self::visit_type_argument].
    fn visit_type_arguments(&mut self, type_arguments: &mut Vec<TypeArgument>) {}

    /// Visits [TypeArgument].
    /// 
    /// # Visit rule
    /// 
    /// Consider type argument `[+Ljava/lang/Object;]`, parameter `type_argument` will be `java/lang/Object`,
    /// then the type argument will be visited by either [Self::visit_type_argument_bounded] or 
    /// [Self::visit_type_argument_wildcard].
    fn visit_type_argument(&mut self, type_argument: &mut TypeArgument) {}

    /// Visits [TypeArgument::Bounded].
    /// 
    /// # Visit rule
    /// 
    /// Consider type argument `[+Ljava/lang/Object;]`, parameter `wildcard` will be 
    /// [WildcardIndicator::EXTENDS], and parameter `reference_type` will be `java/lang/Object`,
    /// them the `reference_type` will be visited by either [Self::visit_array_type], 
    /// [Self::visit_class_type], or [Self::visit_type_variable].
    fn visit_type_argument_bounded(&mut self, wildcard: &mut WildcardIndicator, reference_type: &mut ReferenceType) {}

    /// Visits [TypeArgument::Wildcard].
    /// 
    /// # Visit rule
    /// 
    /// It is guaranteed that this function will only visit type argument `[*]`, as known as wildcard,
    /// and no more visiting will be occurred.
    /// 
    /// # Mutation
    /// 
    /// To mutate type argument itself, use [Self::visit_type_argument].
    fn visit_type_argument_wildcard(&mut self) {}
}

/// Default signature visitor implementation This visitor is meant to used in non-behavioral situations.
/// 
/// # Usage
/// While implementing a signature visitor, you need to specify children visitor's implementation as
/// well, sometimes you may not want to do anything with children visitor, to avoid this, you can use
/// [SignatureVisitorImpl].
/// 
/// ```rust
/// use ka_pi::asm::visitor::signature::SignatureVisitor;
/// 
/// #[derive(Default)]
/// pub struct ExampleVisitor {
///     // ...
/// }
/// ```
#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl SignatureVisitor for SignatureVisitorImpl {
    type TV = Self;
    type FTPV = Self;

    fn visit_super_class(&mut self, super_class: &mut ClassType) -> Self::TV {
        todo!()
    }

    fn visit_interfaces(&mut self, interfaces: &mut Vec<ClassType>) {
        todo!()
    }

    fn visit_interface(&mut self, interface: &mut ClassType) -> Self::TV {
        todo!()
    }

    fn visit_field_type(&mut self, field_type: &mut ReferenceType) -> Self::TV {
        todo!()
    }

    fn visit_parameter_types(&mut self, parameter_types: &mut Vec<SignatureType>) {
        todo!()
    }

    fn visit_parameter_type(&mut self, parameter_type: &mut SignatureType) -> Self::TV {
        todo!()
    }

    fn visit_return_type(&mut self, return_type: &mut SignatureType) -> Self::TV {
        todo!()
    }

    fn visit_exception_types(&mut self, throws_types: &mut Vec<ThrowsType>) {
        todo!()
    }

    fn visit_exception_type(&mut self, throws_type: &mut ThrowsType) -> Self::TV {
        todo!()
    }

    fn visit_formal_type_parameters(&mut self, formal_type_parameters: &mut Vec<FormalTypeParameter>) {
        todo!()
    }

    fn visit_formal_type_parameter(&mut self, _: &mut FormalTypeParameter) -> Self::FTPV {
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
