// ASM API versions.
// These APIs are unused at this moments.

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::BitOr;

use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

use crate::asm::handle::Handle;

pub const ASM4: u32 = 4 << 16 | 0 << 8;
pub const ASM5: u32 = 5 << 16 | 0 << 8;
pub const ASM6: u32 = 6 << 16 | 0 << 8;
pub const ASM7: u32 = 7 << 16 | 0 << 8;
pub const ASM8: u32 = 8 << 16 | 0 << 8;
pub const ASM9: u32 = 9 << 16 | 0 << 8;

pub const SOURCE_DEPRECATED: u32 = 0x100;
pub const SOURCE_MASK: u32 = SOURCE_DEPRECATED;

// Java ClassFile versions (the minor version is stored in the 16 most significant bits, and the
// major version in the 16 least significant bits).

pub const V1_1: u32 = 3 << 16 | 45;
pub const V1_2: u32 = 0 << 16 | 46;
pub const V1_3: u32 = 0 << 16 | 47;
pub const V1_4: u32 = 0 << 16 | 48;
pub const V1_5: u32 = 0 << 16 | 49;
pub const V1_6: u32 = 0 << 16 | 50;
pub const V1_7: u32 = 0 << 16 | 51;
pub const V1_8: u32 = 0 << 16 | 52;
pub const V9: u32 = 0 << 16 | 53;
pub const V10: u32 = 0 << 16 | 54;
pub const V11: u32 = 0 << 16 | 55;
pub const V12: u32 = 0 << 16 | 56;
pub const V13: u32 = 0 << 16 | 57;
pub const V14: u32 = 0 << 16 | 58;
pub const V15: u32 = 0 << 16 | 59;
pub const V16: u32 = 0 << 16 | 60;
pub const V17: u32 = 0 << 16 | 61;
pub const V18: u32 = 0 << 16 | 62;
pub const V19: u32 = 0 << 16 | 63;
pub const V20: u32 = 0 << 16 | 64;
pub const V21: u32 = 0 << 16 | 65;

/**
Version flag indicating that the class is using 'preview' features.
 */
pub const V_PREVIEW: u32 = 0xFFFF0000;

// Access flags values, defined in
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.1-200-E.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.5-200-A.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.6-200-A.1
// - https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.7.25

pub(crate) trait AccessFlag<'a, T>
where
    T: Into<u16> + Copy + 'a,
    Self: IntoIterator<Item = &'a T> + Sized,
{
    fn fold_flags(self) -> u16 {
        self.into_iter()
            .map(|flag| (*flag).into())
            .fold(0, u16::bitor)
    }
}

#[repr(u16)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, Serialize, Deserialize,
)]
pub enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

impl<'a> AccessFlag<'a, ClassAccessFlag> for &'a [ClassAccessFlag] {}
impl<'a> AccessFlag<'a, ClassAccessFlag> for &'a Vec<ClassAccessFlag> {}

#[repr(u16)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, Serialize, Deserialize,
)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

impl<'a> AccessFlag<'a, FieldAccessFlag> for &'a [FieldAccessFlag] {}
impl<'a> AccessFlag<'a, FieldAccessFlag> for &'a Vec<FieldAccessFlag> {}

#[repr(u16)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, Serialize, Deserialize,
)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

impl<'a> AccessFlag<'a, MethodAccessFlag> for &'a [MethodAccessFlag] {}
impl<'a> AccessFlag<'a, MethodAccessFlag> for &'a Vec<MethodAccessFlag> {}

