use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::attribute::{Attribute, Code, Exception};
use crate::node::constant::ConstantPool;
use crate::node::opcode::instruction::{
    ANewArray, CheckCast, GetField, GetStatic, InstanceOf, InvokeDynamic, InvokeInterface,
    InvokeSpecial, InvokeStatic, InvokeVirtual, Ldc, Ldc2_W, Ldc_W, MultiANewArray, New, PutField,
    PutStatic, Wide,
};
use crate::node::opcode::{ArrayType, Instruction, Opcode};
use crate::parse::attribute::attribute_info;
use crate::parse::error::{ParseError, ParseResult};
use crate::parse::ParsingOption;

pub(super) fn code<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<Option<Attribute>> {
    let max_stack = input.read_u16::<BigEndian>()?;
    let max_locals = input.read_u16::<BigEndian>()?;
    let code_length = input.read_u32::<BigEndian>()?;
    let mut code = vec![0; code_length as usize];

    input.read_exact(&mut code)?;

    let exception_table_length = input.read_u16::<BigEndian>()?;
    let mut exception_table = Vec::with_capacity(exception_table_length as usize);

    for _ in 0..exception_table_length {
        exception_table.push(exception(input)?);
    }

    let attributes_length = input.read_u16::<BigEndian>()?;
    let mut attributes = Vec::with_capacity(attributes_length as usize);

    for _ in 0..attributes_length {
        attributes.push(attribute_info(input, constant_pool, option)?);
    }

    let instructions = instructions(Cursor::new(&mut code.clone()), code_length)?;

    Ok(Some(Attribute::Code(Code {
        max_stack,
        max_locals,
        code_length,
        code,
        instructions,
        exception_table_length,
        exception_table,
        attributes_length,
        attributes,
    })))
}

#[inline]
fn exception<R: Read>(input: &mut R) -> ParseResult<Exception> {
    let start_pc = input.read_u16::<BigEndian>()?;
    let end_pc = input.read_u16::<BigEndian>()?;
    let handler_pc = input.read_u16::<BigEndian>()?;
    let catch_type = input.read_u16::<BigEndian>()?;

    Ok(Exception {
        start_pc,
        end_pc,
        handler_pc,
        catch_type,
    })
}

#[inline]
fn instructions(mut input: Cursor<&mut [u8]>, length: u32) -> ParseResult<Vec<Instruction>> {
    let mut pc = 0;
    let mut instructions = Vec::new();

    while pc < length {
        instructions.push(instruction(&mut input, &mut pc)?);
    }

    Ok(instructions)
}

