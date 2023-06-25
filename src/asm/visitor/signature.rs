use crate::asm::node::signature::{ArrayType, BaseType, ClassType, Signature, TypeArgument};
use crate::asm::node::signature::{
    FormalTypeParameter, ReferenceType, SignatureType, ThrowsType, WildcardIndicator,
};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A general signature visitor for visiting [Signature](Signature).
///
/// # Visit strategy overview
/// ```mermaid
/// flowchart
///     subgraph SignatureVisitor
///         SV_A0[(Entry Point)]
///     end
///     
///     subgraph ClassSignatureVisitor
///         CSV_A1{{FormalTypeParameterVisitable}}
///         CSV_A3[visit_super_class]
///         CSV_A4[visit_interfaces]
///         CSV_A5[visit_interface]
///
///         SV_A0 -->|Signature::Class| CSV_A1 --> CSV_A3 --> CSV_A4 --> CSV_A5
///     end
///     
///     subgraph FieldSignatureVisitor
///         FSV_A1[visit_field_type]
///
///         SV_A0 -->|Signature::Field| FSV_A1
///     end
///     
///     subgraph MethodVisitor
///         MV_A1{{FormalTypeParameterVisitable}}
///         MV_A3[visit_parameter_types]
///         MV_A4[visit_parameter_type]
///         MV_A5[visit_return_type]
///         MV_A6[visit_exception_types]
///         MV_A7[visit_exception_type]
///
///         SV_A0 -->|Signature::Method| MV_A1 --> MV_A3 --> MV_A4 --> MV_A5 --> MV_A6 --> MV_A7
///     end
///     
///     subgraph FormalTypeParameterVisitable
///         FTPV0_A1[visit_formal_types]
///         FTPV0_A2[visit_formal_type]
///
///         CSV_A1 & MV_A1 --> FTPV0_A1
///         FTPV0_A1 --> FTPV0_A2
///     end
///     
///     subgraph FormalTypeParameterVisitor
///         FTPV_A1[visit_class_bound]
///         FTPV_A2[visit_interface_bounds]
///         FTPV_A3[visit_interface_bound]
///
///         FTPV0_A2 --> FTPV_A1
///         FTPV_A1 --> FTPV_A2
///         FTPV_A2 --> FTPV_A3
///     end
///     
///     subgraph TypeVisitor
///         TV_TYP_REF{ReferenceType}
///         TV_TYP_SIG{SignatureType}
///         TV_TYP_THR{ThrowsType}
///         TV_TYP_BAS{BaseType}
///         TV_TYP_ARR{ArrayType}
///         TV_TYP_CLS{ClassType}
///         TV_TYP_TYP_VAR{TypeVariable}
///         
///         TV_A1[visit_base_type]
///         TV_A2[visit_class_type]
///         TV_A3[visit_array_type]
///         TV_A4[visit_type_variable]
///         TV_B0[visit_type_arguments]
///         TV_B1[visit_type_argument]
///         TV_B2[visit_type_argument_bound]
///         TV_B3[visit_type_argument_wildcard]
///         TV_C0[visit_inner_classes]
///         TV_C1[visit_inner_class]
///
///         CSV_A3 & CSV_A5 & FTPV_A3 & FTPV_A1 & TV_TYP_REF & TV_B2 & TV_A3 --> TV_TYP_CLS
///
///         FSV_A1 & TV_TYP_SIG & TV_TYP_THR --> TV_TYP_REF
///
///         MV_A4 & MV_A5 --> TV_TYP_SIG
///         MV_A7 --> TV_TYP_THR
///
///         TV_TYP_SIG & TV_A3 --> TV_TYP_BAS
///         TV_TYP_THR --> TV_TYP_TYP_VAR
///         TV_TYP_REF --> TV_TYP_ARR
///         TV_TYP_REF --> TV_TYP_TYP_VAR
///         TV_TYP_BAS --> TV_A1
///         TV_TYP_ARR --> TV_A3
///         TV_TYP_CLS --> TV_A2
///         TV_TYP_TYP_VAR --> TV_A4
///
///         TV_A2 --> TV_B0
///         TV_B0 --> TV_B1
///         TV_B1 --> TV_B2
///         TV_B1 --> TV_B3
///         TV_B2 --> TV_A3 & TV_TYP_TYP_VAR
///
///         TV_A2 --> TV_C0
///         TV_C0 --> TV_C1
///         TV_C1 --> TV_B0
///         TV_C1 --> TV_C0
///     end
/// ```
pub trait SignatureVisitor:
    ClassSignatureVisitor + FieldSignatureVisitor + MethodSignatureVisitor
{
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Visitor used for visiting [Signature::Class].
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Entry Point)]
///     A1["< Self as FormalTypeParameterVisitable>::visit_parameter_types"]
///     A2["< Self as FormalTypeParameterVisitable>::visit_parameter_type"]
///     A3[visit_super_class]
///     A4[visit_interfaces]
///     A5[visit_interface]
///     
///     A0 --> A1 --> A2 --> A3 --> A4 --> A5
/// ```
pub trait ClassSignatureVisitor: FormalTypeParameterVisitable {
    /// Type visitor for super class visiting.
    type SCTV: TypeVisitor;
    /// Type visitor for interface visiting.
    type ITV: TypeVisitor;

