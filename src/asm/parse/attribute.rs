use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32};
use nom::number::streaming::be_u8;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{error, IResult};

use crate::asm::node::attribute;
use crate::asm::node::attribute::{
    Attribute, AttributeInfo, ConstantValue, EnclosingMethod, Exceptions, ModuleMainClass,
    ModulePackages, NestHost, NestMembers, PermittedSubclasses, Signature, SourceDebugExtension,
    SourceFile,
};
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::parse::attribute::annotation::{
    annotation_default, runtime_invisible_annotations, runtime_invisible_parameter_annotations,
    runtime_invisible_type_annotations, runtime_visible_annotations,
    runtime_visible_parameter_annotations, runtime_visible_type_annotations,
};
use crate::asm::parse::attribute::bootstrap_methods::bootstrap_methods;
use crate::asm::parse::attribute::code::code;
use crate::asm::parse::attribute::inner_classes::inner_classes;
use crate::asm::parse::attribute::line_number_table::line_number_table;
use crate::asm::parse::attribute::local_variable_table::local_variable_table;
use crate::asm::parse::attribute::local_variable_type_table::local_variable_type_table;
use crate::asm::parse::attribute::method_parameters::method_parameters;
use crate::asm::parse::attribute::module::module;
use crate::asm::parse::attribute::stack_map_table::stack_map_table;
use crate::asm::parse::collect;

mod annotation;
mod bootstrap_methods;
mod code;
mod inner_classes;
mod line_number_table;
mod local_variable_table;
mod local_variable_type_table;
mod method_parameters;
mod module;
mod record;
mod stack_map_table;

pub(crate) fn attribute_info<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], AttributeInfo> {
    let (input, attribute_name_index) = be_u16(input)?;
    let (input, attribute_len) = be_u32(input)?;
    let (input, info) = take(attribute_len as usize)(input)?;
    let name_constant = constant_pool.get(attribute_name_index);

    let attribute = if let Some(Constant::Utf8(constant)) = name_constant {
        if let Ok(attribute_name) = constant.string() {
            let (remain, attribute) =
                attribute(info, constant_pool, attribute_len, &attribute_name)?;

            if !remain.is_empty() {
                return Err(Error(error::Error::new(remain, ErrorKind::NonEmpty)));
            }

            attribute
        } else {
            None
        }
    } else {
        None
    };

    Ok((
        input,
        AttributeInfo {
            attribute_name_index,
            attribute_len,
            info: info.to_vec(),
            attribute,
        },
    ))
}

fn attribute<'input: 'constant_pool, 'constant_pool: 'data, 'data>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
    attribute_len: u32,
    data: &'data str,
) -> IResult<&'input [u8], Option<Attribute>> {
    match data {
        attribute::CONSTANT_VALUE => map(be_u16, |constant_value_index| {
            Some(Attribute::ConstantValue(ConstantValue {
                constant_value_index,
            }))
        })(input),
        attribute::CODE => code(input, constant_pool),
        attribute::STACK_MAP_TABLE => stack_map_table(input),
        attribute::EXCEPTIONS => exceptions(input),
        attribute::INNER_CLASSES => inner_classes(input),
        attribute::ENCLOSING_METHOD => enclosing_method(input),
        attribute::SYNTHETIC => Ok((&[], Some(Attribute::Synthetic))),
        attribute::SOURCE_FILE => source_file(input),
        attribute::SOURCE_DEBUG_EXTENSION => source_debug_extension(input, attribute_len),
        attribute::LINE_NUMBER_TABLE => line_number_table(input),
        attribute::LOCAL_VARIABLE_TABLE => local_variable_table(input),
        attribute::LOCAL_VARIABLE_TYPE_TABLE => local_variable_type_table(input),
        attribute::DEPRECATED => Ok((&[], Some(Attribute::Deprecate))),
        attribute::RUNTIME_VISIBLE_ANNOTATIONS => runtime_visible_annotations(input),
        attribute::RUNTIME_INVISIBLE_ANNOTATIONS => runtime_invisible_annotations(input),
        attribute::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
            runtime_visible_parameter_annotations(input)
        }
        attribute::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
            runtime_invisible_parameter_annotations(input)
        }
        attribute::RUNTIME_VISIBLE_TYPE_ANNOTATIONS => runtime_visible_type_annotations(input),
        attribute::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => runtime_invisible_type_annotations(input),
        attribute::ANNOTATION_DEFAULT => annotation_default(input),
        attribute::BOOTSTRAP_METHODS => bootstrap_methods(input),
        attribute::METHOD_PARAMETERS => method_parameters(input),
        attribute::MODULE => module(input),
        attribute::MODULE_PACKAGES => module_packages(input),
        attribute::MODULE_MAIN_CLASS => module_main_class(input),
        attribute::NEST_HOST => nest_host(input),
        attribute::NEST_MEMBERS => nest_members(input),
        attribute::PERMITTED_SUBCLASSES => permitted_subclasses(input),
        _ => Ok((&[], None)), // Discard input data to ignore unrecognized attribute
    }
}

fn exceptions(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, be_u16),
        |(number_of_exceptions, exception_index_table)| {
            Some(Attribute::Exceptions(Exceptions {
                number_of_exceptions,
                exception_index_table,
            }))
        },
    )(input)
}

fn enclosing_method(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(tuple((be_u16, be_u16)), |(class_index, method_index)| {
        Some(Attribute::EnclosingMethod(EnclosingMethod {
            class_index,
            method_index,
        }))
    })(input)
}

fn signature(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |signature_index| {
        Some(Attribute::Signature(Signature { signature_index }))
    })(input)
}

fn source_file(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |source_file_index| {
        Some(Attribute::SourceFile(SourceFile { source_file_index }))
    })(input)
}

fn source_debug_extension(input: &[u8], attribute_len: u32) -> IResult<&[u8], Option<Attribute>> {
    map(count(be_u8, attribute_len as usize), |debug_extension| {
        Some(Attribute::SourceDebugExtension(SourceDebugExtension {
            debug_extension,
        }))
    })(input)
}

fn module_packages(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(collect(be_u16, be_u16), |(package_count, package_index)| {
        Some(Attribute::ModulePackages(ModulePackages {
            package_count,
            package_index,
        }))
    })(input)
}

fn module_main_class(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |main_class_index| {
        Some(Attribute::ModuleMainClass(ModuleMainClass {
            main_class_index,
        }))
    })(input)
}

fn nest_host(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |host_class_index| {
        Some(Attribute::NestHost(NestHost { host_class_index }))
    })(input)
}

fn nest_members(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(collect(be_u16, be_u16), |(number_of_classes, classes)| {
        Some(Attribute::NestMembers(NestMembers {
            number_of_classes,
            classes,
        }))
    })(input)
}

fn permitted_subclasses(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(collect(be_u16, be_u16), |(number_of_classes, classes)| {
        Some(Attribute::PermittedSubclasses(PermittedSubclasses {
            number_of_classes,
            classes,
        }))
    })(input)
}