fn instruction<R: Read + Seek>(input: &mut R, pc: &mut u32) -> ParseResult<Instruction> {
    let opcode = input.read_u8()?;

    *pc += 1;

    if let Ok(opcode) = Opcode::try_from(opcode) {
        let instruction = match opcode {
            Opcode::NOP => Instruction::NOP,
            Opcode::ACONST_NULL => Instruction::ACONST_NULL,
            Opcode::ICONST_M1 => Instruction::ICONST_M1,
            Opcode::ICONST_0 => Instruction::ICONST_0,
            Opcode::ICONST_1 => Instruction::ICONST_1,
            Opcode::ICONST_2 => Instruction::ICONST_2,
            Opcode::ICONST_3 => Instruction::ICONST_3,
            Opcode::ICONST_4 => Instruction::ICONST_4,
            Opcode::ICONST_5 => Instruction::ICONST_5,
            Opcode::LCONST_0 => Instruction::LCONST_0,
            Opcode::LCONST_1 => Instruction::LCONST_1,
            Opcode::FCONST_0 => Instruction::FCONST_0,
            Opcode::FCONST_1 => Instruction::FCONST_1,
            Opcode::FCONST_2 => Instruction::FCONST_2,
            Opcode::DCONST_0 => Instruction::DCONST_0,
            Opcode::DCONST_1 => Instruction::DCONST_1,
            Opcode::BIPUSH => {
                let byte = input.read_i8()?;

                *pc += 1;

                Instruction::BIPUSH(byte)
            }
            Opcode::SIPUSH => {
                let byte = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::SIPUSH(byte)
            }
            Opcode::LDC => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::LDC(Ldc { index })
            }
            Opcode::LDC_W => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::LDC_W(Ldc_W { index })
            }
            Opcode::LDC2_W => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::LDC2_W(Ldc2_W { index })
            }
            Opcode::ILOAD => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::ILOAD(index)
            }
            Opcode::LLOAD => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::LLOAD(index)
            }
            Opcode::FLOAD => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::FLOAD(index)
            }
            Opcode::DLOAD => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::DLOAD(index)
            }
            Opcode::ALOAD => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::ALOAD(index)
            }
            Opcode::ILOAD_0 => Instruction::ILOAD_0,
            Opcode::ILOAD_1 => Instruction::ILOAD_1,
            Opcode::ILOAD_2 => Instruction::ILOAD_2,
            Opcode::ILOAD_3 => Instruction::ILOAD_3,
            Opcode::LLOAD_0 => Instruction::LLOAD_0,
            Opcode::LLOAD_1 => Instruction::LLOAD_1,
            Opcode::LLOAD_2 => Instruction::LLOAD_2,
            Opcode::LLOAD_3 => Instruction::LLOAD_3,
            Opcode::FLOAD_0 => Instruction::FLOAD_0,
            Opcode::FLOAD_1 => Instruction::FLOAD_1,
            Opcode::FLOAD_2 => Instruction::FLOAD_2,
            Opcode::FLOAD_3 => Instruction::FLOAD_3,
            Opcode::DLOAD_0 => Instruction::DLOAD_0,
            Opcode::DLOAD_1 => Instruction::DLOAD_1,
            Opcode::DLOAD_2 => Instruction::DLOAD_2,
            Opcode::DLOAD_3 => Instruction::DLOAD_3,
            Opcode::ALOAD_0 => Instruction::ALOAD_0,
            Opcode::ALOAD_1 => Instruction::ALOAD_1,
            Opcode::ALOAD_2 => Instruction::ALOAD_2,
            Opcode::ALOAD_3 => Instruction::ALOAD_3,
            Opcode::IALOAD => Instruction::IALOAD,
            Opcode::LALOAD => Instruction::LALOAD,
            Opcode::FALOAD => Instruction::FALOAD,
            Opcode::DALOAD => Instruction::DALOAD,
            Opcode::AALOAD => Instruction::AALOAD,
            Opcode::BALOAD => Instruction::BALOAD,
            Opcode::CALOAD => Instruction::CALOAD,
            Opcode::SALOAD => Instruction::SALOAD,
            Opcode::ISTORE => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::ISTORE(index)
            }
            Opcode::LSTORE => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::LSTORE(index)
            }
            Opcode::FSTORE => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::FSTORE(index)
            }
            Opcode::DSTORE => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::DSTORE(index)
            }
            Opcode::ASTORE => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::ASTORE(index)
            }
            Opcode::ISTORE_0 => Instruction::ISTORE_0,
            Opcode::ISTORE_1 => Instruction::ISTORE_1,
            Opcode::ISTORE_2 => Instruction::ISTORE_2,
            Opcode::ISTORE_3 => Instruction::ISTORE_3,
            Opcode::LSTORE_0 => Instruction::LSTORE_0,
            Opcode::LSTORE_1 => Instruction::LSTORE_1,
            Opcode::LSTORE_2 => Instruction::LSTORE_2,
            Opcode::LSTORE_3 => Instruction::LSTORE_3,
            Opcode::FSTORE_0 => Instruction::FSTORE_0,
            Opcode::FSTORE_1 => Instruction::FSTORE_1,
            Opcode::FSTORE_2 => Instruction::FSTORE_2,
            Opcode::FSTORE_3 => Instruction::FSTORE_3,
            Opcode::DSTORE_0 => Instruction::DSTORE_0,
            Opcode::DSTORE_1 => Instruction::DSTORE_1,
            Opcode::DSTORE_2 => Instruction::DSTORE_2,
            Opcode::DSTORE_3 => Instruction::DSTORE_3,
            Opcode::ASTORE_0 => Instruction::ASTORE_0,
            Opcode::ASTORE_1 => Instruction::ASTORE_1,
            Opcode::ASTORE_2 => Instruction::ASTORE_2,
            Opcode::ASTORE_3 => Instruction::ASTORE_3,
            Opcode::IASTORE => Instruction::IASTORE,
            Opcode::LASTORE => Instruction::LASTORE,
            Opcode::FASTORE => Instruction::FASTORE,
            Opcode::DASTORE => Instruction::DASTORE,
            Opcode::AASTORE => Instruction::AASTORE,
            Opcode::BASTORE => Instruction::BASTORE,
            Opcode::CASTORE => Instruction::CASTORE,
            Opcode::SASTORE => Instruction::SASTORE,
            Opcode::POP => Instruction::POP,
            Opcode::POP2 => Instruction::POP2,
            Opcode::DUP => Instruction::DUP,
            Opcode::DUP_X1 => Instruction::DUP_X1,
            Opcode::DUP_X2 => Instruction::DUP_X2,
            Opcode::DUP2 => Instruction::DUP2,
            Opcode::DUP2_X1 => Instruction::DUP_X1,
            Opcode::DUP2_X2 => Instruction::DUP2_X2,
            Opcode::SWAP => Instruction::SWAP,
            Opcode::IADD => Instruction::IADD,
            Opcode::LADD => Instruction::LADD,
            Opcode::FADD => Instruction::FADD,
            Opcode::DADD => Instruction::DADD,
            Opcode::ISUB => Instruction::ISUB,
            Opcode::LSUB => Instruction::LSUB,
            Opcode::FSUB => Instruction::FSUB,
            Opcode::DSUB => Instruction::DSUB,
            Opcode::IMUL => Instruction::IMUL,
            Opcode::LMUL => Instruction::LMUL,
            Opcode::FMUL => Instruction::FMUL,
            Opcode::DMUL => Instruction::DMUL,
            Opcode::IDIV => Instruction::IDIV,
            Opcode::LDIV => Instruction::LDIV,
            Opcode::FDIV => Instruction::FDIV,
            Opcode::DDIV => Instruction::DDIV,
            Opcode::IREM => Instruction::IREM,
            Opcode::LREM => Instruction::LREM,
            Opcode::FREM => Instruction::FREM,
            Opcode::DREM => Instruction::DREM,
            Opcode::INEG => Instruction::INEG,
            Opcode::LNEG => Instruction::LNEG,
            Opcode::FNEG => Instruction::FNEG,
            Opcode::DNEG => Instruction::DNEG,
            Opcode::ISHL => Instruction::ISHL,
            Opcode::LSHL => Instruction::LSHL,
            Opcode::ISHR => Instruction::ISHR,
            Opcode::LSHR => Instruction::LSHR,
            Opcode::IUSHR => Instruction::IUSHR,
            Opcode::LUSHR => Instruction::LUSHR,
            Opcode::IAND => Instruction::IAND,
            Opcode::LAND => Instruction::LAND,
            Opcode::IOR => Instruction::IOR,
            Opcode::LOR => Instruction::LOR,
            Opcode::IXOR => Instruction::IXOR,
            Opcode::LXOR => Instruction::LXOR,
            Opcode::IINC => {
                let index = input.read_u8()?;
                let value = input.read_i8()?;

                *pc += 2;

                Instruction::IINC { index, value }
            }
            Opcode::I2L => Instruction::I2L,
            Opcode::I2F => Instruction::I2F,
            Opcode::I2D => Instruction::I2D,
            Opcode::L2I => Instruction::L2I,
            Opcode::L2F => Instruction::L2F,
            Opcode::L2D => Instruction::L2D,
            Opcode::F2I => Instruction::F2I,
            Opcode::F2L => Instruction::F2L,
            Opcode::F2D => Instruction::F2D,
            Opcode::D2I => Instruction::D2I,
            Opcode::D2L => Instruction::D2L,
            Opcode::D2F => Instruction::D2L,
            Opcode::I2B => Instruction::I2B,
            Opcode::I2C => Instruction::I2C,
            Opcode::I2S => Instruction::I2S,
            Opcode::LCMP => Instruction::LCMP,
            Opcode::FCMPL => Instruction::FCMPL,
            Opcode::FCMPG => Instruction::DCMPG,
            Opcode::DCMPL => Instruction::DCMPL,
            Opcode::DCMPG => Instruction::DCMPG,
            Opcode::IFEQ => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFEQ(offset)
            }
            Opcode::IFNE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFNE(offset)
            }
            Opcode::IFLT => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFLT(offset)
            }
            Opcode::IFGE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFGE(offset)
            }
            Opcode::IFGT => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFGT(offset)
            }
            Opcode::IFLE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFLE(offset)
            }
            Opcode::IF_ICMPEQ => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPEQ(offset)
            }
            Opcode::IF_ICMPNE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPNE(offset)
            }
            Opcode::IF_ICMPLT => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPLT(offset)
            }
            Opcode::IF_ICMPGE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPGE(offset)
            }
            Opcode::IF_ICMPGT => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPGT(offset)
            }
            Opcode::IF_ICMPLE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ICMPLE(offset)
            }
            Opcode::IF_ACMPEQ => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ACMPEQ(offset)
            }
            Opcode::IF_ACMPNE => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IF_ACMPNE(offset)
            }
            Opcode::GOTO => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::GOTO(offset)
            }
            Opcode::JSR => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::JSR(offset)
            }
            Opcode::RET => {
                let index = input.read_u8()?;

                *pc += 1;

                Instruction::RET(index)
            }
            Opcode::TABLESWITCH => {
                let pad = align(input, *pc)?;
                let default = input.read_i32::<BigEndian>()?;
                let low = input.read_i32::<BigEndian>()?;
                let high = input.read_i32::<BigEndian>()?;
                let offsets_length = (high - low + 1) as usize;
                let mut offsets = vec![0; offsets_length];

                for i in 0..offsets_length {
                    offsets[i] = input.read_i32::<BigEndian>()?;
                }

                *pc += pad + 12 + (high - low + 1) as u32;

                Instruction::TABLESWITCH {
                    default,
                    low,
                    high,
                    offsets,
                }
            }
            Opcode::LOOKUPSWITCH => {
                let pad = align(input, *pc)?;
                let default = input.read_i32::<BigEndian>()?;
                let npairs = input.read_u32::<BigEndian>()?;
                let mut pairs = Vec::with_capacity(npairs as usize);

                for _ in 0..npairs {
                    pairs.push(lookup_table_pair(input)?);
                }

                *pc += pad + 8 + npairs * 8;

                Instruction::LOOKUPSWITCH {
                    default,
                    npairs,
                    pairs,
                }
            }
            Opcode::IRETURN => Instruction::IRETURN,
            Opcode::LRETURN => Instruction::LRETURN,
            Opcode::FRETURN => Instruction::FRETURN,
            Opcode::DRETURN => Instruction::DRETURN,
            Opcode::ARETURN => Instruction::ARETURN,
            Opcode::RETURN => Instruction::RETURN,
            Opcode::GETSTATIC => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::GETSTATIC(GetStatic { index })
            }
            Opcode::PUTSTATIC => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::PUTSTATIC(PutStatic { index })
            }
            Opcode::GETFIELD => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::GETFIELD(GetField { index })
            }
            Opcode::PUTFIELD => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::PUTFIELD(PutField { index })
            }
            Opcode::INVOKEVIRTUAL => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::INVOKEVIRTUAL(InvokeVirtual { index })
            }
            Opcode::INVOKESPECIAL => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::INVOKESPECIAL(InvokeSpecial { index })
            }
            Opcode::INVOKESTATIC => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::INVOKESTATIC(InvokeStatic { index })
            }
            Opcode::INVOKEINTERFACE => {
                let index = input.read_u16::<BigEndian>()?;
                let count = input.read_u8()?;

                *pc += 3;

                Instruction::INVOKEINTERFACE(InvokeInterface { index, count })
            }
            Opcode::INVOKEDYNAMIC => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::INVOKEDYNAMIC(InvokeDynamic { index })
            }
            Opcode::NEW => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::NEW(New { index })
            }
            Opcode::NEWARRAY => {
                let array_type = input.read_u8()?;

                if let Ok(array_type) = ArrayType::try_from(array_type) {
                    Instruction::NEWARRAY(array_type)
                } else {
                    return Err(ParseError::MatchOutOfBoundUsize(
                        "array type",
                        vec!["4..=11"],
                        array_type as usize,
                    ));
                }
            }
            Opcode::ANEWARRAY => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::ANEWARRAY(ANewArray { index })
            }
            Opcode::ARRAYLENGTH => Instruction::ARRAYLENGTH,
            Opcode::ATHROW => Instruction::ATHROW,
            Opcode::CHECKCAST => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::CHECKCAST(CheckCast { index })
            }
            Opcode::INSTANCEOF => {
                let index = input.read_u16::<BigEndian>()?;

                *pc += 2;

                Instruction::INSTANCEOF(InstanceOf { index })
            }
            Opcode::MONITORENTER => Instruction::MONITORENTER,
            Opcode::MONITOREXIT => Instruction::MONITOREXIT,
            Opcode::WIDE => {
                let wide = wide(input, pc)?;

                Instruction::WIDE(wide)
            }
            Opcode::MULTIANEWARRAY => {
                let index = input.read_u16::<BigEndian>()?;
                let dimensions = input.read_u8()?;

                *pc += 3;

                Instruction::MULTIANEWARRAY(MultiANewArray { index, dimensions })
            }
            Opcode::IFNULL => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFNULL(offset)
            }
            Opcode::IFNONNULL => {
                let offset = input.read_i16::<BigEndian>()?;

                *pc += 2;

                Instruction::IFNONNULL(offset)
            }
            Opcode::GOTO_W => {
                let offset = input.read_i64::<BigEndian>()?;

                *pc += 4;

                Instruction::GOTO_W(offset)
            }
            Opcode::JSR_W => {
                let offset = input.read_i64::<BigEndian>()?;

                *pc += 4;

                Instruction::JSR_W(offset)
            }
        };

        Ok(instruction)
    } else {
        // None of listed opcode described by Java SE 20 Specification
        Err(ParseError::MatchOutOfBoundOpcode(opcode))
    }
}

