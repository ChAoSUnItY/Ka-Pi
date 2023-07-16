use nom::bytes::complete::take;
use nom::combinator::{complete, map};
use nom::error::{make_error, ErrorKind};
use nom::multi::{count, many0};
use nom::number::complete::{be_i16, be_i32, be_i64, be_i8, be_u16, be_u32, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::Offset;

use byte_span::{offset, BytesSpan};

use crate::node::attribute::{Attribute, Code, Exception};
use crate::node::constant::ConstantPool;
use crate::node::opcode::instruction::{
    ANewArray, CheckCast, GetField, GetStatic, InstanceOf, InvokeDynamic, InvokeInterface,
    InvokeSpecial, InvokeStatic, InvokeVirtual, Ldc, Ldc2_W, Ldc_W, MultiANewArray, New, PutField,
    PutStatic, Wide,
};
use crate::node::opcode::{ArrayType, Instruction, Opcode};
use crate::node::{Node, Nodes};
use crate::parse::attribute::attribute_info;
use crate::parse::{collect, collect_with_constant_pool, map_node, node, take_node, ParseResult};

pub(crate) fn code<'fragment: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'fragment>,
    constant_pool: &'constant_pool ConstantPool,
) -> ParseResult<'fragment, Node<Attribute>> {
    let (input, code_offset) = offset(input)?;
    let (input, max_stack) = node(be_u16)(input)?;
    let (input, max_locals) = node(be_u16)(input)?;
    let (input, code_length) = node(be_u32)(input)?;
    let (input, code) = take_node(*code_length as usize)(input)?;
    let (input, (exception_table_length, exception_table)) =
        collect(node(be_u16), exception)(input)?;
    let (input, (attributes_length, attributes)) =
        collect_with_constant_pool(node(be_u16), attribute_info, constant_pool)(input)?;
    let (_, instructions) = instructions(code.clone().into())?;

    Ok((
        input,
        Node(
            code_offset..input.offset,
            Attribute::Code(Code {
                max_stack,
                max_locals,
                code_length,
                code: code.map(<[u8]>::to_vec),
                instructions,
                exception_table_length,
                exception_table,
                attributes_length,
                attributes,
            }),
        ),
    ))
}

fn exception(input: BytesSpan) -> ParseResult<Node<Exception>> {
    map_node(
        tuple((node(be_u16), node(be_u16), node(be_u16), node(be_u16))),
        |(start_pc, end_pc, handler_pc, catch_type)| Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    )(input)
}

fn instructions<'fragment>(
    code: BytesSpan<'fragment>,
) -> ParseResult<'fragment, Nodes<Instruction>> {
    node(many0(complete(move |input: BytesSpan<'fragment>| {
        let (input, address) = opcode_offset(code, input)?;

        instruction(input, address)
    })))(code)
}

