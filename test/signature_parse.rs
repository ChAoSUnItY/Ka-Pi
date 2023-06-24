use insta::assert_yaml_snapshot;

use ka_pi::asm::parse::{parse_class_signature, parse_field_signature, parse_method_signature};
use ka_pi::asm::visitor::signature::{FormalTypeParameterVisitor, SignatureVisitor, TypeVisitor};
use ka_pi::asm::visitor::Visitable;
use ka_pi::error::KapiResult;

#[test]
fn test_class_signature_with_generic() -> KapiResult<()> {
    let class_signature =
        parse_class_signature("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")?;

    assert_yaml_snapshot!(class_signature);

    Ok(())
}

#[test]
fn test_field_signature_object() -> KapiResult<()> {
    let field_signature = parse_field_signature("Ljava/lang/Object;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_field_signature_type_variable() -> KapiResult<()> {
    let field_signature = parse_field_signature("TT;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_method_signature_with_generic() -> KapiResult<()> {
    let method_signature = parse_method_signature(
        "<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;",
    )?;

    assert_yaml_snapshot!(method_signature);

    Ok(())
}

// Visitor test
// Example usage of visitor: Remap specific type argument name.

#[derive(Default)]
struct Visitor {}

impl SignatureVisitor for Visitor {
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

    fn visit_formal_type_parameter(&mut self, name: &mut String) -> Self::FTPV {
        if name == "T" {
            *name = "V".to_string();
        } else if name == "R" {
            *name = "K".to_string();
        }

        Self::default()
    }
}

impl FormalTypeParameterVisitor for Visitor {
    type TV = Self;

    fn visit_class_bound(&mut self) -> Self::TV {
        Self::default()
    }

    fn visit_interface_bound(&mut self) -> Self::TV {
        Self::default()
    }
}

impl TypeVisitor for Visitor {}

#[test]
fn test_signature_visitor() -> KapiResult<()> {
    let mut class_signature =
        parse_class_signature("<T:Ljava/lang/Object;R:>Ljava/lang/Object;Ljava/lang/Runnable;")?;

    class_signature.visit(&mut Visitor::default());

    assert_yaml_snapshot!(class_signature);

    Ok(())
}