// class, field, method
pub const ACC_PUBLIC: u32 = 0x0001;
// class, field, method
pub const ACC_PRIVATE: u32 = 0x0002;
// class, field, method
pub const ACC_PROTECTED: u32 = 0x0004;
// field, method
pub const ACC_STATIC: u32 = 0x0008;
// class, field, method, parameter
pub const ACC_FINAL: u32 = 0x0010;
// class
pub const ACC_SUPER: u32 = 0x0020;
// method
pub const ACC_SYNCHRONIZED: u32 = 0x0020;
// module
pub const ACC_OPEN: u32 = 0x0020;
// module requires
pub const ACC_TRANSITIVE: u32 = 0x0020;
// field
pub const ACC_VOLATILE: u32 = 0x0040;
// method
pub const ACC_BRIDGE: u32 = 0x0040;
// module requires
pub const ACC_STATIC_PHASE: u32 = 0x0040;
// method
pub const ACC_VARARGS: u32 = 0x0080;
// field
pub const ACC_TRANSIENT: u32 = 0x0080;
// method
pub const ACC_NATIVE: u32 = 0x0100;
// class
pub const ACC_INTERFACE: u32 = 0x0200;
// class, method
pub const ACC_ABSTRACT: u32 = 0x0400;
// method
pub const ACC_STRICT: u32 = 0x0800;
// class, field, method, parameter, module *
pub const ACC_SYNTHETIC: u32 = 0x1000;
// class
pub const ACC_ANNOTATION: u32 = 0x2000;
// class(?) field inner
pub const ACC_ENUM: u32 = 0x4000;
// field, method, parameter, module, module *
pub const ACC_MANDATED: u32 = 0x8000;
// class
pub const ACC_MODULE: u32 = 0x8000;

// Possible values for the type operand of the NEWARRAY instruction.
// See https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-6.html#jvms-6.5.newarray.

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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

pub const T_BOOLEAN: u8 = 4;
pub const T_CHAR: u8 = 5;
pub const T_FLOAT: u8 = 6;
pub const T_DOUBLE: u8 = 7;
pub const T_BYTE: u8 = 8;
pub const T_SHORT: u8 = 9;
pub const T_INT: u8 = 10;
pub const T_LONG: u8 = 11;

// Possible values for the reference_kind field of CONSTANT_MethodHandle_info structures.
// See https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-4.html#jvms-4.4.8.

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum RefKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

pub const H_GETFIELD: u8 = 1;
pub const H_GETSTATIC: u8 = 2;
pub const H_PUTFIELD: u8 = 3;
pub const H_PUTSTATIC: u8 = 4;
pub const H_INVOKEVIRTUAL: u8 = 5;
pub const H_INVOKESTATIC: u8 = 6;
pub const H_INVOKESPECIAL: u8 = 7;
pub const H_NEWINVOKESPECIAL: u8 = 8;
pub const H_INVOKEINTERFACE: u8 = 9;

pub const F_NEW: i8 = -1;

/** A compressed frame with complete frame data. */
pub const F_FULL: i8 = 0;

/**
A compressed frame where locals are the same as the locals in the previous frame, except that
additional 1-3 locals are defined, and with an empty stack.
 */
pub const F_APPEND: i8 = 1;

/**
A compressed frame where locals are the same as the locals in the previous frame, except that
the last 1-3 locals are absent and with an empty stack.
 */
pub const F_CHOP: i8 = 2;

/**
A compressed frame with exactly the same locals as the previous frame and with an empty stack.
 */
pub const F_SAME: i8 = 3;

/**
A compressed frame with exactly the same locals as the previous frame and with a single value
on the stack.
 */
pub const F_SAME1: i8 = 4;

// Standard stack map frame element types, used in {@link ClassVisitor#visitFrame}.

// pub const TOP: u8 = frame::ITEM_TOP;
// pub const INTEGER: u8 = frame::ITEM_INTEGER;
// pub const FLOAT: u8 = frame::ITEM_FLOAT;
// pub const DOUBLE: u8 = frame::ITEM_DOUBLE;
// pub const LONG: u8 = frame::ITEM_LONG;
// pub const NULL: u8 = frame::ITEM_NULL;
// pub const UNINITIALIZED_THIS: u8 = frame::ITEM_UNINITIALIZED_THIS;

// The JVM opcode values (with the MethodVisitor method name used to visit them in comment, and
// where '-' means 'same method name as on the previous line').
// See https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-6.html.

// noinspection SpellCheckingInspection
/// [NativeOpcode] represents low level JVM bytecode opcodes without any accompany data.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum NativeOpcode {
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
}

