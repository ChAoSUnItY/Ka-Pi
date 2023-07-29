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

    let instruction = match opcode {
        0x00 => Instruction::NOP,
        0x01 => Instruction::ACONST_NULL,
        0x02 => Instruction::ICONST_M1,
        0x03 => Instruction::ICONST_0,
        0x04 => Instruction::ICONST_1,
        0x05 => Instruction::ICONST_2,
        0x06 => Instruction::ICONST_3,
        0x07 => Instruction::ICONST_4,
        0x08 => Instruction::ICONST_5,
        0x09 => Instruction::LCONST_0,
        0x0A => Instruction::LCONST_1,
        0x0B => Instruction::FCONST_0,
        0x0C => Instruction::FCONST_1,
        0x0D => Instruction::FCONST_2,
        0x0E => Instruction::DCONST_0,
        0x0F => Instruction::DCONST_1,
        0x10 => {
            let byte = input.read_i8()?;

            *pc += 1;

            Instruction::BIPUSH(byte)
        }
        0x11 => {
            let byte = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::SIPUSH(byte)
        }
        0x12 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::LDC(Ldc { index })
        }
        0x13 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::LDC_W(Ldc_W { index })
        }
        0x14 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::LDC2_W(Ldc2_W { index })
        }
        0x15 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::ILOAD(index)
        }
        0x16 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::LLOAD(index)
        }
        0x17 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::FLOAD(index)
        }
        0x18 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::DLOAD(index)
        }
        0x19 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::ALOAD(index)
        }
        0x1A => Instruction::ILOAD_0,
        0x1B => Instruction::ILOAD_1,
        0x1C => Instruction::ILOAD_2,
        0x1D => Instruction::ILOAD_3,
        0x1E => Instruction::LLOAD_0,
        0x1F => Instruction::LLOAD_1,
        0x20 => Instruction::LLOAD_2,
        0x21 => Instruction::LLOAD_3,
        0x22 => Instruction::FLOAD_0,
        0x23 => Instruction::FLOAD_1,
        0x24 => Instruction::FLOAD_2,
        0x25 => Instruction::FLOAD_3,
        0x26 => Instruction::DLOAD_0,
        0x27 => Instruction::DLOAD_1,
        0x28 => Instruction::DLOAD_2,
        0x29 => Instruction::DLOAD_3,
        0x2A => Instruction::ALOAD_0,
        0x2B => Instruction::ALOAD_1,
        0x2C => Instruction::ALOAD_2,
        0x2D => Instruction::ALOAD_3,
        0x2E => Instruction::IALOAD,
        0x2F => Instruction::LALOAD,
        0x30 => Instruction::FALOAD,
        0x31 => Instruction::DALOAD,
        0x32 => Instruction::AALOAD,
        0x33 => Instruction::BALOAD,
        0x34 => Instruction::CALOAD,
        0x35 => Instruction::SALOAD,
        0x36 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::ISTORE(index)
        }
        0x37 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::LSTORE(index)
        }
        0x38 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::FSTORE(index)
        }
        0x39 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::DSTORE(index)
        }
        0x3A => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::ASTORE(index)
        }
        0x3B => Instruction::ISTORE_0,
        0x3C => Instruction::ISTORE_1,
        0x3D => Instruction::ISTORE_2,
        0x3E => Instruction::ISTORE_3,
        0x3F => Instruction::LSTORE_0,
        0x40 => Instruction::LSTORE_1,
        0x41 => Instruction::LSTORE_2,
        0x42 => Instruction::LSTORE_3,
        0x43 => Instruction::FSTORE_0,
        0x44 => Instruction::FSTORE_1,
        0x45 => Instruction::FSTORE_2,
        0x46 => Instruction::FSTORE_3,
        0x47 => Instruction::DSTORE_0,
        0x48 => Instruction::DSTORE_1,
        0x49 => Instruction::DSTORE_2,
        0x4A => Instruction::DSTORE_3,
        0x4B => Instruction::ASTORE_0,
        0x4C => Instruction::ASTORE_1,
        0x4D => Instruction::ASTORE_2,
        0x4E => Instruction::ASTORE_3,
        0x4F => Instruction::IASTORE,
        0x50 => Instruction::LASTORE,
        0x51 => Instruction::FASTORE,
        0x52 => Instruction::DASTORE,
        0x53 => Instruction::AASTORE,
        0x54 => Instruction::BASTORE,
        0x55 => Instruction::CASTORE,
        0x56 => Instruction::SASTORE,
        0x57 => Instruction::POP,
        0x58 => Instruction::POP2,
        0x59 => Instruction::DUP,
        0x5A => Instruction::DUP_X1,
        0x5B => Instruction::DUP_X2,
        0x5C => Instruction::DUP2,
        0x5D => Instruction::DUP_X1,
        0x5E => Instruction::DUP2_X2,
        0x5F => Instruction::SWAP,
        0x60 => Instruction::IADD,
        0x61 => Instruction::LADD,
        0x62 => Instruction::FADD,
        0x63 => Instruction::DADD,
        0x64 => Instruction::ISUB,
        0x65 => Instruction::LSUB,
        0x66 => Instruction::FSUB,
        0x67 => Instruction::DSUB,
        0x68 => Instruction::IMUL,
        0x69 => Instruction::LMUL,
        0x6A => Instruction::FMUL,
        0x6B => Instruction::DMUL,
        0x6C => Instruction::IDIV,
        0x6D => Instruction::LDIV,
        0x6E => Instruction::FDIV,
        0x6F => Instruction::DDIV,
        0x70 => Instruction::IREM,
        0x71 => Instruction::LREM,
        0x72 => Instruction::FREM,
        0x73 => Instruction::DREM,
        0x74 => Instruction::INEG,
        0x75 => Instruction::LNEG,
        0x76 => Instruction::FNEG,
        0x77 => Instruction::DNEG,
        0x78 => Instruction::ISHL,
        0x79 => Instruction::LSHL,
        0x7A => Instruction::ISHR,
        0x7B => Instruction::LSHR,
        0x7C => Instruction::IUSHR,
        0x7D => Instruction::LUSHR,
        0x7E => Instruction::IAND,
        0x7F => Instruction::LAND,
        0x80 => Instruction::IOR,
        0x81 => Instruction::LOR,
        0x82 => Instruction::IXOR,
        0x83 => Instruction::LXOR,
        0x84 => {
            let index = input.read_u8()?;
            let value = input.read_i8()?;

            *pc += 2;

            Instruction::IINC { index, value }
        }
        0x85 => Instruction::I2L,
        0x86 => Instruction::I2F,
        0x87 => Instruction::I2D,
        0x88 => Instruction::L2I,
        0x89 => Instruction::L2F,
        0x8A => Instruction::L2D,
        0x8B => Instruction::F2I,
        0x8C => Instruction::F2L,
        0x8D => Instruction::F2D,
        0x8E => Instruction::D2I,
        0x8F => Instruction::D2L,
        0x90 => Instruction::D2L,
        0x91 => Instruction::I2B,
        0x92 => Instruction::I2C,
        0x93 => Instruction::I2S,
        0x94 => Instruction::LCMP,
        0x95 => Instruction::FCMPL,
        0x96 => Instruction::DCMPG,
        0x97 => Instruction::DCMPL,
        0x98 => Instruction::DCMPG,
        0x99 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFEQ(offset)
        }
        0x9A => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFNE(offset)
        }
        0x9B => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFLT(offset)
        }
        0x9C => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFGE(offset)
        }
        0x9D => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFGT(offset)
        }
        0x9E => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFLE(offset)
        }
        0x9F => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPEQ(offset)
        }
        0xA0 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPNE(offset)
        }
        0xA1 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPLT(offset)
        }
        0xA2 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPGE(offset)
        }
        0xA3 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPGT(offset)
        }
        0xA4 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ICMPLE(offset)
        }
        0xA5 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ACMPEQ(offset)
        }
        0xA6 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IF_ACMPNE(offset)
        }
        0xA7 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::GOTO(offset)
        }
        0xA8 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::JSR(offset)
        }
        0xA9 => {
            let index = input.read_u8()?;

            *pc += 1;

            Instruction::RET(index)
        }
        0xAA => {
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
        0xAB => {
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
        0xAC => Instruction::IRETURN,
        0xAD => Instruction::LRETURN,
        0xAE => Instruction::FRETURN,
        0xAF => Instruction::DRETURN,
        0xB0 => Instruction::ARETURN,
        0xB1 => Instruction::RETURN,
        0xB2 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::GETSTATIC(GetStatic { index })
        }
        0xB3 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::PUTSTATIC(PutStatic { index })
        }
        0xB4 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::GETFIELD(GetField { index })
        }
        0xB5 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::PUTFIELD(PutField { index })
        }
        0xB6 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::INVOKEVIRTUAL(InvokeVirtual { index })
        }
        0xB7 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::INVOKESPECIAL(InvokeSpecial { index })
        }
        0xB8 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::INVOKESTATIC(InvokeStatic { index })
        }
        0xB9 => {
            let index = input.read_u16::<BigEndian>()?;
            let count = input.read_u8()?;

            *pc += 3;

            Instruction::INVOKEINTERFACE(InvokeInterface { index, count })
        }
        0xBA => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::INVOKEDYNAMIC(InvokeDynamic { index })
        }
        0xBB => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::NEW(New { index })
        }
        0xBC => {
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
        0xBD => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::ANEWARRAY(ANewArray { index })
        }
        0xBE => Instruction::ARRAYLENGTH,
        0xBF => Instruction::ATHROW,
        0xC0 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::CHECKCAST(CheckCast { index })
        }
        0xC1 => {
            let index = input.read_u16::<BigEndian>()?;

            *pc += 2;

            Instruction::INSTANCEOF(InstanceOf { index })
        }
        0xC2 => Instruction::MONITORENTER,
        0xC3 => Instruction::MONITOREXIT,
        0xC4 => {
            let wide = wide(input, pc)?;

            Instruction::WIDE(wide)
        }
        0xC5 => {
            let index = input.read_u16::<BigEndian>()?;
            let dimensions = input.read_u8()?;

            *pc += 3;

            Instruction::MULTIANEWARRAY(MultiANewArray { index, dimensions })
        }
        0xC6 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFNULL(offset)
        }
        0xC7 => {
            let offset = input.read_i16::<BigEndian>()?;

            *pc += 2;

            Instruction::IFNONNULL(offset)
        }
        0xC8 => {
            let offset = input.read_i64::<BigEndian>()?;

            *pc += 4;

            Instruction::GOTO_W(offset)
        }
        0xC9 => {
            let offset = input.read_i64::<BigEndian>()?;

            *pc += 4;

            Instruction::JSR_W(offset)
        }
        _ => {
            // None of listed opcode described by Java SE 20 Specification
            return Err(ParseError::MatchOutOfBoundOpcode(opcode));
        }
    };

    Ok(instruction)
}

