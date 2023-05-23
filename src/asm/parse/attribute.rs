use nom::{error, IResult};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::Err::Error;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use crate::asm::node::attribute;

use crate::asm::node::attribute::{Attribute, AttributeInfo, Exception};
use crate::asm::node::constant::{Constant, ConstantPool};

pub(crate) fn attribute_infos<'input: 'constant_pool, 'constant_pool>(input: &'input [u8], constant_pool: &'constant_pool ConstantPool) -> IResult<&'input [u8], (u16, Vec<AttributeInfo>)> {
    let (mut input, len) = be_u16(input)?;
    let mut attributes = Vec::with_capacity(len as usize);
    
    for _ in 0..len {
        let (remain, attribute) = attribute_info(input, constant_pool)?;
        
        attributes.push(attribute);
        input = remain;
    }
    
    Ok((input, (len, attributes)))
}

fn attribute_info<'input: 'constant_pool, 'constant_pool>(input: &'input [u8], constant_pool : &'constant_pool ConstantPool) -> IResult<&'input [u8], AttributeInfo> {
    let (input, attribute_name_index) = be_u16(input)?;
    let (input, attribute_len) = be_u32(input)?;
    let (input, info) = take(attribute_len as usize)(input)?;
    let name_constant = constant_pool.get(attribute_name_index as usize);
    
    let attribute = if let Some(constant) = name_constant {
        if let Constant::Utf8 { data } = constant {
            let (remain, attribute) = attribute(info, constant_pool, data)?;
            
            if !remain.is_empty() {
                return Err(Error(error::Error::new(
                    remain,
                    ErrorKind::NonEmpty
                )));
            }
            
            attribute
        } else {
            None
        }
    } else {
        None
    };
    
    Ok((input, AttributeInfo {
        attribute_name_index,
        attribute_len,
        info: info.to_vec(),
        attribute
    }))
}

fn attribute<'input: 'data, 'constant_pool, 'data>(input: &'input[u8], constant_pool: &'constant_pool ConstantPool, data: &'data str) -> IResult<&'input [u8], Option<Attribute>> {
    match data {
        attribute::CONSTANT_VALUE => {
            map(be_u16, |constant_value_index| Some(Attribute::ConstantValue {
                constant_value_index
            }))(input)
        }
        attribute::CODE => {
            let (input, max_stack) = be_u16(input)?;
            let (input, max_locals) = be_u16(input)?;
            let (input, code_length) = be_u32(input)?;
            let (input, code) = take(code_length as usize)(input)?;
            let (mut input, exception_table_length) = be_u16(input)?;
            let mut exception_table = Vec::with_capacity(exception_table_length as usize);
            
            for _ in 0..exception_table_length {
                let (remain, exception) = exception(input)?;

                exception_table.push(exception);
                input = remain;
            }
            
            let (input, (attributes_length, attributes)) = attribute_infos(input, constant_pool)?;
            
            Ok((input, Some(Attribute::Code {
                max_stack,
                max_locals,
                code_length,
                code: code.to_vec(),
                exception_table_length,
                exception_table,
                attributes_length,
                attributes
            })))
        }
        _ => Ok((&[], None)) // Discard input data to ignore unrecognized attribute
    }
}

fn exception(input: &[u8]) -> IResult<&[u8], Exception> {
    map(tuple((be_u16, be_u16, be_u16, be_u16)), |(start_pc, end_pc, handler_pc, catch_type)| Exception {
        start_pc,
        end_pc,
        handler_pc,
        catch_type
    })(input)
}