fn instruction(input: BytesSpan, address: usize) -> ParseResult<Node<Instruction>> {
    let (input, opcode_offset) = offset(input)?;
    let (input, opcode) = node(be_u8)(input)?;
    let (input, instruction) = if let Ok(opcode) = Opcode::try_from(*opcode) {
        match opcode {
            Opcode::NOP => (input, Instruction::NOP),
            Opcode::ACONST_NULL => (input, Instruction::ACONST_NULL),
            Opcode::ICONST_M1 => (input, Instruction::ICONST_M1),
            Opcode::ICONST_0 => (input, Instruction::ICONST_0),
            Opcode::ICONST_1 => (input, Instruction::ICONST_1),
            Opcode::ICONST_2 => (input, Instruction::ICONST_2),
            Opcode::ICONST_3 => (input, Instruction::ICONST_3),
            Opcode::ICONST_4 => (input, Instruction::ICONST_4),
            Opcode::ICONST_5 => (input, Instruction::ICONST_5),
            Opcode::LCONST_0 => (input, Instruction::LCONST_0),
            Opcode::LCONST_1 => (input, Instruction::LCONST_1),
            Opcode::FCONST_0 => (input, Instruction::FCONST_0),
            Opcode::FCONST_1 => (input, Instruction::FCONST_1),
            Opcode::FCONST_2 => (input, Instruction::FCONST_2),
            Opcode::DCONST_0 => (input, Instruction::DCONST_0),
            Opcode::DCONST_1 => (input, Instruction::DCONST_1),
            Opcode::BIPUSH => map(node(be_i8), |byte: Node<i8>| Instruction::BIPUSH(byte))(input)?,
            Opcode::SIPUSH => {
                map(node(be_i16), |byte: Node<i16>| Instruction::SIPUSH(byte))(input)?
            }
            Opcode::LDC => map(node(be_u8), |index: Node<u8>| {
                Instruction::LDC(Ldc { index })
            })(input)?,
            Opcode::LDC_W => map(node(be_u16), |index: Node<u16>| {
                Instruction::LDC_W(Ldc_W { index })
            })(input)?,
            Opcode::LDC2_W => map(node(be_u16), |index: Node<u16>| {
                Instruction::LDC2_W(Ldc2_W { index })
            })(input)?,
            Opcode::ILOAD => map(node(be_u8), |index: Node<u8>| Instruction::ILOAD(index))(input)?,
            Opcode::LLOAD => map(node(be_u8), |index: Node<u8>| Instruction::LLOAD(index))(input)?,
            Opcode::FLOAD => map(node(be_u8), |index: Node<u8>| Instruction::FLOAD(index))(input)?,
            Opcode::DLOAD => map(node(be_u8), |index: Node<u8>| Instruction::DLOAD(index))(input)?,
            Opcode::ALOAD => map(node(be_u8), |index: Node<u8>| Instruction::ALOAD(index))(input)?,
            Opcode::ILOAD_0 => (input, Instruction::ILOAD_0),
            Opcode::ILOAD_1 => (input, Instruction::ILOAD_1),
            Opcode::ILOAD_2 => (input, Instruction::ILOAD_2),
            Opcode::ILOAD_3 => (input, Instruction::ILOAD_3),
            Opcode::LLOAD_0 => (input, Instruction::LLOAD_0),
            Opcode::LLOAD_1 => (input, Instruction::LLOAD_1),
            Opcode::LLOAD_2 => (input, Instruction::LLOAD_2),
            Opcode::LLOAD_3 => (input, Instruction::LLOAD_3),
            Opcode::FLOAD_0 => (input, Instruction::FLOAD_0),
            Opcode::FLOAD_1 => (input, Instruction::FLOAD_1),
            Opcode::FLOAD_2 => (input, Instruction::FLOAD_2),
            Opcode::FLOAD_3 => (input, Instruction::FLOAD_3),
            Opcode::DLOAD_0 => (input, Instruction::DLOAD_0),
            Opcode::DLOAD_1 => (input, Instruction::DLOAD_1),
            Opcode::DLOAD_2 => (input, Instruction::DLOAD_2),
            Opcode::DLOAD_3 => (input, Instruction::DLOAD_3),
            Opcode::ALOAD_0 => (input, Instruction::ALOAD_0),
            Opcode::ALOAD_1 => (input, Instruction::ALOAD_1),
            Opcode::ALOAD_2 => (input, Instruction::ALOAD_2),
            Opcode::ALOAD_3 => (input, Instruction::ALOAD_3),
            Opcode::IALOAD => (input, Instruction::IALOAD),
            Opcode::LALOAD => (input, Instruction::LALOAD),
            Opcode::FALOAD => (input, Instruction::FALOAD),
            Opcode::DALOAD => (input, Instruction::DALOAD),
            Opcode::AALOAD => (input, Instruction::AALOAD),
            Opcode::BALOAD => (input, Instruction::BALOAD),
            Opcode::CALOAD => (input, Instruction::CALOAD),
            Opcode::SALOAD => (input, Instruction::SALOAD),
            Opcode::ISTORE => {
                map(node(be_u8), |index: Node<u8>| Instruction::ISTORE(index))(input)?
            }
            Opcode::LSTORE => {
                map(node(be_u8), |index: Node<u8>| Instruction::LSTORE(index))(input)?
            }
            Opcode::FSTORE => {
                map(node(be_u8), |index: Node<u8>| Instruction::FSTORE(index))(input)?
            }
            Opcode::DSTORE => {
                map(node(be_u8), |index: Node<u8>| Instruction::DSTORE(index))(input)?
            }
            Opcode::ASTORE => {
                map(node(be_u8), |index: Node<u8>| Instruction::ASTORE(index))(input)?
            }
            Opcode::ISTORE_0 => (input, Instruction::ISTORE_0),
            Opcode::ISTORE_1 => (input, Instruction::ISTORE_1),
            Opcode::ISTORE_2 => (input, Instruction::ISTORE_2),
            Opcode::ISTORE_3 => (input, Instruction::ISTORE_3),
            Opcode::LSTORE_0 => (input, Instruction::LSTORE_0),
            Opcode::LSTORE_1 => (input, Instruction::LSTORE_1),
            Opcode::LSTORE_2 => (input, Instruction::LSTORE_2),
            Opcode::LSTORE_3 => (input, Instruction::LSTORE_3),
            Opcode::FSTORE_0 => (input, Instruction::FSTORE_0),
            Opcode::FSTORE_1 => (input, Instruction::FSTORE_1),
            Opcode::FSTORE_2 => (input, Instruction::FSTORE_2),
            Opcode::FSTORE_3 => (input, Instruction::FSTORE_3),
            Opcode::DSTORE_0 => (input, Instruction::DSTORE_0),
            Opcode::DSTORE_1 => (input, Instruction::DSTORE_1),
            Opcode::DSTORE_2 => (input, Instruction::DSTORE_2),
            Opcode::DSTORE_3 => (input, Instruction::DSTORE_3),
            Opcode::ASTORE_0 => (input, Instruction::ASTORE_0),
            Opcode::ASTORE_1 => (input, Instruction::ASTORE_1),
            Opcode::ASTORE_2 => (input, Instruction::ASTORE_2),
            Opcode::ASTORE_3 => (input, Instruction::ASTORE_3),
            Opcode::IASTORE => (input, Instruction::IASTORE),
            Opcode::LASTORE => (input, Instruction::LASTORE),
            Opcode::FASTORE => (input, Instruction::FASTORE),
            Opcode::DASTORE => (input, Instruction::DASTORE),
            Opcode::AASTORE => (input, Instruction::AASTORE),
            Opcode::BASTORE => (input, Instruction::BASTORE),
            Opcode::CASTORE => (input, Instruction::CASTORE),
            Opcode::SASTORE => (input, Instruction::SASTORE),
            Opcode::POP => (input, Instruction::POP),
            Opcode::POP2 => (input, Instruction::POP2),
            Opcode::DUP => (input, Instruction::DUP),
            Opcode::DUP_X1 => (input, Instruction::DUP_X1),
            Opcode::DUP_X2 => (input, Instruction::DUP_X2),
            Opcode::DUP2 => (input, Instruction::DUP2),
            Opcode::DUP2_X1 => (input, Instruction::DUP_X1),
            Opcode::DUP2_X2 => (input, Instruction::DUP2_X2),
            Opcode::SWAP => (input, Instruction::SWAP),
            Opcode::IADD => (input, Instruction::IADD),
            Opcode::LADD => (input, Instruction::LADD),
            Opcode::FADD => (input, Instruction::FADD),
            Opcode::DADD => (input, Instruction::DADD),
            Opcode::ISUB => (input, Instruction::ISUB),
            Opcode::LSUB => (input, Instruction::LSUB),
            Opcode::FSUB => (input, Instruction::FSUB),
            Opcode::DSUB => (input, Instruction::DSUB),
            Opcode::IMUL => (input, Instruction::IMUL),
            Opcode::LMUL => (input, Instruction::LMUL),
            Opcode::FMUL => (input, Instruction::FMUL),
            Opcode::DMUL => (input, Instruction::DMUL),
            Opcode::IDIV => (input, Instruction::IDIV),
            Opcode::LDIV => (input, Instruction::LDIV),
            Opcode::FDIV => (input, Instruction::FDIV),
            Opcode::DDIV => (input, Instruction::DDIV),
            Opcode::IREM => (input, Instruction::IREM),
            Opcode::LREM => (input, Instruction::LREM),
            Opcode::FREM => (input, Instruction::FREM),
            Opcode::DREM => (input, Instruction::DREM),
            Opcode::INEG => (input, Instruction::INEG),
            Opcode::LNEG => (input, Instruction::LNEG),
            Opcode::FNEG => (input, Instruction::FNEG),
            Opcode::DNEG => (input, Instruction::DNEG),
            Opcode::ISHL => (input, Instruction::ISHL),
            Opcode::LSHL => (input, Instruction::LSHL),
            Opcode::ISHR => (input, Instruction::ISHR),
            Opcode::LSHR => (input, Instruction::LSHR),
            Opcode::IUSHR => (input, Instruction::IUSHR),
            Opcode::LUSHR => (input, Instruction::LUSHR),
            Opcode::IAND => (input, Instruction::IAND),
            Opcode::LAND => (input, Instruction::LAND),
            Opcode::IOR => (input, Instruction::IOR),
            Opcode::LOR => (input, Instruction::LOR),
            Opcode::IXOR => (input, Instruction::IXOR),
            Opcode::LXOR => (input, Instruction::LXOR),
            Opcode::IINC => map(
                tuple((node(be_u8), node(be_i8))),
                |(index, value): (Node<u8>, Node<i8>)| Instruction::IINC { index, value },
            )(input)?,
            Opcode::I2L => (input, Instruction::I2L),
            Opcode::I2F => (input, Instruction::I2F),
            Opcode::I2D => (input, Instruction::I2D),
            Opcode::L2I => (input, Instruction::L2I),
            Opcode::L2F => (input, Instruction::L2F),
            Opcode::L2D => (input, Instruction::L2D),
            Opcode::F2I => (input, Instruction::F2I),
            Opcode::F2L => (input, Instruction::F2L),
            Opcode::F2D => (input, Instruction::F2D),
            Opcode::D2I => (input, Instruction::D2I),
            Opcode::D2L => (input, Instruction::D2L),
            Opcode::D2F => (input, Instruction::D2L),
            Opcode::I2B => (input, Instruction::I2B),
            Opcode::I2C => (input, Instruction::I2C),
            Opcode::I2S => (input, Instruction::I2S),
            Opcode::LCMP => (input, Instruction::LCMP),
            Opcode::FCMPL => (input, Instruction::FCMPL),
            Opcode::FCMPG => (input, Instruction::DCMPG),
            Opcode::DCMPL => (input, Instruction::DCMPL),
            Opcode::DCMPG => (input, Instruction::DCMPG),
            Opcode::IFEQ => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFEQ(offset))(input)?
            }
            Opcode::IFNE => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFNE(offset))(input)?
            }
            Opcode::IFLT => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFLT(offset))(input)?
            }
            Opcode::IFGE => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFGE(offset))(input)?
            }
            Opcode::IFGT => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFGT(offset))(input)?
            }
            Opcode::IFLE => {
                map(node(be_i16), |offset: Node<i16>| Instruction::IFLE(offset))(input)?
            }
            Opcode::IF_ICMPEQ => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPEQ(offset)
            })(input)?,
            Opcode::IF_ICMPNE => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPNE(offset)
            })(input)?,
            Opcode::IF_ICMPLT => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPLT(offset)
            })(input)?,
            Opcode::IF_ICMPGE => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPGE(offset)
            })(input)?,
            Opcode::IF_ICMPGT => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPGT(offset)
            })(input)?,
            Opcode::IF_ICMPLE => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ICMPLE(offset)
            })(input)?,
            Opcode::IF_ACMPEQ => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ACMPEQ(offset)
            })(input)?,
            Opcode::IF_ACMPNE => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IF_ACMPNE(offset)
            })(input)?,
            Opcode::GOTO => {
                map(node(be_i16), |offset: Node<i16>| Instruction::GOTO(offset))(input)?
            }
            Opcode::JSR => map(node(be_i16), |offset: Node<i16>| Instruction::JSR(offset))(input)?,
            Opcode::RET => map(node(be_u8), |index: Node<u8>| Instruction::RET(index))(input)?,
            Opcode::TABLESWITCH => {
                let (input, alignment) = align(input, address + 1)?;
                let (input, default) = node(be_i32)(input)?;
                let (input, low) = node(be_i32)(input)?;
                let (input, high) = node(be_i32)(input)?;
                let (input, offsets) =
                    node(count(node(be_i32), (*high - *low + 1) as usize))(input)?;

                (
                    input,
                    Instruction::TABLESWITCH {
                        alignment: Into::<Node<&[u8]>>::into(alignment).map(<&[u8]>::into),
                        default,
                        low,
                        high,
                        offsets,
                    },
                )
            }
            Opcode::LOOKUPSWITCH => {
                let (input, alignment) = align(input, address + 1)?;
                let (input, default) = node(be_i32)(input)?;
                let (input, npairs) = node(be_u32)(input)?;
                let (input, pairs) = node(count(lookup_table_pair, *npairs as usize))(input)?;

                (
                    input,
                    Instruction::LOOKUPSWITCH {
                        alignment: Into::<Node<&[u8]>>::into(alignment).map(<&[u8]>::into),
                        default,
                        npairs,
                        pairs,
                    },
                )
            }
            Opcode::IRETURN => (input, Instruction::IRETURN),
            Opcode::LRETURN => (input, Instruction::LRETURN),
            Opcode::FRETURN => (input, Instruction::FRETURN),
            Opcode::DRETURN => (input, Instruction::DRETURN),
            Opcode::ARETURN => (input, Instruction::ARETURN),
            Opcode::RETURN => (input, Instruction::RETURN),
            Opcode::GETSTATIC => map(node(be_u16), |index: Node<u16>| {
                Instruction::GETSTATIC(GetStatic { index })
            })(input)?,
            Opcode::PUTSTATIC => map(node(be_u16), |index: Node<u16>| {
                Instruction::PUTSTATIC(PutStatic { index })
            })(input)?,
            Opcode::GETFIELD => map(node(be_u16), |index: Node<u16>| {
                Instruction::GETFIELD(GetField { index })
            })(input)?,
            Opcode::PUTFIELD => map(node(be_u16), |index: Node<u16>| {
                Instruction::PUTFIELD(PutField { index })
            })(input)?,
            Opcode::INVOKEVIRTUAL => map(node(be_u16), |index: Node<u16>| {
                Instruction::INVOKEVIRTUAL(InvokeVirtual { index })
            })(input)?,
            Opcode::INVOKESPECIAL => map(node(be_u16), |index: Node<u16>| {
                Instruction::INVOKESPECIAL(InvokeSpecial { index })
            })(input)?,
            Opcode::INVOKESTATIC => map(node(be_u16), |index: Node<u16>| {
                Instruction::INVOKESTATIC(InvokeStatic { index })
            })(input)?,
            Opcode::INVOKEINTERFACE => map(
                tuple((node(be_u16), node(be_u8))),
                |(index, count): (Node<u16>, Node<u8>)| {
                    Instruction::INVOKEINTERFACE(InvokeInterface { index, count })
                },
            )(input)?,
            Opcode::INVOKEDYNAMIC => map(node(be_u16), |index: Node<u16>| {
                Instruction::INVOKEDYNAMIC(InvokeDynamic { index })
            })(input)?,
            Opcode::NEW => map(node(be_u16), |index: Node<u16>| {
                Instruction::NEW(New { index })
            })(input)?,
            Opcode::NEWARRAY => {
                let (input, array_type) = node(be_u8)(input)?;

                if let Ok(array_type) = ArrayType::try_from(*array_type) {
                    (input, Instruction::NEWARRAY(array_type))
                } else {
                    return Err(Error(nom::error::Error {
                        input,
                        code: ErrorKind::NoneOf,
                    }));
                }
            }
            Opcode::ANEWARRAY => map(node(be_u16), |index: Node<u16>| {
                Instruction::ANEWARRAY(ANewArray { index })
            })(input)?,
            Opcode::ARRAYLENGTH => (input, Instruction::ARRAYLENGTH),
            Opcode::ATHROW => (input, Instruction::ATHROW),
            Opcode::CHECKCAST => map(node(be_u16), |index: Node<u16>| {
                Instruction::CHECKCAST(CheckCast { index })
            })(input)?,
            Opcode::INSTANCEOF => map(node(be_u16), |index: Node<u16>| {
                Instruction::INSTANCEOF(InstanceOf { index })
            })(input)?,
            Opcode::MONITORENTER => (input, Instruction::MONITORENTER),
            Opcode::MONITOREXIT => (input, Instruction::MONITOREXIT),
            Opcode::WIDE => map(wide, |wide: Wide| Instruction::WIDE(wide))(input)?,
            Opcode::MULTIANEWARRAY => map(
                tuple((node(be_u16), node(be_u8))),
                |(index, dimensions): (Node<u16>, Node<u8>)| {
                    Instruction::MULTIANEWARRAY(MultiANewArray { index, dimensions })
                },
            )(input)?,
            Opcode::IFNULL => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IFNULL(offset)
            })(input)?,
            Opcode::IFNONNULL => map(node(be_i16), |offset: Node<i16>| {
                Instruction::IFNONNULL(offset)
            })(input)?,
            Opcode::GOTO_W => map(node(be_i64), |offset: Node<i64>| {
                Instruction::GOTO_W(offset)
            })(input)?,
            Opcode::JSR_W => {
                map(node(be_i64), |offset: Node<i64>| Instruction::JSR_W(offset))(input)?
            }
        }
    } else {
        // None of listed opcode described by Java SE 20 Specification
        return Err(Error(make_error(input, ErrorKind::NoneOf)));
    };

    Ok((input, Node(opcode_offset..input.offset, instruction)))
}

