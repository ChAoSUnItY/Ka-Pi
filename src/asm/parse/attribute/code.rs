use nom::bytes::complete::take;
use nom::combinator::complete;
use nom::error::{make_error, ErrorKind};
use nom::multi::{count, many0};
use nom::number::complete::{be_i16, be_i32, be_i64, be_i8, be_u16, be_u32, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{IResult, Offset};

use crate::asm::node::attribute::{Attribute, Code};
use crate::asm::node::constant::ConstantPool;
use crate::asm::node::opcode::instruction::{
    ANewArray, CheckCast, GetField, GetStatic, InstanceOf, InvokeDynamic, InvokeInterface,
    InvokeSpecial, InvokeStatic, InvokeVirtual, Ldc, Ldc2_W, Ldc_W, MultiANewArray, New, PutField,
    PutStatic, Wide,
};
use crate::asm::node::opcode::{ArrayType, Instruction, Opcode};
use crate::asm::parse::attribute::{attribute_infos, exception};
use crate::asm::parse::collect;

pub(crate) fn code<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Option<Attribute>> {
    let (input, max_stack) = be_u16(input)?;
    let (input, max_locals) = be_u16(input)?;
    let (input, code_length) = be_u32(input)?;
    let (input, code) = take(code_length as usize)(input)?;
    let (input, (exception_table_length, exception_table)) = collect(be_u16, exception)(input)?;
    let (input, (attributes_length, attributes)) = attribute_infos(input, constant_pool)?;
    let (_, instructions) = instructions(code)?;

    Ok((
        input,
        Some(Attribute::Code(Code {
            max_stack,
            max_locals,
            code_length,
            code: code.to_vec(),
            instructions,
            exception_table_length,
            exception_table,
            attributes_length,
            attributes,
        })),
    ))
}

fn instructions(code: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
    many0(complete(move |input| {
        let (input, address) = offset(code, input)?;

        instruction(input, address)
    }))(code)
}

fn instruction(input: &[u8], address: usize) -> IResult<&[u8], Instruction> {
    let (input, opcode) = be_u8(input)?;
    let (input, instruction) = if let Ok(opcode) = Opcode::try_from(opcode) {
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
            Opcode::BIPUSH => {
                let (input, byte) = be_i8(input)?;

                (input, Instruction::BIPUSH(byte))
            }
            Opcode::SIPUSH => {
                let (input, byte) = be_i16(input)?;

                (input, Instruction::SIPUSH(byte))
            }
            Opcode::LDC => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::LDC(Ldc { index }))
            }
            Opcode::LDC_W => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::LDC_W(Ldc_W { index }))
            }
            Opcode::LDC2_W => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::LDC2_W(Ldc2_W { index }))
            }
            Opcode::ILOAD => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::ILOAD(index))
            }
            Opcode::LLOAD => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::LLOAD(index))
            }
            Opcode::FLOAD => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::FLOAD(index))
            }
            Opcode::DLOAD => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::DLOAD(index))
            }
            Opcode::ALOAD => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::ALOAD(index))
            }
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
                let (input, index) = be_u8(input)?;

                (input, Instruction::ISTORE(index))
            }
            Opcode::LSTORE => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::LSTORE(index))
            }
            Opcode::FSTORE => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::FSTORE(index))
            }
            Opcode::DSTORE => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::DSTORE(index))
            }
            Opcode::ASTORE => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::ASTORE(index))
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
            Opcode::IINC => {
                let (input, index) = be_u8(input)?;
                let (input, value) = be_i8(input)?;

                (input, Instruction::IINC { index, value })
            }
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
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFEQ(offset))
            }
            Opcode::IFNE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFNE(offset))
            }
            Opcode::IFLT => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFLT(offset))
            }
            Opcode::IFGE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFGE(offset))
            }
            Opcode::IFGT => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFGT(offset))
            }
            Opcode::IFLE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFLE(offset))
            }
            Opcode::IF_ICMPEQ => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPEQ(offset))
            }
            Opcode::IF_ICMPNE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPNE(offset))
            }
            Opcode::IF_ICMPLT => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPLT(offset))
            }
            Opcode::IF_ICMPGE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPGE(offset))
            }
            Opcode::IF_ICMPGT => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPGT(offset))
            }
            Opcode::IF_ICMPLE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ICMPLE(offset))
            }
            Opcode::IF_ACMPEQ => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ACMPEQ(offset))
            }
            Opcode::IF_ACMPNE => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IF_ACMPNE(offset))
            }
            Opcode::GOTO => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::GOTO(offset))
            }
            Opcode::JSR => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::JSR(offset))
            }
            Opcode::RET => {
                let (input, index) = be_u8(input)?;

                (input, Instruction::RET(index))
            }
            Opcode::TABLESWITCH => {
                let (input, _) = align(input, address + 1)?;
                let (input, default) = be_i32(input)?;
                let (input, low) = be_i32(input)?;
                let (input, high) = be_i32(input)?;
                let (input, offsets) = count(be_i32, (high - low + 1) as usize)(input)?;

                (
                    input,
                    Instruction::TABLESWITCH {
                        default,
                        low,
                        high,
                        offsets,
                    },
                )
            }
            Opcode::LOOKUPSWITCH => {
                let (input, _) = align(input, address + 1)?;
                let (input, default) = be_i32(input)?;
                let (input, npairs) = be_u32(input)?;
                let (input, pairs) = count(lookup_table_pair, npairs as usize)(input)?;

                (
                    input,
                    Instruction::LOOKUPSWITCH {
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
            Opcode::GETSTATIC => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::GETSTATIC(GetStatic { index }))
            }
            Opcode::PUTSTATIC => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::PUTSTATIC(PutStatic { index }))
            }
            Opcode::GETFIELD => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::GETFIELD(GetField { index }))
            }
            Opcode::PUTFIELD => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::PUTFIELD(PutField { index }))
            }
            Opcode::INVOKEVIRTUAL => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::INVOKEVIRTUAL(InvokeVirtual { index }))
            }
            Opcode::INVOKESPECIAL => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::INVOKESPECIAL(InvokeSpecial { index }))
            }
            Opcode::INVOKESTATIC => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::INVOKESTATIC(InvokeStatic { index }))
            }
            Opcode::INVOKEINTERFACE => {
                let (input, index) = be_u16(input)?;
                let (input, count) = be_u8(input)?;

                (
                    input,
                    Instruction::INVOKEINTERFACE(InvokeInterface { index, count }),
                )
            }
            Opcode::INVOKEDYNAMIC => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::INVOKEDYNAMIC(InvokeDynamic { index }))
            }
            Opcode::NEW => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::NEW(New { index }))
            }
            Opcode::NEWARRAY => {
                let (input, array_type) = be_u8(input)?;

                if let Ok(array_type) = ArrayType::try_from(array_type) {
                    (input, Instruction::NEWARRAY(array_type))
                } else {
                    return Err(Error(nom::error::Error {
                        input,
                        code: ErrorKind::NoneOf,
                    }));
                }
            }
            Opcode::ANEWARRAY => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::ANEWARRAY(ANewArray { index }))
            }
            Opcode::ARRAYLENGTH => (input, Instruction::ARRAYLENGTH),
            Opcode::ATHROW => (input, Instruction::ATHROW),
            Opcode::CHECKCAST => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::CHECKCAST(CheckCast { index }))
            }
            Opcode::INSTANCEOF => {
                let (input, index) = be_u16(input)?;

                (input, Instruction::INSTANCEOF(InstanceOf { index }))
            }
            Opcode::MONITORENTER => (input, Instruction::MONITORENTER),
            Opcode::MONITOREXIT => (input, Instruction::MONITOREXIT),
            Opcode::WIDE => {
                let (input, wide) = wide(input)?;

                (input, Instruction::WIDE(wide))
            }
            Opcode::MULTIANEWARRAY => {
                let (input, index) = be_u16(input)?;
                let (input, dimensions) = be_u8(input)?;

                (
                    input,
                    Instruction::MULTIANEWARRAY(MultiANewArray { index, dimensions }),
                )
            }
            Opcode::IFNULL => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFNULL(offset))
            }
            Opcode::IFNONNULL => {
                let (input, offset) = be_i16(input)?;

                (input, Instruction::IFNONNULL(offset))
            }
            Opcode::GOTO_W => {
                let (input, offset) = be_i64(input)?;

                (input, Instruction::GOTO_W(offset))
            }
            Opcode::JSR_W => {
                let (input, offset) = be_i64(input)?;

                (input, Instruction::JSR_W(offset))
            }
        }
    } else {
        // None of listed opcode described by Java SE 20 Specification
        return Err(Error(make_error(input, ErrorKind::NoneOf)));
    };

    Ok((input, instruction))
}