#[inline]
fn lookup_table_pair<R: Read>(input: &mut R) -> ParseResult<(i32, i32)> {
    let left = input.read_i32::<BigEndian>()?;
    let right = input.read_i32::<BigEndian>()?;

    Ok((left, right))
}

fn wide<R: Read>(input: &mut R, pc: &mut u32) -> ParseResult<Wide> {
    let widened_opcode = input.read_u8()?;

    let wide = match widened_opcode {
        0x15 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::ILOAD(index)
        }
        0x17 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::FLOAD(index)
        }
        0x19 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::ALOAD(index)
        }
        0x16 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::LLOAD(index)
        }
        0x18 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::DLOAD(index)
        }
        0x36 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::ISTORE(index)
        }
        0x38 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::FSTORE(index)
        }
        0x3A => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::ASTORE(index)
        }
        0x37 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::LSTORE(index)
        }
        0x39 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::DSTORE(index)
        }
        0xA9 => {
            let index = input.read_u16::<BigEndian>()?;

            Wide::RET(index)
        }
        0x84 => {
            let index = input.read_u16::<BigEndian>()?;
            let value = input.read_i16::<BigEndian>()?;

            Wide::IINC(index, value)
        }
        _ => {
            return Err(ParseError::MatchOutOfBoundWideOpcode(
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
        }
    };

    *pc += if matches!(wide, Wide::IINC(_, _)) {
        9
    } else {
        7
    };

    Ok(wide)
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