#[inline]
fn lookup_table_pair<R: Read>(input: &mut R) -> ParseResult<(i32, i32)> {
    let left = input.read_i32::<BigEndian>()?;
    let right = input.read_i32::<BigEndian>()?;

    Ok((left, right))
}

fn wide<R: Read>(input: &mut R, pc: &mut u32) -> ParseResult<Wide> {
    let widened_opcode = input.read_u8()?;

    return if let Ok(widened_opcode) = Opcode::try_from(widened_opcode) {
        let wide = match widened_opcode {
            Opcode::ILOAD => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::ILOAD(index)
            }
            Opcode::FLOAD => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::FLOAD(index)
            }
            Opcode::ALOAD => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::ALOAD(index)
            }
            Opcode::LLOAD => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::LLOAD(index)
            }
            Opcode::DLOAD => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::DLOAD(index)
            }
            Opcode::ISTORE => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::ISTORE(index)
            }
            Opcode::FSTORE => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::FSTORE(index)
            }
            Opcode::ASTORE => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::ASTORE(index)
            }
            Opcode::LSTORE => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::LSTORE(index)
            }
            Opcode::DSTORE => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::DSTORE(index)
            }
            Opcode::RET => {
                let index = input.read_u16::<BigEndian>()?;

                Wide::RET(index)
            }
            Opcode::IINC => {
                let index = input.read_u16::<BigEndian>()?;
                let value = input.read_i16::<BigEndian>()?;

                Wide::IINC(index, value)
            }
            _ => {
                return Err(ParseError::MatchOutOfBoundWideOpcode(
                    widened_opcode as u8,
                    vec![
                        Opcode::ILOAD,
                        Opcode::FLOAD,
                        Opcode::ALOAD,
                        Opcode::LLOAD,
                        Opcode::DLOAD,
                        Opcode::ISTORE,
                        Opcode::FSTORE,
                        Opcode::ASTORE,
                        Opcode::LSTORE,
                        Opcode::DSTORE,
                        Opcode::RET,
                        Opcode::IINC,
                    ],
                ))
            }
        };

        *pc += if matches!(wide, Wide::IINC(_, _)) {
            9
        } else {
            7
        };

        Ok(wide)
    } else {
        Err(ParseError::MatchOutOfBoundWideOpcode(
            widened_opcode,
            vec![
                Opcode::ILOAD,
                Opcode::FLOAD,
                Opcode::ALOAD,
                Opcode::LLOAD,
                Opcode::DLOAD,
                Opcode::ISTORE,
                Opcode::FSTORE,
                Opcode::ASTORE,
                Opcode::LSTORE,
                Opcode::DSTORE,
                Opcode::RET,
                Opcode::IINC,
            ],
        ))
    };
}