fn lookup_table_pair(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
    tuple((be_i32, be_i32))(input)
}

fn wide(input: &[u8]) -> IResult<&[u8], Wide> {
    let (input, widened_opcode) = be_u8(input)?;

    return if let Ok(widened_opcode) = Opcode::try_from(widened_opcode) {
        match widened_opcode {
            Opcode::ILOAD => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::ILOAD(index)))
            }
            Opcode::FLOAD => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::FLOAD(index)))
            }
            Opcode::ALOAD => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::ALOAD(index)))
            }
            Opcode::LLOAD => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::LLOAD(index)))
            }
            Opcode::DLOAD => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::DLOAD(index)))
            }
            Opcode::ISTORE => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::ISTORE(index)))
            }
            Opcode::FSTORE => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::FSTORE(index)))
            }
            Opcode::ASTORE => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::ASTORE(index)))
            }
            Opcode::LSTORE => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::LSTORE(index)))
            }
            Opcode::DSTORE => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::DSTORE(index)))
            }
            Opcode::RET => {
                let (input, index) = be_u16(input)?;

                Ok((input, Wide::RET(index)))
            }
            Opcode::IINC => {
                let (input, index) = be_u16(input)?;
                let (input, value) = be_i16(input)?;

                Ok((input, Wide::IINC(index, value)))
            }
            _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
        }
    } else {
        Err(Error(make_error(input, ErrorKind::NoneOf)))
    };
}

