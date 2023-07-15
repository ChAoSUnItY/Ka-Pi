use std::fs::File;
use std::io::Read;
use std::path::Path;

use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::{IResult, Parser, ToUsize};

use byte_span::{offset, BytesSpan};

use crate::error::{KapiError, KapiResult};
use crate::node::access_flag::AccessFlag;
use crate::node::class::Class;
use crate::node::constant::ConstantPool;
use crate::node::signature::Signature;
use crate::node::Node;

pub(crate) mod attribute;
pub(crate) mod class;
pub(crate) mod constant;
pub(crate) mod field;
pub(crate) mod method;
pub(crate) mod signature;
mod traits;

type ParseResult<'fragment, T, E = nom::error::Error<BytesSpan<'fragment>>> =
    IResult<BytesSpan<'fragment>, T, E>;

pub fn read_class<P: AsRef<Path>>(class_path: P) -> KapiResult<Class> {
    let class_path = class_path.as_ref();
    let mut file = match File::open(class_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(KapiError::ClassParseError(format!(
                "Unable to open class file {}, reason: {}",
                class_path.display(),
                err
            )))
        }
    };
    let mut class_bytes = Vec::new();

    if let Err(err) = file.read_to_end(&mut class_bytes) {
        return Err(KapiError::ClassParseError(format!(
            "Unable to read class file {}, reason: {}",
            class_path.display(),
            err
        )));
    }

    to_class(&class_bytes)
}

pub fn to_class(class_bytes: &[u8]) -> KapiResult<Class> {
    match class::class(BytesSpan::new(&class_bytes[..])) {
        Ok((remain, class)) => {
            if !remain.fragment.is_empty() {
                Err(KapiError::ClassParseError(format!("Unable to parse class bytes, reason: class is fully parsed but there are {} bytes left, {remain:?}", remain.len())))
            } else {
                Ok(class)
            }
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable parse class bytes, reason: {err}"
        ))),
    }
}

pub fn parse_class_signature(class_signature: &str) -> KapiResult<Signature> {
    match signature::class_signature(class_signature) {
        Ok((remain, signature)) => {
            if !remain.is_empty() {
                Err(KapiError::ClassParseError(format!("Unable to parse class signature, reason: signature is fully parsed but there are {} characters left, {remain:?}", remain.len())))
            } else {
                Ok(signature)
            }
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable to parse class signature, reason: {err}"
        ))),
    }
}

pub fn parse_field_signature(class_signature: &str) -> KapiResult<Signature> {
    match signature::field_signature(class_signature) {
        Ok((remain, signature)) => {
            if !remain.is_empty() {
                Err(KapiError::ClassParseError(format!("Unable to parse field signature, reason: signature is fully parsed but there are {} characters left, {remain:?}", remain.len())))
            } else {
                Ok(signature)
            }
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable to parse field signature, reason: {err}"
        ))),
    }
}

pub fn parse_method_signature(class_signature: &str) -> KapiResult<Signature> {
    match signature::method_signature(class_signature) {
        Ok((remain, signature)) => {
            if !remain.is_empty() {
                Err(KapiError::ClassParseError(format!("Unable to parse method signature, reason: signature is fully parsed but there are {} characters left, {remain:?}", remain.len())))
            } else {
                Ok(signature)
            }
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable to parse method signature, reason: {err}"
        ))),
    }
}

