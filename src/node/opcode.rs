use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::node::opcode::instruction::{
    ANewArray, CheckCast, GetField, GetStatic, InstanceOf, InvokeDynamic, InvokeInterface,
    InvokeSpecial, InvokeStatic, InvokeVirtual, Ldc, Ldc2_W, Ldc_W, MultiANewArray, New, PutField,
    PutStatic, Wide,
};

pub mod instruction;

/// [ArrayType] represents all possible types for [Instruction::NEWARRAY] to use with.
///
/// See [Table 6.5.newarray-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=595).
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    TryFromPrimitive,
)]
pub enum ArrayType {
    Boolean = 4,
    Char = 5,
    Float = 6,
    Double = 7,
    Byte = 8,
    Short = 9,
    Int = 10,
    Long = 11,
}

// noinspection SpellCheckingInspection
/// Represents opcodes without any accompany data.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    TryFromPrimitive,
)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    NOP = 0,
    ACONST_NULL = 1,
    ICONST_M1 = 2,
    ICONST_0 = 3,
    ICONST_1 = 4,
    ICONST_2 = 5,
    ICONST_3 = 6,
    ICONST_4 = 7,
    ICONST_5 = 8,
    LCONST_0 = 9,
    LCONST_1 = 10,
    FCONST_0 = 11,
    FCONST_1 = 12,
    FCONST_2 = 13,
    DCONST_0 = 14,
    DCONST_1 = 15,
    BIPUSH = 16,
    SIPUSH = 17,
    LDC = 18,
    LDC_W = 19,
    LDC2_W = 20,
    ILOAD = 21,
    LLOAD = 22,
    FLOAD = 23,
    DLOAD = 24,
    ALOAD = 25,
    ILOAD_0 = 26,
    ILOAD_1 = 27,
    ILOAD_2 = 28,
    ILOAD_3 = 29,
    LLOAD_0 = 30,
    LLOAD_1 = 31,
    LLOAD_2 = 32,
    LLOAD_3 = 33,
    FLOAD_0 = 34,
    FLOAD_1 = 35,
    FLOAD_2 = 36,
    FLOAD_3 = 37,
    DLOAD_0 = 38,
    DLOAD_1 = 39,
    DLOAD_2 = 40,
    DLOAD_3 = 41,
    ALOAD_0 = 42,
    ALOAD_1 = 43,
    ALOAD_2 = 44,
    ALOAD_3 = 45,
    IALOAD = 46,
    LALOAD = 47,
    FALOAD = 48,
    DALOAD = 49,
    AALOAD = 50,
    BALOAD = 51,
    CALOAD = 52,
    SALOAD = 53,
    ISTORE = 54,
    LSTORE = 55,
    FSTORE = 56,
    DSTORE = 57,
    ASTORE = 58,
    ISTORE_0 = 59,
    ISTORE_1 = 60,
    ISTORE_2 = 61,
    ISTORE_3 = 62,
    LSTORE_0 = 63,
    LSTORE_1 = 64,
    LSTORE_2 = 65,
    LSTORE_3 = 66,
    FSTORE_0 = 67,
    FSTORE_1 = 68,
    FSTORE_2 = 69,
    FSTORE_3 = 70,
    DSTORE_0 = 71,
    DSTORE_1 = 72,
    DSTORE_2 = 73,
    DSTORE_3 = 74,
    ASTORE_0 = 75,
    ASTORE_1 = 76,
    ASTORE_2 = 77,
    ASTORE_3 = 78,
    IASTORE = 79,
    LASTORE = 80,
    FASTORE = 81,
    DASTORE = 82,
    AASTORE = 83,
    BASTORE = 84,
    CASTORE = 85,
    SASTORE = 86,
    POP = 87,
    POP2 = 88,
    DUP = 89,
    DUP_X1 = 90,
    DUP_X2 = 91,
    DUP2 = 92,
    DUP2_X1 = 93,
    DUP2_X2 = 94,
    SWAP = 95,
    IADD = 96,
    LADD = 97,
    FADD = 98,
    DADD = 99,
    ISUB = 100,
    LSUB = 101,
    FSUB = 102,
    DSUB = 103,
    IMUL = 104,
    LMUL = 105,
    FMUL = 106,
    DMUL = 107,
    IDIV = 108,
    LDIV = 109,
    FDIV = 110,
    DDIV = 111,
    IREM = 112,
    LREM = 113,
    FREM = 114,
    DREM = 115,
    INEG = 116,
    LNEG = 117,
    FNEG = 118,
    DNEG = 119,
    ISHL = 120,
    LSHL = 121,
    ISHR = 122,
    LSHR = 123,
    IUSHR = 124,
    LUSHR = 125,
    IAND = 126,
    LAND = 127,
    IOR = 128,
    LOR = 129,
    IXOR = 130,
    LXOR = 131,
    IINC = 132,
    I2L = 133,
    I2F = 134,
    I2D = 135,
    L2I = 136,
    L2F = 137,
    L2D = 138,
    F2I = 139,
    F2L = 140,
    F2D = 141,
    D2I = 142,
    D2L = 143,
    D2F = 144,
    I2B = 145,
    I2C = 146,
    I2S = 147,
    LCMP = 148,
    FCMPL = 149,
    FCMPG = 150,
    DCMPL = 151,
    DCMPG = 152,
    IFEQ = 153,
    IFNE = 154,
    IFLT = 155,
    IFGE = 156,
    IFGT = 157,
    IFLE = 158,
    IF_ICMPEQ = 159,
    IF_ICMPNE = 160,
    IF_ICMPLT = 161,
    IF_ICMPGE = 162,
    IF_ICMPGT = 163,
    IF_ICMPLE = 164,
    IF_ACMPEQ = 165,
    IF_ACMPNE = 166,
    GOTO = 167,
    JSR = 168,
    RET = 169,
    TABLESWITCH = 170,
    LOOKUPSWITCH = 171,
    IRETURN = 172,
    LRETURN = 173,
    FRETURN = 174,
    DRETURN = 175,
    ARETURN = 176,
    RETURN = 177,
    GETSTATIC = 178,
    PUTSTATIC = 179,
    GETFIELD = 180,
    PUTFIELD = 181,
    INVOKEVIRTUAL = 182,
    INVOKESPECIAL = 183,
    INVOKESTATIC = 184,
    INVOKEINTERFACE = 185,
    INVOKEDYNAMIC = 186,
    NEW = 187,
    NEWARRAY = 188,
    ANEWARRAY = 189,
    ARRAYLENGTH = 190,
    ATHROW = 191,
    CHECKCAST = 192,
    INSTANCEOF = 193,
    MONITORENTER = 194,
    MONITOREXIT = 195,
    WIDE = 196,
    MULTIANEWARRAY = 197,
    IFNULL = 198,
    IFNONNULL = 199,
    GOTO_W = 200,
    JSR_W = 201,
}

