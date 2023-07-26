use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::attribute;
use crate::node::attribute::{
    Attribute, AttributeInfo, ConstantValue, EnclosingMethod, Exceptions, ModuleMainClass,
    ModulePackages, NestHost, NestMembers, PermittedSubclasses, Signature, SourceDebugExtension,
    SourceFile,
};
use crate::node::constant::ConstantPool;
use crate::parse::attribute::annotation::{
    annotation_default, runtime_invisible_annotations, runtime_invisible_parameter_annotations,
    runtime_invisible_type_annotations, runtime_visible_annotations,
    runtime_visible_parameter_annotations, runtime_visible_type_annotations,
};
use crate::parse::attribute::bootstrap_methods::bootstrap_methods;
use crate::parse::attribute::code::code;
use crate::parse::attribute::inner_classes::inner_classes;
use crate::parse::attribute::line_number_table::line_number_table;
use crate::parse::attribute::local_variable_table::local_variable_table;
use crate::parse::attribute::local_variable_type_table::local_variable_type_table;
use crate::parse::attribute::method_parameters::method_parameters;
use crate::parse::attribute::module::module;
use crate::parse::attribute::record::record;
use crate::parse::attribute::stack_map_table::stack_map_table;
use crate::parse::error::{ParseError, ParseResult};
use crate::parse::ParsingOption;

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

pub(super) fn attribute_info<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<AttributeInfo> {
    let attribute_name_index = input.read_u16::<BigEndian>()?;
    let attribute_len = input.read_u32::<BigEndian>()?;
    let mut info = vec![0; attribute_len as usize];

    input.read_exact(&mut info)?;

    let attribute = if option.parse_attribute {
        if let Some(Ok(attribute_name)) = constant_pool
            .get_utf8(attribute_name_index)
            .map(|utf8| utf8.string())
        {
            let mut info = Cursor::new(&mut info[..]);
            let attribute = attribute(
                &mut info,
                constant_pool,
                attribute_len,
                &attribute_name,
                option,
            )?;

            let mut remain = vec![];
            info.read_to_end(&mut remain)?;

            if !remain.is_empty() {
                return Err(ParseError::Remains(remain.len()));
            }

            attribute
        } else {
            None
        }
    } else {
        None
    };

    Ok(AttributeInfo {
        attribute_name_index,
        attribute_len,
        info,
        attribute,
    })
}

fn attribute<'input: 'constant_pool, 'constant_pool: 'data, 'data, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    attribute_len: u32,
    data: &'data str,
    option: &ParsingOption,
) -> ParseResult<Option<Attribute>> {
    let attribute = match data {
        attribute::CONSTANT_VALUE => constant_value(input)?,
        attribute::CODE => code(input, constant_pool, option)?,
        attribute::STACK_MAP_TABLE => stack_map_table(input)?,
        attribute::EXCEPTIONS => exceptions(input)?,
        attribute::INNER_CLASSES => inner_classes(input)?,
        attribute::ENCLOSING_METHOD => enclosing_method(input)?,
        attribute::SYNTHETIC => Some(Attribute::Synthetic),
        attribute::SIGNATURE => signature(input)?,
        attribute::SOURCE_FILE => source_file(input)?,
        attribute::SOURCE_DEBUG_EXTENSION => source_debug_extension(input, attribute_len)?,
        attribute::LINE_NUMBER_TABLE => line_number_table(input)?,
        attribute::LOCAL_VARIABLE_TABLE => local_variable_table(input)?,
        attribute::LOCAL_VARIABLE_TYPE_TABLE => local_variable_type_table(input)?,
        attribute::DEPRECATED => Some(Attribute::Deprecate),
        attribute::RUNTIME_VISIBLE_ANNOTATIONS => runtime_visible_annotations(input)?,
        attribute::RUNTIME_INVISIBLE_ANNOTATIONS => runtime_invisible_annotations(input)?,
        attribute::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
            runtime_visible_parameter_annotations(input)?
        }
        attribute::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
            runtime_invisible_parameter_annotations(input)?
        }
        attribute::RUNTIME_VISIBLE_TYPE_ANNOTATIONS => runtime_visible_type_annotations(input)?,
        attribute::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => runtime_invisible_type_annotations(input)?,
        attribute::ANNOTATION_DEFAULT => annotation_default(input)?,
        attribute::BOOTSTRAP_METHODS => bootstrap_methods(input)?,
        attribute::METHOD_PARAMETERS => method_parameters(input)?,
        attribute::MODULE => module(input)?,
        attribute::MODULE_PACKAGES => module_packages(input)?,
        attribute::MODULE_MAIN_CLASS => module_main_class(input)?,
        attribute::NEST_HOST => nest_host(input)?,
        attribute::NEST_MEMBERS => nest_members(input)?,
        attribute::RECORD => record(input, constant_pool, option)?,
        attribute::PERMITTED_SUBCLASSES => permitted_subclasses(input)?,
        _ => None,
    };

    Ok(attribute)
}

#[inline]
fn constant_value<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let constant_value_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::ConstantValue(ConstantValue {
        constant_value_index,
    })))
}

#[inline]
fn exceptions<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let number_of_exceptions = input.read_u16::<BigEndian>()?;
    let mut exception_index_table = vec![0; number_of_exceptions as usize];

    for i in 0..number_of_exceptions {
        exception_index_table[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Some(Attribute::Exceptions(Exceptions {
        number_of_exceptions,
        exception_index_table,
    })))
}

#[inline]
fn enclosing_method<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let class_index = input.read_u16::<BigEndian>()?;
    let method_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::EnclosingMethod(EnclosingMethod {
        class_index,
        method_index,
    })))
}

#[inline]
fn signature<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let signature_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::Signature(Signature { signature_index })))
}

#[inline]
fn source_file<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let source_file_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::SourceFile(SourceFile {
        source_file_index,
    })))
}

#[inline]
fn source_debug_extension<R: Read>(
    input: &mut R,
    attribute_len: u32,
) -> ParseResult<Option<Attribute>> {
    let mut debug_extension = vec![0; attribute_len as usize];

    input.read_exact(&mut debug_extension)?;

    Ok(Some(Attribute::SourceDebugExtension(
        SourceDebugExtension { debug_extension },
    )))
}

#[inline]
fn module_packages<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let package_count = input.read_u16::<BigEndian>()?;
    let mut package_index = vec![0; package_count as usize];

    for i in 0..package_count {
        package_index[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Some(Attribute::ModulePackages(ModulePackages {
        package_count,
        package_index,
    })))
}

#[inline]
fn module_main_class<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let main_class_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::ModuleMainClass(ModuleMainClass {
        main_class_index,
    })))
}

#[inline]
fn nest_host<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let host_class_index = input.read_u16::<BigEndian>()?;

    Ok(Some(Attribute::NestHost(NestHost { host_class_index })))
}

#[inline]
fn nest_members<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let number_of_classes = input.read_u16::<BigEndian>()?;
    let mut classes = vec![0; number_of_classes as usize];

    for i in 0..number_of_classes {
        classes[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Some(Attribute::NestMembers(NestMembers {
        number_of_classes,
        classes,
    })))
}

#[inline]
fn permitted_subclasses<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let number_of_classes = input.read_u16::<BigEndian>()?;
    let mut classes = vec![0; number_of_classes as usize];

    for i in 0..number_of_classes {
        classes[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Some(Attribute::PermittedSubclasses(PermittedSubclasses {
        number_of_classes,
        classes,
    })))
}