// noinspection SpellCheckingInspection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Opcode {
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
    LDC(ConstantObject),
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
    IINC(u8, u8),
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
    IFEQ(u16),
    IFNE(u16),
    IFLT(u16),
    IFGE(u16),
    IFGT(u16),
    IFLE(u16),
    IF_ICMPEQ(u16),
    IF_ICMPNE(u16),
    IF_ICMPLT(u16),
    IF_ICMPGE(u16),
    IF_ICMPGT(u16),
    IF_ICMPLE(u16),
    IF_ACMPEQ(u16),
    IF_ACMPNE(u16),
    GOTO(u16),
    JSR(u16),
    RET(u8),
    TABLESWITCH,  // TODO
    LOOKUPSWITCH, // TODO
    IRETURN,
    LRETURN,
    FRETURN,
    DRETURN,
    ARETURN,
    RETURN,
    GETSTATIC,
    PUTSTATIC,
    GETFIELD,
    PUTFIELD,
    INVOKEVIRTUAL,
    INVOKESPECIAL,
    INVOKESTATIC,
    INVOKEINTERFACE,
    INVOKEDYNAMIC,
    NEW,
    NEWARRAY,
    ANEWARRAY,
    ARRAYLENGTH,
    ATHROW,
    CHECKCAST,
    INSTANCEOF,
    MONITORENTER,
    MONITOREXIT,
    MULTIANEWARRAY,
    IFNULL,
    IFNONNULL,
}

