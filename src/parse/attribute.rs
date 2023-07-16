use nom::combinator::map;
use nom::error;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use nom::Err::Error;

use byte_span::{offset, BytesSpan};

use crate::node::attribute::{
    Attribute, AttributeInfo, ConstantValue, EnclosingMethod, Exceptions, ModuleMainClass,
    ModulePackages, NestHost, NestMembers, PermittedSubclasses, Signature, SourceDebugExtension,
    SourceFile,
};
use crate::node::constant::ConstantPool;
use crate::node::{attribute, Node, Nodes};
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
use crate::parse::{collect, map_node, node, take_node, ParseResult};

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

pub(crate) fn attribute_info<'fragment: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'fragment>,
    constant_pool: &'constant_pool ConstantPool,
) -> ParseResult<'fragment, Node<AttributeInfo>> {
    let (input, offset) = offset(input)?;
    let (input, attribute_name_index) = node(be_u16)(input)?;
    let (input, attribute_len) = node(be_u32)(input)?;
    let (input, info) = take_node(*attribute_len as usize)(input)?;
    let name_constant = constant_pool.get_utf8(*attribute_name_index);

    let attribute = if let Some(constant) = name_constant {
        if let Ok(attribute_name) = constant.string() {
            let (remain, attribute) = attribute(
                info.clone().into(),
                constant_pool,
                *attribute_len,
                &attribute_name,
            )?;

            if !remain.fragment.is_empty() {
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
        Node(
            offset..input.offset,
            AttributeInfo {
                attribute_name_index,
                attribute_len,
                info: info.map(<[u8]>::to_vec),
                attribute,
            },
        ),
    ))
}

fn attribute<'fragment: 'constant_pool, 'constant_pool: 'data, 'data>(
    mut input: BytesSpan<'fragment>,
    constant_pool: &'constant_pool ConstantPool,
    attribute_len: u32,
    data: &'data str,
) -> ParseResult<'fragment, Option<Node<Attribute>>> {
    let (input, attribute) = match data {
        attribute::CONSTANT_VALUE => map(
            node(be_u16),
            |Node(span, constant_value_index): Node<u16>| {
                Node(
                    span.clone(),
                    Attribute::ConstantValue(ConstantValue {
                        constant_value_index: Node(span, constant_value_index),
                    }),
                )
            },
        )(input)?,
        attribute::CODE => code(input, constant_pool)?,
        attribute::STACK_MAP_TABLE => stack_map_table(input)?,
        attribute::EXCEPTIONS => exceptions(input)?,
        attribute::INNER_CLASSES => inner_classes(input)?,
        attribute::ENCLOSING_METHOD => enclosing_method(input)?,
        attribute::SYNTHETIC => (
            input,
            Node(input.offset..input.offset, Attribute::Synthetic),
        ),
        attribute::SIGNATURE => signature(input)?,
        attribute::SOURCE_FILE => source_file(input)?,
        attribute::SOURCE_DEBUG_EXTENSION => source_debug_extension(input, attribute_len)?,
        attribute::LINE_NUMBER_TABLE => line_number_table(input)?,
        attribute::LOCAL_VARIABLE_TABLE => local_variable_table(input)?,
        attribute::LOCAL_VARIABLE_TYPE_TABLE => local_variable_type_table(input)?,
        attribute::DEPRECATED => (
            input,
            Node(input.offset..input.offset, Attribute::Deprecate),
        ),
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
        attribute::RECORD => record(input, constant_pool)?,
        attribute::PERMITTED_SUBCLASSES => permitted_subclasses(input)?,
        _ => return Ok((input.clear(), None)), // Discard input data to ignore unrecognized attribute
    };

    Ok((input, Some(attribute)))
}

fn exceptions(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), node(be_u16)),
        |(number_of_exceptions, exception_index_table): (Node<u16>, Nodes<u16>)| {
            Attribute::Exceptions(Exceptions {
                number_of_exceptions,
                exception_index_table,
            })
        },
    )(input)
}

fn enclosing_method(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        tuple((node(be_u16), node(be_u16))),
        |(class_index, method_index): (Node<u16>, Node<u16>)| {
            Attribute::EnclosingMethod(EnclosingMethod {
                class_index,
                method_index,
            })
        },
    )(input)
}

fn signature(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(node(be_u16), |signature_index: Node<u16>| {
        Attribute::Signature(Signature { signature_index })
    })(input)
}

fn source_file(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(node(be_u16), |source_file_index: Node<u16>| {
        Attribute::SourceFile(SourceFile { source_file_index })
    })(input)
}

fn source_debug_extension(input: BytesSpan, attribute_len: u32) -> ParseResult<Node<Attribute>> {
    map_node(take_node(attribute_len), |debug_extension: Node<&[u8]>| {
        Attribute::SourceDebugExtension(SourceDebugExtension {
            debug_extension: debug_extension.map(<[u8]>::to_vec),
        })
    })(input)
}

fn module_packages(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), node(be_u16)),
        |(package_count, package_index): (Node<u16>, Nodes<u16>)| {
            Attribute::ModulePackages(ModulePackages {
                package_count,
                package_index,
            })
        },
    )(input)
}

fn module_main_class(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(node(be_u16), |main_class_index| {
        Attribute::ModuleMainClass(ModuleMainClass { main_class_index })
    })(input)
}

fn nest_host(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(node(be_u16), |host_class_index: Node<u16>| {
        Attribute::NestHost(NestHost { host_class_index })
    })(input)
}

fn nest_members(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), node(be_u16)),
        |(number_of_classes, classes): (Node<u16>, Nodes<u16>)| {
            Attribute::NestMembers(NestMembers {
                number_of_classes,
                classes,
            })
        },
    )(input)
}

fn permitted_subclasses(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), node(be_u16)),
        |(number_of_classes, classes): (Node<u16>, Nodes<u16>)| {
            Attribute::PermittedSubclasses(PermittedSubclasses {
                number_of_classes,
                classes,
            })
        },
    )(input)
}
