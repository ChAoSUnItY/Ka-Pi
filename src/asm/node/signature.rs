use crate::asm::signature::{
    ClassSignatureVisitor, FormalTypeParameterVisitable, FormalTypeParameterVisitor, TypeVisitor,
};

pub enum Signature {
    Class {
        formal_type_parameters: Vec<FormalTypeParameter>,
        super_class: Type,
    },
}

struct ClassSignatureCollector {
    formal_type_parameters: Vec<FormalTypeParameter>,
}

impl ClassSignatureVisitor for ClassSignatureCollector {}
impl FormalTypeParameterVisitable for ClassSignatureCollector {
    fn visit_formal_type_parameter(
        &mut self,
        name: &String,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        Box::new(FormalTypeParameterCollector::new(
            name.to_owned(),
            |parameter| self.formal_type_parameters.push(parameter),
        ))
    }
}

pub struct FormalTypeParameter {
    parameter_name: String,
    class_bound: Option<Type>,
    interface_bounds: Vec<Type>,
}

struct FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    parameter_name: String,
    class_bound: Option<Type>,
    interface_bounds: Vec<Type>,
    post_action: F,
}

impl<F> FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    fn new(parameter_name: String, post_action: F) -> Self {
        Self {
            parameter_name,
            class_bound: None,
            interface_bounds: Vec::new(),
            post_action,
        }
    }
}

impl<F> FormalTypeParameterVisitor for FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    fn visit_end(&mut self) {
        (self.post_action)(FormalTypeParameter {
            parameter_name: self.parameter_name.clone(),
            class_bound: self.class_bound.clone(),
            interface_bounds: self.interface_bounds.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    InnerClass(String),
    Array(Box<Type>),
    Unknown,
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum BaseType {
    
}

struct TypeCollector<F>
where
    F: FnMut(Type),
{
    holder: Type,
    post_action: F,
}

impl<F> TypeCollector<F>
where
    F: FnMut(Type),
{
    fn new(post_action: F) -> Self {
        Self {
            holder: Type::Unknown,
            post_action,
        }
    }
}

impl<F> TypeVisitor for TypeCollector<F>
where
    F: FnMut(Type),
{
    fn visit_array_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| {
            self.holder = Type::Array(Box::new(typ))
        }))
    }

    fn visit_end(&mut self) {
        (self.post_action)(self.holder.clone())
    }
}