impl Opcode {
    pub const fn native_opcode(&self) -> NativeOpcode {
        match self {
            Opcode::NOP => NativeOpcode::NOP,
            Opcode::ACONST_NULL => NativeOpcode::ACONST_NULL,
            Opcode::ICONST_M1 => NativeOpcode::ICONST_M1,
            Opcode::ICONST_0 => NativeOpcode::ICONST_0,
            Opcode::ICONST_1 => NativeOpcode::ICONST_1,
            Opcode::ICONST_2 => NativeOpcode::ICONST_2,
            Opcode::ICONST_3 => NativeOpcode::ICONST_3,
            Opcode::ICONST_4 => NativeOpcode::ICONST_4,
            Opcode::ICONST_5 => NativeOpcode::ICONST_5,
            Opcode::LCONST_0 => NativeOpcode::LCONST_0,
            Opcode::LCONST_1 => NativeOpcode::LCONST_1,
            Opcode::FCONST_0 => NativeOpcode::FCONST_0,
            Opcode::FCONST_1 => NativeOpcode::FCONST_1,
            Opcode::FCONST_2 => NativeOpcode::FCONST_2,
            Opcode::DCONST_0 => NativeOpcode::DCONST_0,
            Opcode::DCONST_1 => NativeOpcode::DCONST_1,
            Opcode::BIPUSH(_) => NativeOpcode::BIPUSH,
            Opcode::SIPUSH(_) => NativeOpcode::SIPUSH,
            Opcode::LDC(_) => NativeOpcode::LDC,
            Opcode::ILOAD(_) => NativeOpcode::ILOAD,
            Opcode::LLOAD(_) => NativeOpcode::LLOAD,
            Opcode::FLOAD(_) => NativeOpcode::FLOAD,
            Opcode::DLOAD(_) => NativeOpcode::DLOAD,
            Opcode::ALOAD(_) => NativeOpcode::ALOAD,
            Opcode::ILOAD_0 => NativeOpcode::ILOAD_0,
            Opcode::ILOAD_1 => NativeOpcode::ILOAD_1,
            Opcode::ILOAD_2 => NativeOpcode::ILOAD_2,
            Opcode::ILOAD_3 => NativeOpcode::ILOAD_3,
            Opcode::LLOAD_0 => NativeOpcode::LLOAD_0,
            Opcode::LLOAD_1 => NativeOpcode::LLOAD_1,
            Opcode::LLOAD_2 => NativeOpcode::LLOAD_2,
            Opcode::LLOAD_3 => NativeOpcode::LLOAD_3,
            Opcode::FLOAD_0 => NativeOpcode::FLOAD_0,
            Opcode::FLOAD_1 => NativeOpcode::FLOAD_1,
            Opcode::FLOAD_2 => NativeOpcode::FLOAD_2,
            Opcode::FLOAD_3 => NativeOpcode::FLOAD_3,
            Opcode::DLOAD_0 => NativeOpcode::DLOAD_0,
            Opcode::DLOAD_1 => NativeOpcode::DLOAD_1,
            Opcode::DLOAD_2 => NativeOpcode::DLOAD_2,
            Opcode::DLOAD_3 => NativeOpcode::DLOAD_3,
            Opcode::ALOAD_0 => NativeOpcode::ALOAD_0,
            Opcode::ALOAD_1 => NativeOpcode::ALOAD_1,
            Opcode::ALOAD_2 => NativeOpcode::ALOAD_2,
            Opcode::ALOAD_3 => NativeOpcode::ALOAD_3,
            Opcode::IALOAD => NativeOpcode::IALOAD,
            Opcode::LALOAD => NativeOpcode::LALOAD,
            Opcode::FALOAD => NativeOpcode::FALOAD,
            Opcode::DALOAD => NativeOpcode::DALOAD,
            Opcode::AALOAD => NativeOpcode::AALOAD,
            Opcode::BALOAD => NativeOpcode::BALOAD,
            Opcode::CALOAD => NativeOpcode::CALOAD,
            Opcode::SALOAD => NativeOpcode::SALOAD,
            Opcode::ISTORE(_) => NativeOpcode::ISTORE,
            Opcode::LSTORE(_) => NativeOpcode::LSTORE,
            Opcode::FSTORE(_) => NativeOpcode::FSTORE,
            Opcode::DSTORE(_) => NativeOpcode::DSTORE,
            Opcode::ASTORE(_) => NativeOpcode::ASTORE,
            Opcode::ISTORE_0 => NativeOpcode::ISTORE_0,
            Opcode::ISTORE_1 => NativeOpcode::ISTORE_1,
            Opcode::ISTORE_2 => NativeOpcode::ISTORE_2,
            Opcode::ISTORE_3 => NativeOpcode::ISTORE_3,
            Opcode::LSTORE_0 => NativeOpcode::LSTORE_0,
            Opcode::LSTORE_1 => NativeOpcode::LSTORE_1,
            Opcode::LSTORE_2 => NativeOpcode::LSTORE_2,
            Opcode::LSTORE_3 => NativeOpcode::LSTORE_3,
            Opcode::FSTORE_0 => NativeOpcode::FSTORE_0,
            Opcode::FSTORE_1 => NativeOpcode::FSTORE_1,
            Opcode::FSTORE_2 => NativeOpcode::FSTORE_2,
            Opcode::FSTORE_3 => NativeOpcode::FSTORE_3,
            Opcode::DSTORE_0 => NativeOpcode::DSTORE_0,
            Opcode::DSTORE_1 => NativeOpcode::DSTORE_1,
            Opcode::DSTORE_2 => NativeOpcode::DSTORE_2,
            Opcode::DSTORE_3 => NativeOpcode::DSTORE_3,
            Opcode::ASTORE_0 => NativeOpcode::ASTORE_0,
            Opcode::ASTORE_1 => NativeOpcode::ASTORE_1,
            Opcode::ASTORE_2 => NativeOpcode::ASTORE_2,
            Opcode::ASTORE_3 => NativeOpcode::ASTORE_3,
            Opcode::IASTORE => NativeOpcode::IASTORE,
            Opcode::LASTORE => NativeOpcode::LASTORE,
            Opcode::FASTORE => NativeOpcode::FASTORE,
            Opcode::DASTORE => NativeOpcode::DASTORE,
            Opcode::AASTORE => NativeOpcode::AASTORE,
            Opcode::BASTORE => NativeOpcode::BASTORE,
            Opcode::CASTORE => NativeOpcode::CASTORE,
            Opcode::SASTORE => NativeOpcode::SASTORE,
            Opcode::POP => NativeOpcode::POP,
            Opcode::POP2 => NativeOpcode::POP2,
            Opcode::DUP => NativeOpcode::DUP,
            Opcode::DUP_X1 => NativeOpcode::DUP_X1,
            Opcode::DUP_X2 => NativeOpcode::DUP_X2,
            Opcode::DUP2 => NativeOpcode::DUP2,
            Opcode::DUP2_X1 => NativeOpcode::DUP2_X1,
            Opcode::DUP2_X2 => NativeOpcode::DUP2_X2,
            Opcode::SWAP => NativeOpcode::SWAP,
            Opcode::IADD => NativeOpcode::IADD,
            Opcode::LADD => NativeOpcode::LADD,
            Opcode::FADD => NativeOpcode::FADD,
            Opcode::DADD => NativeOpcode::DADD,
            Opcode::ISUB => NativeOpcode::ISUB,
            Opcode::LSUB => NativeOpcode::LSUB,
            Opcode::FSUB => NativeOpcode::FSUB,
            Opcode::DSUB => NativeOpcode::DSUB,
            Opcode::IMUL => NativeOpcode::IMUL,
            Opcode::LMUL => NativeOpcode::LMUL,
            Opcode::FMUL => NativeOpcode::FMUL,
            Opcode::DMUL => NativeOpcode::DMUL,
            Opcode::IDIV => NativeOpcode::IDIV,
            Opcode::LDIV => NativeOpcode::LDIV,
            Opcode::FDIV => NativeOpcode::FDIV,
            Opcode::DDIV => NativeOpcode::DDIV,
            Opcode::IREM => NativeOpcode::IREM,
            Opcode::LREM => NativeOpcode::LREM,
            Opcode::FREM => NativeOpcode::FREM,
            Opcode::DREM => NativeOpcode::DREM,
            Opcode::INEG => NativeOpcode::INEG,
            Opcode::LNEG => NativeOpcode::LNEG,
            Opcode::FNEG => NativeOpcode::FNEG,
            Opcode::DNEG => NativeOpcode::DNEG,
            Opcode::ISHL => NativeOpcode::ISHL,
            Opcode::LSHL => NativeOpcode::LSHL,
            Opcode::ISHR => NativeOpcode::ISHR,
            Opcode::LSHR => NativeOpcode::LSHR,
            Opcode::IUSHR => NativeOpcode::IUSHR,
            Opcode::LUSHR => NativeOpcode::LUSHR,
            Opcode::IAND => NativeOpcode::IAND,
            Opcode::LAND => NativeOpcode::LAND,
            Opcode::IOR => NativeOpcode::IOR,
            Opcode::LOR => NativeOpcode::LOR,
            Opcode::IXOR => NativeOpcode::IXOR,
            Opcode::LXOR => NativeOpcode::LXOR,
            Opcode::IINC(_, _) => NativeOpcode::IINC,
            Opcode::I2L => NativeOpcode::I2L,
            Opcode::I2F => NativeOpcode::I2F,
            Opcode::I2D => NativeOpcode::I2D,
            Opcode::L2I => NativeOpcode::L2I,
            Opcode::L2F => NativeOpcode::L2F,
            Opcode::L2D => NativeOpcode::L2D,
            Opcode::F2I => NativeOpcode::F2I,
            Opcode::F2L => NativeOpcode::F2L,
            Opcode::F2D => NativeOpcode::F2D,
            Opcode::D2I => NativeOpcode::D2I,
            Opcode::D2L => NativeOpcode::D2L,
            Opcode::D2F => NativeOpcode::D2F,
            Opcode::I2B => NativeOpcode::I2B,
            Opcode::I2C => NativeOpcode::I2C,
            Opcode::I2S => NativeOpcode::I2S,
            Opcode::LCMP => NativeOpcode::LCMP,
            Opcode::FCMPL => NativeOpcode::FCMPL,
            Opcode::FCMPG => NativeOpcode::FCMPG,
            Opcode::DCMPL => NativeOpcode::DCMPL,
            Opcode::DCMPG => NativeOpcode::DCMPG,
            Opcode::IFEQ(_) => NativeOpcode::IFEQ,
            Opcode::IFNE(_) => NativeOpcode::IFNE,
            Opcode::IFLT(_) => NativeOpcode::IFLT,
            Opcode::IFGE(_) => NativeOpcode::IFGE,
            Opcode::IFGT(_) => NativeOpcode::IFGT,
            Opcode::IFLE(_) => NativeOpcode::IFLE,
            Opcode::IF_ICMPEQ(_) => NativeOpcode::IF_ICMPEQ,
            Opcode::IF_ICMPNE(_) => NativeOpcode::IF_ICMPNE,
            Opcode::IF_ICMPLT(_) => NativeOpcode::IF_ICMPLT,
            Opcode::IF_ICMPGE(_) => NativeOpcode::IF_ICMPGE,
            Opcode::IF_ICMPGT(_) => NativeOpcode::IF_ICMPGT,
            Opcode::IF_ICMPLE(_) => NativeOpcode::IF_ICMPLE,
            Opcode::IF_ACMPEQ(_) => NativeOpcode::IF_ACMPEQ,
            Opcode::IF_ACMPNE(_) => NativeOpcode::IF_ACMPNE,
            Opcode::GOTO(_) => NativeOpcode::GOTO,
            Opcode::JSR(_) => NativeOpcode::JSR,
            Opcode::RET(_) => NativeOpcode::RET,
            Opcode::TABLESWITCH => NativeOpcode::TABLESWITCH,
            Opcode::LOOKUPSWITCH => NativeOpcode::LOOKUPSWITCH,
            Opcode::IRETURN => NativeOpcode::IRETURN,
            Opcode::LRETURN => NativeOpcode::LRETURN,
            Opcode::FRETURN => NativeOpcode::FRETURN,
            Opcode::DRETURN => NativeOpcode::DRETURN,
            Opcode::ARETURN => NativeOpcode::ARETURN,
            Opcode::RETURN => NativeOpcode::RETURN,
            Opcode::GETSTATIC => NativeOpcode::GETSTATIC,
            Opcode::PUTSTATIC => NativeOpcode::PUTSTATIC,
            Opcode::GETFIELD => NativeOpcode::GETFIELD,
            Opcode::PUTFIELD => NativeOpcode::PUTFIELD,
            Opcode::INVOKEVIRTUAL => NativeOpcode::INVOKEVIRTUAL,
            Opcode::INVOKESPECIAL => NativeOpcode::INVOKESPECIAL,
            Opcode::INVOKESTATIC => NativeOpcode::INVOKESTATIC,
            Opcode::INVOKEINTERFACE => NativeOpcode::INVOKEINTERFACE,
            Opcode::INVOKEDYNAMIC => NativeOpcode::INVOKEDYNAMIC,
            Opcode::NEW => NativeOpcode::NEW,
            Opcode::NEWARRAY => NativeOpcode::NEWARRAY,
            Opcode::ANEWARRAY => NativeOpcode::ANEWARRAY,
            Opcode::ARRAYLENGTH => NativeOpcode::ARRAYLENGTH,
            Opcode::ATHROW => NativeOpcode::ATHROW,
            Opcode::CHECKCAST => NativeOpcode::CHECKCAST,
            Opcode::INSTANCEOF => NativeOpcode::INSTANCEOF,
            Opcode::MONITORENTER => NativeOpcode::MONITORENTER,
            Opcode::MONITOREXIT => NativeOpcode::MONITOREXIT,
            Opcode::MULTIANEWARRAY => NativeOpcode::MULTIANEWARRAY,
            Opcode::IFNULL => NativeOpcode::IFNULL,
            Opcode::IFNONNULL => NativeOpcode::IFNONNULL,
        }
    }
    