#[inline]
fn align<R: Read + Seek>(input: &mut R, pc: u32) -> ParseResult<u32> {
    let pad = (4 - pc % 4) % 4;

    match input.seek(SeekFrom::Current(pad as i64)) {
        Ok(_) => Ok(pad),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::node::opcode::instruction::Wide;
    use crate::node::opcode::{Instruction, Opcode};
    use crate::parse::attribute::code::instruction;

    #[test]
    fn test_alignment() {
        #[rustfmt::skip]
        let test_cases = vec![
            (3u32, vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31]),
            (0u32, vec![Opcode::TABLESWITCH as u8, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 30, 0, 0, 0, 31])
        ];

        for (address, input) in test_cases {
            let mut pc = address;
            let result = instruction(&mut Cursor::new(&input), &mut pc);

            assert!(result.is_ok());
            assert_eq!(
                Instruction::TABLESWITCH {
                    default: 10,
                    low: 20,
                    high: 21,
                    offsets: vec![30, 31],
                },
                result.unwrap()
            );
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
            let mut pc = 0;
            let result = instruction(&mut Cursor::new(&input), &mut pc);

            assert!(result.is_ok());
            assert_eq!(result_instruction, result.unwrap());
        }
    }

    #[test]
    fn test_invalid_opcode() {
        let test_cases = (0xCAu8..0xFFu8).map(|opcode| [opcode]).collect::<Vec<_>>();

        for test_case in test_cases {
            let mut pc = 0;

            assert!(instruction(&mut Cursor::new(test_case), &mut pc).is_err());
        }
    }
}