// noinspection SpellCheckingInspection
/// Represents opcode with accompany data.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP,
    ACONST_NULL,
    ICONST_M1,
    ICONST_0,
    ICONST_1,
    ICONST_2,
    ICONST_3,
    ICONST_4,
    ICONST_5,
    LCONST_0,
    LCONST_1,
    FCONST_0,
    FCONST_1,
    FCONST_2,
    DCONST_0,
    DCONST_1,
    BIPUSH(i8),
    SIPUSH(i16),
    LDC(Ldc),
    LDC_W(Ldc_W),
    LDC2_W(Ldc2_W),
    ILOAD(u8),
    LLOAD(u8),
    FLOAD(u8),
    DLOAD(u8),
    ALOAD(u8),
    ILOAD_0,
    ILOAD_1,
    ILOAD_2,
    ILOAD_3,
    LLOAD_0,
    LLOAD_1,
    LLOAD_2,
    LLOAD_3,
    FLOAD_0,
    FLOAD_1,
    FLOAD_2,
    FLOAD_3,
    DLOAD_0,
    DLOAD_1,
    DLOAD_2,
    DLOAD_3,
    ALOAD_0,
    ALOAD_1,
    ALOAD_2,
    ALOAD_3,
    IALOAD,
    LALOAD,
    FALOAD,
    DALOAD,
    AALOAD,
    BALOAD,
    CALOAD,
    SALOAD,
    ISTORE(u8),
    LSTORE(u8),
    FSTORE(u8),
    DSTORE(u8),
    ASTORE(u8),
    ISTORE_0,
    ISTORE_1,
    ISTORE_2,
    ISTORE_3,
    LSTORE_0,
    LSTORE_1,
    LSTORE_2,
    LSTORE_3,
    FSTORE_0,
    FSTORE_1,
    FSTORE_2,
    FSTORE_3,
    DSTORE_0,
    DSTORE_1,
    DSTORE_2,
    DSTORE_3,
    ASTORE_0,
    ASTORE_1,
    ASTORE_2,
    ASTORE_3,
    IASTORE,
    LASTORE,
    FASTORE,
    DASTORE,
    AASTORE,
    BASTORE,
    CASTORE,
    SASTORE,
    POP,
    POP2,
    DUP,
    DUP_X1,
    DUP_X2,
    DUP2,
    DUP2_X1,
    DUP2_X2,
    SWAP,
    IADD,
    LADD,
    FADD,
    DADD,
    ISUB,
    LSUB,
    FSUB,
    DSUB,
    IMUL,
    LMUL,
    FMUL,
    DMUL,
    IDIV,
    LDIV,
    FDIV,
    DDIV,
    IREM,
    LREM,
    FREM,
    DREM,
    INEG,
    LNEG,
    FNEG,
    DNEG,
    ISHL,
    LSHL,
    ISHR,
    LSHR,
    IUSHR,
    LUSHR,
    IAND,
    LAND,
    IOR,
    LOR,
    IXOR,
    LXOR,
    IINC {
        index: u8,
        value: i8,
    },
    I2L,
    I2F,
    I2D,
    L2I,
    L2F,
    L2D,
    F2I,
    F2L,
    F2D,
    D2I,
    D2L,
    D2F,
    I2B,
    I2C,
    I2S,
    LCMP,
    FCMPL,
    FCMPG,
    DCMPL,
    DCMPG,
    IFEQ(i16),
    IFNE(i16),
    IFLT(i16),
    IFGE(i16),
    IFGT(i16),
    IFLE(i16),
    IF_ICMPEQ(i16),
    IF_ICMPNE(i16),
    IF_ICMPLT(i16),
    IF_ICMPGE(i16),
    IF_ICMPGT(i16),
    IF_ICMPLE(i16),
    IF_ACMPEQ(i16),
    IF_ACMPNE(i16),
    GOTO(i16),
    JSR(i16),
    RET(u8),
    TABLESWITCH {
        default: i32,
        low: i32,
        high: i32,
        offsets: Vec<i32>,
    },
    LOOKUPSWITCH {
        default: i32,
        npairs: u32,
        pairs: Vec<(i32, i32)>,
    },
    IRETURN,
    LRETURN,
    FRETURN,
    DRETURN,
    ARETURN,
    RETURN,
    GETSTATIC(GetStatic),
    PUTSTATIC(PutStatic),
    GETFIELD(GetField),
    PUTFIELD(PutField),
    INVOKEVIRTUAL(InvokeVirtual),
    INVOKESPECIAL(InvokeSpecial),
    INVOKESTATIC(InvokeStatic),
    INVOKEINTERFACE(InvokeInterface),
    INVOKEDYNAMIC(InvokeDynamic),
    NEW(New),
    NEWARRAY(ArrayType),
    ANEWARRAY(ANewArray),
    ARRAYLENGTH,
    ATHROW,
    CHECKCAST(CheckCast),
    INSTANCEOF(InstanceOf),
    MONITORENTER,
    MONITOREXIT,
    WIDE(Wide),
    MULTIANEWARRAY(MultiANewArray),
    IFNULL(i16),
    IFNONNULL(i16),
    GOTO_W(i64),
    JSR_W(i64),
}