    /// Visits class signature's super class type.
    ///
    /// # Visit rule
    ///
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `super_class` will be `java/lang/Object`, then super class type will be visited by
    /// [TypeVisitor] provided by [Self::SCTV].
    ///
    /// This would be called on every class type except `java/lang/Object` which does not have any
    /// super type.
    fn visit_super_class(&mut self, super_class: &mut ClassType) -> Self::SCTV;

    //noinspection RsLiveness
    /// Visits class signature's interface types.
    ///
    /// # Visit rule
    ///
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `interfaces` will be `[java/lang/Runnable, java/lang/AutoCloseable]`, then
    /// interface types will be visited by [Self::visit_interface].
    fn visit_interfaces(&mut self, interfaces: &mut Vec<ClassType>) {}

    /// Visits class signature's interface type.
    ///
    /// # Visit rule
    ///
    /// Consider class signature `Ljava/lang/Object;java/lang/Runnable;java/lang/AutoCloseable`,
    /// parameter `interface` will be `java/lang/Runnable` and `java/lang/AutoCloseable`, then
    /// interface types will be visited by [TypeVisitor] provided by [Self::ITV].
    fn visit_interface(&mut self, interface: &mut ClassType) -> Self::ITV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Visitor used for visiting [Signature::Field].
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Entry Point)]
///     A1[visit_field_type]
///     
///     A0 --> A1
/// ```
pub trait FieldSignatureVisitor {
    /// Type visitor for field type visiting.
    type FTV: TypeVisitor;

    /// Visits field signature's type.
    ///
    /// # Visit rule
    ///
    /// Consider field signature `Ljava/lang/String;`, parameter `field_type` will be [ClassType]
    /// `java/lang/String`, then the type will be visited by [TypeVisitor] provided by [Self::FTV].
    fn visit_field_type(&mut self, field_type: &mut ReferenceType) -> Self::FTV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Visitor used for visiting [Signature::Method].
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Entry Point)]
///     A1["< Self as FormalTypeParameterVisitable>::visit_parameter_types"]
///     A2["< Self as FormalTypeParameterVisitable>::visit_parameter_type"]
///     A3[visit_parameter_types]
///     A4[visit_parameter_type]
///     A5[visit_return_type]
///     A6[visit_exception_types]
///     A7[visit_exception_type]
///     
///     A0 --> A1 --> A2 --> A3 --> A4 --> A5 --> A6 --> A7
/// ```
pub trait MethodSignatureVisitor: FormalTypeParameterVisitable {
    /// Type visitor for parameter types visiting.
    type PTV: TypeVisitor;
    /// Type visitor for return type visiting.
    type RTV: TypeVisitor;
    /// Type visitor for exception type visiting.
    type ETV: TypeVisitor;

    //noinspection RsLiveness
    /// Visits method signature's parameter types.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `parameter_types` will be
    /// `[java/lang/Object, I]`, then parameter types will be visited by [Self::visit_parameter_type].
    fn visit_parameter_types(&mut self, parameter_types: &mut Vec<SignatureType>) {}

    /// Visits method signature's parameter type.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `parameter_type` will be
    /// [ClassType] `java/lang/Object` and [BaseType] `I`, then parameter types will be visited by
    /// [TypeVisitor] provided by [Self::PTV].
    fn visit_parameter_type(&mut self, parameter_type: &mut SignatureType) -> Self::PTV;

    /// Visits method generic signature's return type.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `(Ljava/lang/Object;I)I`, parameter `return_type` will be
    /// [BaseType] `I`, then return type will be visited by [TypeVisitor] provided by [Self::RTV].
    fn visit_return_type(&mut self, return_type: &mut SignatureType) -> Self::RTV;