fn lookup_table_pair(input: BytesSpan) -> ParseResult<Node<(Node<i32>, Node<i32>)>> {
    node(tuple((node(be_i32), node(be_i32))))(input)
}

fn wide(input: BytesSpan) -> ParseResult<Wide> {
    let (input, widened_opcode) = be_u8(input)?;

    return if let Ok(widened_opcode) = Opcode::try_from(widened_opcode) {
        match widened_opcode {
            Opcode::ILOAD => map(node(be_u16), |index: Node<u16>| Wide::ILOAD(index))(input),
            Opcode::FLOAD => map(node(be_u16), |index: Node<u16>| Wide::FLOAD(index))(input),
            Opcode::ALOAD => map(node(be_u16), |index: Node<u16>| Wide::ALOAD(index))(input),
            Opcode::LLOAD => map(node(be_u16), |index: Node<u16>| Wide::LLOAD(index))(input),
            Opcode::DLOAD => map(node(be_u16), |index: Node<u16>| Wide::DLOAD(index))(input),
            Opcode::ISTORE => map(node(be_u16), |index: Node<u16>| Wide::ISTORE(index))(input),
            Opcode::FSTORE => map(node(be_u16), |index: Node<u16>| Wide::FSTORE(index))(input),
            Opcode::ASTORE => map(node(be_u16), |index: Node<u16>| Wide::ASTORE(index))(input),
            Opcode::LSTORE => map(node(be_u16), |index: Node<u16>| Wide::LSTORE(index))(input),
            Opcode::DSTORE => map(node(be_u16), |index: Node<u16>| Wide::DSTORE(index))(input),
            Opcode::RET => map(node(be_u16), |index: Node<u16>| Wide::RET(index))(input),
            Opcode::IINC => map(
                tuple((node(be_u16), node(be_i16))),
                |(index, value): (Node<u16>, Node<i16>)| Wide::IINC(index, value),
            )(input),
            _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
        }
    } else {
        Err(Error(make_error(input, ErrorKind::NoneOf)))
    };
}