impl Instruction {
    /// Returns corresponding [Opcode] for current [Instruction].
    pub const fn opcode(&self) -> Opcode {
        match self {
            Instruction::NOP => Opcode::NOP,
            Instruction::ACONST_NULL => Opcode::ACONST_NULL,
            Instruction::ICONST_M1 => Opcode::ICONST_M1,
            Instruction::ICONST_0 => Opcode::ICONST_0,
            Instruction::ICONST_1 => Opcode::ICONST_1,
            Instruction::ICONST_2 => Opcode::ICONST_2,
            Instruction::ICONST_3 => Opcode::ICONST_3,
            Instruction::ICONST_4 => Opcode::ICONST_4,
            Instruction::ICONST_5 => Opcode::ICONST_5,
            Instruction::LCONST_0 => Opcode::LCONST_0,
            Instruction::LCONST_1 => Opcode::LCONST_1,
            Instruction::FCONST_0 => Opcode::FCONST_0,
            Instruction::FCONST_1 => Opcode::FCONST_1,
            Instruction::FCONST_2 => Opcode::FCONST_2,
            Instruction::DCONST_0 => Opcode::DCONST_0,
            Instruction::DCONST_1 => Opcode::DCONST_1,
            Instruction::BIPUSH(_) => Opcode::BIPUSH,
            Instruction::SIPUSH(_) => Opcode::SIPUSH,
            Instruction::LDC(_) => Opcode::LDC,
            Instruction::LDC_W(_) => Opcode::LDC_W,
            Instruction::LDC2_W(_) => Opcode::LDC2_W,
            Instruction::ILOAD(_) => Opcode::ILOAD,
            Instruction::LLOAD(_) => Opcode::LLOAD,
            Instruction::FLOAD(_) => Opcode::FLOAD,
            Instruction::DLOAD(_) => Opcode::DLOAD,
            Instruction::ALOAD(_) => Opcode::ALOAD,
            Instruction::ILOAD_0 => Opcode::ILOAD_0,
            Instruction::ILOAD_1 => Opcode::ILOAD_1,
            Instruction::ILOAD_2 => Opcode::ILOAD_2,
            Instruction::ILOAD_3 => Opcode::ILOAD_3,
            Instruction::LLOAD_0 => Opcode::LLOAD_0,
            Instruction::LLOAD_1 => Opcode::LLOAD_1,
            Instruction::LLOAD_2 => Opcode::LLOAD_2,
            Instruction::LLOAD_3 => Opcode::LLOAD_3,
            Instruction::FLOAD_0 => Opcode::FLOAD_0,
            Instruction::FLOAD_1 => Opcode::FLOAD_1,
            Instruction::FLOAD_2 => Opcode::FLOAD_2,
            Instruction::FLOAD_3 => Opcode::FLOAD_3,
            Instruction::DLOAD_0 => Opcode::DLOAD_0,
            Instruction::DLOAD_1 => Opcode::DLOAD_1,
            Instruction::DLOAD_2 => Opcode::DLOAD_2,
            Instruction::DLOAD_3 => Opcode::DLOAD_3,
            Instruction::ALOAD_0 => Opcode::ALOAD_0,
            Instruction::ALOAD_1 => Opcode::ALOAD_1,
            Instruction::ALOAD_2 => Opcode::ALOAD_2,
            Instruction::ALOAD_3 => Opcode::ALOAD_3,
            Instruction::IALOAD => Opcode::IALOAD,
            Instruction::LALOAD => Opcode::LALOAD,
            Instruction::FALOAD => Opcode::FALOAD,
            Instruction::DALOAD => Opcode::DALOAD,
            Instruction::AALOAD => Opcode::AALOAD,
            Instruction::BALOAD => Opcode::BALOAD,
            Instruction::CALOAD => Opcode::CALOAD,
            Instruction::SALOAD => Opcode::SALOAD,
            Instruction::ISTORE(_) => Opcode::ISTORE,
            Instruction::LSTORE(_) => Opcode::LSTORE,
            Instruction::FSTORE(_) => Opcode::FSTORE,
            Instruction::DSTORE(_) => Opcode::DSTORE,
            Instruction::ASTORE(_) => Opcode::ASTORE,
            Instruction::ISTORE_0 => Opcode::ISTORE_0,
            Instruction::ISTORE_1 => Opcode::ISTORE_1,
            Instruction::ISTORE_2 => Opcode::ISTORE_2,
            Instruction::ISTORE_3 => Opcode::ISTORE_3,
            Instruction::LSTORE_0 => Opcode::LSTORE_0,
            Instruction::LSTORE_1 => Opcode::LSTORE_1,
            Instruction::LSTORE_2 => Opcode::LSTORE_2,
            Instruction::LSTORE_3 => Opcode::LSTORE_3,
            Instruction::FSTORE_0 => Opcode::FSTORE_0,
            Instruction::FSTORE_1 => Opcode::FSTORE_1,
            Instruction::FSTORE_2 => Opcode::FSTORE_2,
            Instruction::FSTORE_3 => Opcode::FSTORE_3,
            Instruction::DSTORE_0 => Opcode::DSTORE_0,
            Instruction::DSTORE_1 => Opcode::DSTORE_1,
            Instruction::DSTORE_2 => Opcode::DSTORE_2,
            Instruction::DSTORE_3 => Opcode::DSTORE_3,
            Instruction::ASTORE_0 => Opcode::ASTORE_0,
            Instruction::ASTORE_1 => Opcode::ASTORE_1,
            Instruction::ASTORE_2 => Opcode::ASTORE_2,
            Instruction::ASTORE_3 => Opcode::ASTORE_3,
            Instruction::IASTORE => Opcode::IASTORE,
            Instruction::LASTORE => Opcode::LASTORE,
            Instruction::FASTORE => Opcode::FASTORE,
            Instruction::DASTORE => Opcode::DASTORE,
            Instruction::AASTORE => Opcode::AASTORE,
            Instruction::BASTORE => Opcode::BASTORE,
            Instruction::CASTORE => Opcode::CASTORE,
            Instruction::SASTORE => Opcode::SASTORE,
            Instruction::POP => Opcode::POP,
            Instruction::POP2 => Opcode::POP2,
            Instruction::DUP => Opcode::DUP,
            Instruction::DUP_X1 => Opcode::DUP_X1,
            Instruction::DUP_X2 => Opcode::DUP_X2,
            Instruction::DUP2 => Opcode::DUP2,
            Instruction::DUP2_X1 => Opcode::DUP2_X1,
            Instruction::DUP2_X2 => Opcode::DUP2_X2,
            Instruction::SWAP => Opcode::SWAP,
            Instruction::IADD => Opcode::IADD,
            Instruction::LADD => Opcode::LADD,
            Instruction::FADD => Opcode::FADD,
            Instruction::DADD => Opcode::DADD,
            Instruction::ISUB => Opcode::ISUB,
            Instruction::LSUB => Opcode::LSUB,
            Instruction::FSUB => Opcode::FSUB,
            Instruction::DSUB => Opcode::DSUB,
            Instruction::IMUL => Opcode::IMUL,
            Instruction::LMUL => Opcode::LMUL,
            Instruction::FMUL => Opcode::FMUL,
            Instruction::DMUL => Opcode::DMUL,
            Instruction::IDIV => Opcode::IDIV,
            Instruction::LDIV => Opcode::LDIV,
            Instruction::FDIV => Opcode::FDIV,
            Instruction::DDIV => Opcode::DDIV,
            Instruction::IREM => Opcode::IREM,
            Instruction::LREM => Opcode::LREM,
            Instruction::FREM => Opcode::FREM,
            Instruction::DREM => Opcode::DREM,
            Instruction::INEG => Opcode::INEG,
            Instruction::LNEG => Opcode::LNEG,
            Instruction::FNEG => Opcode::FNEG,
            Instruction::DNEG => Opcode::DNEG,
            Instruction::ISHL => Opcode::ISHL,
            Instruction::LSHL => Opcode::LSHL,
            Instruction::ISHR => Opcode::ISHR,
            Instruction::LSHR => Opcode::LSHR,
            Instruction::IUSHR => Opcode::IUSHR,
            Instruction::LUSHR => Opcode::LUSHR,
            Instruction::IAND => Opcode::IAND,
            Instruction::LAND => Opcode::LAND,
            Instruction::IOR => Opcode::IOR,
            Instruction::LOR => Opcode::LOR,
            Instruction::IXOR => Opcode::IXOR,
            Instruction::LXOR => Opcode::LXOR,
            Instruction::IINC { .. } => Opcode::IINC,
            Instruction::I2L => Opcode::I2L,
            Instruction::I2F => Opcode::I2F,
            Instruction::I2D => Opcode::I2D,
            Instruction::L2I => Opcode::L2I,
            Instruction::L2F => Opcode::L2F,
            Instruction::L2D => Opcode::L2D,
            Instruction::F2I => Opcode::F2I,
            Instruction::F2L => Opcode::F2L,
            Instruction::F2D => Opcode::F2D,
            Instruction::D2I => Opcode::D2I,
            Instruction::D2L => Opcode::D2L,
            Instruction::D2F => Opcode::D2F,
            Instruction::I2B => Opcode::I2B,
            Instruction::I2C => Opcode::I2C,
            Instruction::I2S => Opcode::I2S,
            Instruction::LCMP => Opcode::LCMP,
            Instruction::FCMPL => Opcode::FCMPL,
            Instruction::FCMPG => Opcode::FCMPG,
            Instruction::DCMPL => Opcode::DCMPL,
            Instruction::DCMPG => Opcode::DCMPG,
            Instruction::IFEQ(_) => Opcode::IFEQ,
            Instruction::IFNE(_) => Opcode::IFNE,
            Instruction::IFLT(_) => Opcode::IFLT,
            Instruction::IFGE(_) => Opcode::IFGE,
            Instruction::IFGT(_) => Opcode::IFGT,
            Instruction::IFLE(_) => Opcode::IFLE,
            Instruction::IF_ICMPEQ(_) => Opcode::IF_ICMPEQ,
            Instruction::IF_ICMPNE(_) => Opcode::IF_ICMPNE,
            Instruction::IF_ICMPLT(_) => Opcode::IF_ICMPLT,
            Instruction::IF_ICMPGE(_) => Opcode::IF_ICMPGE,
            Instruction::IF_ICMPGT(_) => Opcode::IF_ICMPGT,
            Instruction::IF_ICMPLE(_) => Opcode::IF_ICMPLE,
            Instruction::IF_ACMPEQ(_) => Opcode::IF_ACMPEQ,
            Instruction::IF_ACMPNE(_) => Opcode::IF_ACMPNE,
            Instruction::GOTO(_) => Opcode::GOTO,
            Instruction::JSR(_) => Opcode::JSR,
            Instruction::RET(_) => Opcode::RET,
            Instruction::TABLESWITCH { .. } => Opcode::TABLESWITCH,
            Instruction::LOOKUPSWITCH { .. } => Opcode::LOOKUPSWITCH,
            Instruction::IRETURN => Opcode::IRETURN,
            Instruction::LRETURN => Opcode::LRETURN,
            Instruction::FRETURN => Opcode::FRETURN,
            Instruction::DRETURN => Opcode::DRETURN,
            Instruction::ARETURN => Opcode::ARETURN,
            Instruction::RETURN => Opcode::RETURN,
            Instruction::GETSTATIC(..) => Opcode::GETSTATIC,
            Instruction::PUTSTATIC(..) => Opcode::PUTSTATIC,
            Instruction::GETFIELD(..) => Opcode::GETFIELD,
            Instruction::PUTFIELD(..) => Opcode::PUTFIELD,
            Instruction::INVOKEVIRTUAL(..) => Opcode::INVOKEVIRTUAL,
            Instruction::INVOKESPECIAL(..) => Opcode::INVOKESPECIAL,
            Instruction::INVOKESTATIC(..) => Opcode::INVOKESTATIC,
            Instruction::INVOKEINTERFACE { .. } => Opcode::INVOKEINTERFACE,
            Instruction::INVOKEDYNAMIC(..) => Opcode::INVOKEDYNAMIC,
            Instruction::NEW(..) => Opcode::NEW,
            Instruction::NEWARRAY(..) => Opcode::NEWARRAY,
            Instruction::ANEWARRAY(..) => Opcode::ANEWARRAY,
            Instruction::ARRAYLENGTH => Opcode::ARRAYLENGTH,
            Instruction::ATHROW => Opcode::ATHROW,
            Instruction::CHECKCAST(..) => Opcode::CHECKCAST,
            Instruction::INSTANCEOF(..) => Opcode::INSTANCEOF,
            Instruction::MONITORENTER => Opcode::MONITORENTER,
            Instruction::MONITOREXIT => Opcode::MONITOREXIT,
            Instruction::WIDE(..) => Opcode::WIDE,
            Instruction::MULTIANEWARRAY { .. } => Opcode::MULTIANEWARRAY,
            Instruction::IFNULL(..) => Opcode::IFNULL,
            Instruction::IFNONNULL(..) => Opcode::IFNONNULL,
            Instruction::GOTO_W(..) => Opcode::GOTO_W,
            Instruction::JSR_W(..) => Opcode::JSR_W,
        }
    }
}