    //noinspection RsLiveness
    /// Visits method signature's exception types.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `()V^TT1;^TT2;`, parameter `throws_types` will be `[T1, T2]`, then
    /// throws types will be visited by [Self::visit_exception_type].
    fn visit_exception_types(&mut self, throws_types: &mut Vec<ThrowsType>) {}

    /// Visits method signature's exception type.
    ///
    /// # Visit rule
    ///
    /// Consider method signature `()V^TT;`, parameter `throws_type` will be
    /// [TypeVariable](crate::asm::node::signature::TypeVariable) `T`, then throws type will be
    /// visited by [TypeVisitor] provided by [Self::ETV].
    fn visit_exception_type(&mut self, throws_type: &mut ThrowsType) -> Self::ETV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Marks visitor that has [FormalTypeParameter] to visit.
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Entry Point)]
///     A1[visit_formal_types]
///     A2[visit_formal_type]
///     
///     A0 --> A1
///     A1 --> A2
/// ```
pub trait FormalTypeParameterVisitable {
    /// Type visitor for formal type parameter visiting.
    type FTPV: FormalTypeParameterVisitor;

    //noinspection RsLiveness
    /// Visits signature's formal type parameters.
    ///
    /// # Visit rule
    ///
    /// Consider formal type parameters `[T:R:]`, parameter `formal_type_parameters` will be
    /// `[T, R]`, then formal type parameters will be visited by [Self::visit_formal_type_parameter].
    fn visit_formal_type_parameters(
        &mut self,
        formal_type_parameters: &mut Vec<FormalTypeParameter>,
    ) {
    }

    /// Visits signature's formal type parameter.
    ///
    /// # Visit rule
    ///
    /// Consider formal type parameters `[T:R:]`, parameter `formal_type_parameter` will be
    /// [TypeVariable](crate::asm::node::signature::TypeVariable) `T` and `R`, then formal type
    /// parameters will be visited by [TypeVisitor] provided by [Self::FTPV].
    fn visit_formal_type_parameter(
        &mut self,
        formal_type_parameter: &mut FormalTypeParameter,
    ) -> Self::FTPV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Visitor used for visiting [FormalTypeParameter].
///
/// # Visit strategy
/// ```mermaid
/// flowchart
///     A0[(Entry Point)]
///     A1[visit_class_bound]
///     A2[visit_interface_bounds]
///     A3[visit_interface_bound]
///     
///     A0 --> A1
///     A1 --> A2
///     A0 --> A2
///     A2 --> A3
/// ```
pub trait FormalTypeParameterVisitor {
    /// Type visitor for class bound type visiting.
    type CBTV: TypeVisitor;
    /// Type visitor for interface bound type visiting.
    type IBTV: TypeVisitor;

    /// Visits formal type parameter's class bound if exists.
    ///
    /// # Visit rule
    ///
    /// ## When class bound exists
    ///
    /// Consider formal type parameter `T:Ljava/lang/Object;`, parameter `class_bound_type` will be
    /// [ClassType] `java/lang/Object`, then class bound type will be visited by [TypeVisitor] provided
    /// by [Self::CBTV].
    ///
    /// ## when class bound does not exists
    ///
    /// Consider formal type parameter `T:`, then [Self::visit_class_bound] will not be called.
    fn visit_class_bound(&mut self, class_bound_type: &mut ClassType) -> Self::CBTV;

    //noinspection RsLiveness
    /// Visits formal type parameter's interface bounds.
    ///
    /// # Visit rule
    ///
    /// Consider formal type parameter `T::Ljava/lang/Runnable;:Ljava/lang/AutoCloseable;`, parameter
    /// `interface_bound_types` will be `[java/lang/Runnable, java/lang/AutoCloseable]`, then interface
    /// bound types will be visited by [Self::visit_interface_bound].
    fn visit_interface_bounds(&mut self, interface_bound_types: &mut Vec<ClassType>) {}

    /// Visits formal type parameter's interface bound.
    ///
    /// # Visit rule
    ///
    /// Consider formal type parameter `T::Ljava/lang/Runnable;:Ljava/lang/AutoCloseable;`, parameter
    /// `interface_bound_type` will be [ClassType] `java/lang/Runnable` and `java/lang/AutoCloseable`,
    /// then interface bound types will be visited by [TypeVisitor] provided by [Self::IBTV].
    fn visit_interface_bound(&mut self, interface_bound_type: &mut ClassType) -> Self::IBTV;
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Visitor used for visiting [SignatureType].
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
    fn visit_type_argument_bounded(
        &mut self,
        wildcard: &mut WildcardIndicator,
        reference_type: &mut ReferenceType,
    ) {
    }

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