fn opcode_offset<'remain>(
    input: BytesSpan,
    remain: BytesSpan<'remain>,
) -> ParseResult<'remain, usize> {
    Ok((remain, input.offset(&remain)))
}

fn align(input: BytesSpan, address: usize) -> ParseResult<BytesSpan> {
    take((4 - address % 4) % 4)(input)
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use byte_span::BytesSpan;

    use crate::node::opcode::instruction::Wide;
    use crate::node::opcode::{Instruction, Opcode};
    use crate::node::Node;
    use crate::parse::attribute::code::instruction;

    #[test]
    fn test_alignment() {
        #[rustfmt::skip]
        let test_cases = vec![
            vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31],
            vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31],
        ];
        let test_result1 = Ok((
            BytesSpan::with_offset(21, &[][..]),
            Node(
                0..21,
                Instruction::TABLESWITCH {
                    alignment: Node(1..1, vec![].into_boxed_slice()),
                    default: Node(1..5, 10),
                    low: Node(5..9, 20),
                    high: Node(9..13, 21),
                    offsets: Node(13..21, vec![Node(13..17, 30), Node(17..21, 31)]),
                },
            ),
        ));
        let test_result2 = Ok((
            BytesSpan::with_offset(24, &[][..]),
            Node(
                0..24,
                Instruction::TABLESWITCH {
                    alignment: Node(1..4, vec![0, 0, 0].into_boxed_slice()),
                    default: Node(4..8, 10),
                    low: Node(8..12, 20),
                    high: Node(12..16, 21),
                    offsets: Node(16..24, vec![Node(16..20, 30), Node(20..24, 31)]),
                },
            ),
        ));

        assert_eq!(
            test_result1,
            instruction(BytesSpan::new(&test_cases[0][..]), 3)
        );
        assert_eq!(
            test_result2,
            instruction(BytesSpan::new(&test_cases[1][..]), 0)
        );
    }

    #[test]
    fn test_wide_opcodes() {
        let test_cases = vec![
            vec![Opcode::WIDE as u8, Opcode::ILOAD as u8, 0, 10],
            vec![Opcode::WIDE as u8, Opcode::IINC as u8, 0, 10, 0, 20],
        ];

        assert_eq!(
            Ok((
                BytesSpan::with_offset(4, &[][..]),
                Node(0..4, Instruction::WIDE(Wide::ILOAD(Node(2..4, 10))))
            )),
            instruction(BytesSpan::new(&test_cases[0][..]), 0)
        );
        assert_eq!(
            Ok((
                BytesSpan::with_offset(6, &[][..]),
                Node(
                    0..6,
                    Instruction::WIDE(Wide::IINC(Node(2..4, 10), Node(4..6, 20)))
                )
            )),
            instruction(BytesSpan::new(&test_cases[1][..]), 0)
        );
    }

    #[test]
    fn test_invalid_opcode() {
        let test_cases = (0xCAu8..0xFFu8).map(|opcode| [opcode]).collect_vec();

        for test_case in test_cases {
            assert!(instruction(BytesSpan::new(&test_case[..]), 0).is_err());
        }
    }
}