    pub const fn opcode_byte(&self) -> u8 {
        self.native_opcode() as u8
    }
}

impl Eq for Opcode {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstantObject {
    String(String),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(String),
    /// # Arguments
    /// - [RefKind]: reference kind
    /// - [String]: class name
    /// - [String]: method name
    /// - [String]: method descriptor
    MethodHandle(RefKind, String, String, String),
    MethodType(String),
    /// # Arguments
    /// - [String]: method name
    /// - [String]: method descriptor
    /// - [Handle]: bootstrap method handle
    /// - [Vec<ConstantObject>]: bootstrap method arguments
    ConstantDynamic(String, String, Handle, Vec<ConstantObject>),
}

impl ConstantObject {
    /// Returns true if [ConstantObject] is [ConstantObject::Long], [ConstantObject::Double].
    pub(crate) const fn is_2(&self) -> bool {
        match self {
            ConstantObject::Long(..) | ConstantObject::Double(..) => true,
            _ => false,
        }
    }
}

impl Eq for ConstantObject {}

impl From<String> for ConstantObject {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ConstantObject {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<i32> for ConstantObject {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for ConstantObject {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for ConstantObject {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f64> for ConstantObject {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

pub const NOP: u8 = 0;
// visitInsn
pub const ACONST_NULL: u8 = 1;
// -
pub const ICONST_M1: u8 = 2;
// -
pub const ICONST_0: u8 = 3;
// -
pub const ICONST_1: u8 = 4;
// -
pub const ICONST_2: u8 = 5;
// -
pub const ICONST_3: u8 = 6;
// -
pub const ICONST_4: u8 = 7;
// -
pub const ICONST_5: u8 = 8;
// -
pub const LCONST_0: u8 = 9;
// -
pub const LCONST_1: u8 = 10;
// -
pub const FCONST_0: u8 = 11;
// -
pub const FCONST_1: u8 = 12;
// -
pub const FCONST_2: u8 = 13;
// -
pub const DCONST_0: u8 = 14;
// -
pub const DCONST_1: u8 = 15;
// -
pub const BIPUSH: u8 = 16;
// visitIntInsn
pub const SIPUSH: u8 = 17;
// -
pub const LDC: u8 = 18;
// visitLdcInsn
pub const ILOAD: u8 = 21;
// visitVarInsn
pub const LLOAD: u8 = 22;
// -
pub const FLOAD: u8 = 23;
// -
pub const DLOAD: u8 = 24;
// -
pub const ALOAD: u8 = 25;
// -
pub const IALOAD: u8 = 46;
// visitInsn
pub const LALOAD: u8 = 47;
// -
pub const FALOAD: u8 = 48;
// -
pub const DALOAD: u8 = 49;
// -
pub const AALOAD: u8 = 50;
// -
pub const BALOAD: u8 = 51;
// -
pub const CALOAD: u8 = 52;
// -
pub const SALOAD: u8 = 53;
// -
pub const ISTORE: u8 = 54;
// visitVarInsn
pub const LSTORE: u8 = 55;
// -
pub const FSTORE: u8 = 56;
// -
pub const DSTORE: u8 = 57;
// -
pub const ASTORE: u8 = 58;
// -
pub const IASTORE: u8 = 79;
// visitInsn
pub const LASTORE: u8 = 80;
// -
pub const FASTORE: u8 = 81;
// -
pub const DASTORE: u8 = 82;
// -
pub const AASTORE: u8 = 83;
// -
pub const BASTORE: u8 = 84;
// -
pub const CASTORE: u8 = 85;
// -
pub const SASTORE: u8 = 86;
// -
pub const POP: u8 = 87;
// -
pub const POP2: u8 = 88;
// -
pub const DUP: u8 = 89;
// -
pub const DUP_X1: u8 = 90;
// -
pub const DUP_X2: u8 = 91;
// -
pub const DUP2: u8 = 92;
// -
pub const DUP2_X1: u8 = 93;
// -
pub const DUP2_X2: u8 = 94;
// -
pub const SWAP: u8 = 95;
// -
pub const IADD: u8 = 96;
// -
pub const LADD: u8 = 97;
// -
pub const FADD: u8 = 98;
// -
pub const DADD: u8 = 99;
// -
pub const ISUB: u8 = 100;
// -
pub const LSUB: u8 = 101;
// -
pub const FSUB: u8 = 102;
// -
pub const DSUB: u8 = 103;
// -
pub const IMUL: u8 = 104;
// -
pub const LMUL: u8 = 105;
// -
pub const FMUL: u8 = 106;
// -
pub const DMUL: u8 = 107;
// -
pub const IDIV: u8 = 108;
// -
pub const LDIV: u8 = 109;
// -
pub const FDIV: u8 = 110;
// -
pub const DDIV: u8 = 111;
// -
pub const IREM: u8 = 112;
// -
pub const LREM: u8 = 113;
// -
pub const FREM: u8 = 114;
// -
pub const DREM: u8 = 115;
// -
pub const INEG: u8 = 116;
// -
pub const LNEG: u8 = 117;
// -
pub const FNEG: u8 = 118;
// -
pub const DNEG: u8 = 119;
// -
pub const ISHL: u8 = 120;
// -
pub const LSHL: u8 = 121;
// -
pub const ISHR: u8 = 122;
// -
pub const LSHR: u8 = 123;
// -
pub const IUSHR: u8 = 124;
// -
pub const LUSHR: u8 = 125;
// -
pub const IAND: u8 = 126;
// -
pub const LAND: u8 = 127;
// -
pub const IOR: u8 = 128;
// -
pub const LOR: u8 = 129;
// -
pub const IXOR: u8 = 130;
// -
pub const LXOR: u8 = 131;
// -
pub const IINC: u8 = 132;
// visitIincInsn
pub const I2L: u8 = 133;
// visitInsn
pub const I2F: u8 = 134;
// -
pub const I2D: u8 = 135;
// -
pub const L2I: u8 = 136;
// -
pub const L2F: u8 = 137;
// -
pub const L2D: u8 = 138;
// -
pub const F2I: u8 = 139;
// -
pub const F2L: u8 = 140;
// -
pub const F2D: u8 = 141;
// -
pub const D2I: u8 = 142;
// -
pub const D2L: u8 = 143;
// -
pub const D2F: u8 = 144;
// -
pub const I2B: u8 = 145;
// -
pub const I2C: u8 = 146;
// -
pub const I2S: u8 = 147;
// -
pub const LCMP: u8 = 148;
// -
pub const FCMPL: u8 = 149;
// -
pub const FCMPG: u8 = 150;
// -
pub const DCMPL: u8 = 151;
// -
pub const DCMPG: u8 = 152;
// -
pub const IFEQ: u8 = 153;
// visitJumpInsn
pub const IFNE: u8 = 154;
// -
pub const IFLT: u8 = 155;
// -
pub const IFGE: u8 = 156;
// -
pub const IFGT: u8 = 157;
// -
pub const IFLE: u8 = 158;
// -
pub const IF_ICMPEQ: u8 = 159;
// -
pub const IF_ICMPNE: u8 = 160;
// -
pub const IF_ICMPLT: u8 = 161;
// -
pub const IF_ICMPGE: u8 = 162;
// -
pub const IF_ICMPGT: u8 = 163;
// -
pub const IF_ICMPLE: u8 = 164;
// -
pub const IF_ACMPEQ: u8 = 165;
// -
pub const IF_ACMPNE: u8 = 166;
// -
pub const GOTO: u8 = 167;
// -
pub const JSR: u8 = 168;
// -
pub const RET: u8 = 169;
// visitVarInsn
pub const TABLESWITCH: u8 = 170;
// visiTableSwitchInsn
pub const LOOKUPSWITCH: u8 = 171;
// visitLookupSwitch
pub const IRETURN: u8 = 172;
// visitInsn
pub const LRETURN: u8 = 173;
// -
pub const FRETURN: u8 = 174;
// -
pub const DRETURN: u8 = 175;
// -
pub const ARETURN: u8 = 176;
// -
pub const RETURN: u8 = 177;
// -
pub const GETSTATIC: u8 = 178;
// visitFieldInsn
pub const PUTSTATIC: u8 = 179;
// -
pub const GETFIELD: u8 = 180;
// -
pub const PUTFIELD: u8 = 181;
// -
pub const INVOKEVIRTUAL: u8 = 182;
// visitMethodInsn
pub const INVOKESPECIAL: u8 = 183;
// -
pub const INVOKESTATIC: u8 = 184;
// -
pub const INVOKEINTERFACE: u8 = 185;
// -
pub const INVOKEDYNAMIC: u8 = 186;
// visitInvokeDynamicInsn
pub const NEW: u8 = 187;
// visitTypeInsn
pub const NEWARRAY: u8 = 188;
// visitIntInsn
pub const ANEWARRAY: u8 = 189;
// visitTypeInsn
pub const ARRAYLENGTH: u8 = 190;
// visitInsn
pub const ATHROW: u8 = 191;
// -
pub const CHECKCAST: u8 = 192;
// visitTypeInsn
pub const INSTANCEOF: u8 = 193;
// -
pub const MONITORENTER: u8 = 194;
// visitInsn
pub const MONITOREXIT: u8 = 195;
// -
pub const MULTIANEWARRAY: u8 = 197;
// visitMultiANewArrayInsn
pub const IFNULL: u8 = 198;
// visitJumpInsn
pub const IFNONNULL: u8 = 199; // -