fn offset<'remain>(input: &[u8], remain: &'remain [u8]) -> IResult<&'remain [u8], usize> {
    Ok((remain, input.offset(remain)))
}

fn align(input: &[u8], address: usize) -> IResult<&[u8], &[u8]> {
    take((4 - address % 4) % 4)(input)
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::asm::node::opcode::instruction::Wide;
    use crate::asm::node::opcode::{Instruction, Opcode};
    use crate::asm::parse::attribute::code::instruction;

    #[test]
    fn test_alignment() {
        #[rustfmt::skip]
        let test_cases = vec![
            (3, vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31]),
            (0, vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31])
        ];
        let test_result = Ok((
            &[][..],
            Instruction::TABLESWITCH {
                default: 10,
                low: 20,
                high: 21,
                offsets: vec![30, 31],
            },
        ));

        for (address, input) in test_cases {
            assert_eq!(test_result, instruction(&input, address));
        }
    }

    #[test]
    fn test_wide_opcodes() {
        let test_cases = vec![
            (
                Instruction::WIDE(Wide::ILOAD(10)),
                vec![Opcode::WIDE as u8, Opcode::ILOAD as u8, 0, 10],
            ),
            (
                Instruction::WIDE(Wide::IINC(10, 20)),
                vec![Opcode::WIDE as u8, Opcode::IINC as u8, 0, 10, 0, 20],
            ),
        ];

        for (result_instruction, input) in test_cases {
            assert_eq!(Ok((&[][..], result_instruction)), instruction(&input, 0));
        }
    }

    #[test]
    fn test_invalid_opcode() {
        let test_cases = (0xCAu8..0xFFu8).map(|opcode| [opcode]).collect_vec();

        for test_case in test_cases {
            assert!(instruction(&test_case, 0).is_err());
        }
    }
}