fn node<'fragment, V, F, E>(
    mut f: F,
) -> impl FnMut(BytesSpan<'fragment>) -> IResult<BytesSpan<'fragment>, Node<V>, E>
where
    F: Parser<BytesSpan<'fragment>, V, E>,
    nom::Err<E>: From<nom::Err<nom::error::Error<BytesSpan<'fragment>>>>,
{
    move |input: BytesSpan| {
        let (input, offset) = offset(input)?;
        let (input, v) = f.parse(input)?;

        Ok((input, Node(offset..input.offset, v)))
    }
}

fn map_node<'fragment, O1, O2, F, G, E>(
    mut f: F,
    mut g: G,
) -> impl FnMut(BytesSpan<'fragment>) -> IResult<BytesSpan<'fragment>, Node<O2>, E>
where
    F: Parser<BytesSpan<'fragment>, O1, E>,
    G: FnMut(O1) -> O2,
    nom::Err<E>: From<nom::Err<nom::error::Error<BytesSpan<'fragment>>>>,
{
    move |input: BytesSpan| {
        let (input, offset) = offset(input)?;
        let (input, o1) = f.parse(input)?;

        Ok((input, Node(offset..input.offset, g(o1))))
    }
}

fn take_node<'fragment>(
    count: impl ToUsize,
) -> impl FnMut(BytesSpan<'fragment>) -> IResult<BytesSpan<'fragment>, Node<&'fragment [u8]>> {
    map(take(count), Into::<Node<_>>::into)
}

fn take_sized_node<const SIZE: usize>() -> impl FnMut(BytesSpan) -> ParseResult<Node<[u8; SIZE]>> {
    move |input: BytesSpan| {
        let (mut input, offset) = offset(input)?;
        let mut container = [0u8; SIZE];

        for i in 0..SIZE {
            let (remain, byte) = take(1usize)(input)?;

            container[i] = byte.fragment[0];
            input = remain;
        }

        Ok((input, Node(offset..offset + SIZE, container)))
    }
}

fn tag_sized_node<const SIZE: usize>(
    sized_tag: [u8; SIZE],
) -> impl FnMut(BytesSpan) -> ParseResult<Node<[u8; SIZE]>> {
    move |input: BytesSpan| {
        let (input, tag) = tag(&sized_tag[..])(input)?;
        let mut container = [0u8; SIZE];

        for i in 0..SIZE {
            container[i] = tag.fragment[i];
        }

        Ok((input, Node(tag.range(), container)))
    }
}

fn collect<'fragment, L, T, LP, TP, E>(
    mut len_parser: LP,
    mut item_parser: TP,
) -> impl FnMut(BytesSpan<'fragment>) -> IResult<BytesSpan<'fragment>, (L, Node<Vec<T>>), E>
where
    L: ToUsize,
    LP: Parser<BytesSpan<'fragment>, L, E>,
    TP: Parser<BytesSpan<'fragment>, T, E>,
    nom::Err<E>: From<nom::Err<nom::error::Error<BytesSpan<'fragment>>>>,
{
    move |input| {
        let (input, len) = len_parser.parse(input)?;
        let (mut input, container_offset) = offset(input)?;
        let length = len.to_usize();
        let mut items = Vec::with_capacity(length);

        for _ in 0..length {
            let (remain, item) = item_parser.parse(input)?;

            items.push(item);
            input = remain;
        }

        Ok((input, (len, Node(container_offset..input.offset, items))))
    }
}

fn collect_with_constant_pool<'input: 'constant_pool, 'constant_pool, L, T, LP, TP>(
    mut len_parser: LP,
    mut item_parser: TP,
    constant_pool: &'constant_pool ConstantPool,
) -> impl FnMut(BytesSpan<'input>) -> IResult<BytesSpan<'input>, (L, Node<Vec<T>>)> + '_
where
    L: ToUsize,
    LP: FnMut(BytesSpan<'input>) -> IResult<BytesSpan<'input>, L> + 'constant_pool,
    TP: FnMut(BytesSpan<'input>, &'constant_pool ConstantPool) -> IResult<BytesSpan<'input>, T>
        + 'constant_pool,
{
    move |input: BytesSpan| {
        let (input, len) = len_parser(input)?;
        let (mut input, container_offset) = offset(input)?;
        let length = len.to_usize();
        let mut items = Vec::with_capacity(length);

        for _ in 0..length {
            let (remain, item) = item_parser(input, constant_pool)?;

            items.push(item);
            input = remain;
        }

        Ok((input, (len, Node(container_offset..input.offset, items))))
    }
}

fn access_flag<F>(input: BytesSpan) -> ParseResult<Node<Vec<F>>>
where
    F: AccessFlag,
{
    map(node(be_u16), |node: Node<u16>| node.map(F::extract_flags))(input)
}
