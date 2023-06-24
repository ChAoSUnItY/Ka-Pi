use nom::IResult;
use crate::asm::node::signature::Signature;
use crate::asm::node::types::Type;
use crate::error::KapiResult;

pub(crate) fn parse_class_signature() -> KapiResult<Signature> {
    
}

pub(crate) fn java_type_signature(input: &[char]) -> IResult<&[char], Type> {
    
}

pub(crate) fn base_type(input: &[char]) -> IResult<&[char], Ba>
