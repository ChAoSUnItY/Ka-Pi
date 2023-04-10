// ASM API versions.
// These APIs are unused at this moments.

use serde::{Deserialize, Serialize};

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

#[repr(u16)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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

#[repr(u16)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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

#[repr(u16)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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
    MULTIANEWARRAY = 197,
    IFNULL = 198,
    IFNONNULL = 199,
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
