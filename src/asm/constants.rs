// The ClassFile attribute names, in the order they are defined in
// https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.7-300.

use crate::asm::opcodes;

pub(crate) const CONSTANT_VALUE: &'static str = "ConstantValue";
pub(crate) const CODE: &'static str = "Code";
pub(crate) const STACK_MAP_TABLE: &'static str = "StackMapTable";
pub(crate) const EXCEPTIONS: &'static str = "Exceptions";
pub(crate) const INNER_CLASSES: &'static str = "InnerClasses";
pub(crate) const ENCLOSING_METHOD: &'static str = "EnclosingMethod";
pub(crate) const SYNTHETIC: &'static str = "Synthetic";
pub(crate) const SIGNATURE: &'static str = "Signature";
pub(crate) const SOURCE_FILE: &'static str = "SourceFile";
pub(crate) const SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
pub(crate) const LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
pub(crate) const LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
pub(crate) const LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
pub(crate) const DEPRECATED: &'static str = "Deprecated";
pub(crate) const RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
pub(crate) const RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
pub(crate) const RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeVisibleParameterAnnotations";
pub(crate) const RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str =
    "RuntimeInvisibleParameterAnnotations";
pub(crate) const RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
pub(crate) const RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str =
    "RuntimeInvisibleTypeAnnotations";
pub(crate) const ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
pub(crate) const BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
pub(crate) const METHOD_PARAMETERS: &'static str = "MethodParameters";
pub(crate) const MODULE: &'static str = "Module";
pub(crate) const MODULE_PACKAGES: &'static str = "ModulePackages";
pub(crate) const MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
pub(crate) const NEST_HOST: &'static str = "NestHost";
pub(crate) const NEST_MEMBERS: &'static str = "NestMembers";
pub(crate) const PERMITTED_SUBCLASSES: &'static str = "PermittedSubclasses";
pub(crate) const RECORD: &'static str = "Record";

// ASM specific access flags.
// WARNING: the 16 least significant bits must NOT be used, to avoid conflicts with standard
// access flags, and also to make sure that these flags are automatically filtered out when
// written in class files (because access flags are stored using 16 bits only).

pub(crate) const ACC_CONSTRUCTOR: u32 = 0x40000; // method access flag.

// ASM specific stack map frame types, used in {@link ClassVisitor#visitFrame}.

/**
 * A frame inserted between already existing frames. This internal stack map frame type (in
 * addition to the ones declared in {@link Opcodes}) can only be used if the frame content can be
 * computed from the previous existing frame and from the instructions between this existing frame
 * and the inserted one, without any knowledge of the type hierarchy. This kind of frame is only
 * used when an unconditional jump is inserted in a method while expanding an ASM specific
 * instruction. Keep in sync with opcodes::java.
 */
pub(crate) const F_INSERT: u32 = 256;

// The JVM opcode values which are not part of the ASM public API.
// See https://docs.oracle.com/javase/specs/jvms/se9/html/jvms-6.html.

pub(crate) const LDC_W: u8 = 19;
pub(crate) const LDC2_W: u8 = 20;
pub(crate) const ILOAD_0: u8 = 26;
pub(crate) const ILOAD_1: u8 = 27;
pub(crate) const ILOAD_2: u8 = 28;
pub(crate) const ILOAD_3: u8 = 29;
pub(crate) const LLOAD_0: u8 = 30;
pub(crate) const LLOAD_1: u8 = 31;
pub(crate) const LLOAD_2: u8 = 32;
pub(crate) const LLOAD_3: u8 = 33;
pub(crate) const FLOAD_0: u8 = 34;
pub(crate) const FLOAD_1: u8 = 35;
pub(crate) const FLOAD_2: u8 = 36;
pub(crate) const FLOAD_3: u8 = 37;
pub(crate) const DLOAD_0: u8 = 38;
pub(crate) const DLOAD_1: u8 = 39;
pub(crate) const DLOAD_2: u8 = 40;
pub(crate) const DLOAD_3: u8 = 41;
pub(crate) const ALOAD_0: u8 = 42;
pub(crate) const ALOAD_1: u8 = 43;
pub(crate) const ALOAD_2: u8 = 44;
pub(crate) const ALOAD_3: u8 = 45;
pub(crate) const ISTORE_0: u8 = 59;
pub(crate) const ISTORE_1: u8 = 60;
pub(crate) const ISTORE_2: u8 = 61;
pub(crate) const ISTORE_3: u8 = 62;
pub(crate) const LSTORE_0: u8 = 63;
pub(crate) const LSTORE_1: u8 = 64;
pub(crate) const LSTORE_2: u8 = 65;
pub(crate) const LSTORE_3: u8 = 66;
pub(crate) const FSTORE_0: u8 = 67;
pub(crate) const FSTORE_1: u8 = 68;
pub(crate) const FSTORE_2: u8 = 69;
pub(crate) const FSTORE_3: u8 = 70;
pub(crate) const DSTORE_0: u8 = 71;
pub(crate) const DSTORE_1: u8 = 72;
pub(crate) const DSTORE_2: u8 = 73;
pub(crate) const DSTORE_3: u8 = 74;
pub(crate) const ASTORE_0: u8 = 75;
pub(crate) const ASTORE_1: u8 = 76;
pub(crate) const ASTORE_2: u8 = 77;
pub(crate) const ASTORE_3: u8 = 78;
pub(crate) const WIDE: u8 = 196;
pub(crate) const GOTO_W: u8 = 200;
pub(crate) const JSR_W: u8 = 201;

// Constants to convert between normal and wide jump instructions.

// The delta between the GOTO_W and JSR_W opcodes and GOTO and JUMP.
pub(crate) const WIDE_JUMP_OPCODE_DELTA: u8 = GOTO_W - opcodes::GOTO;

// Constants to convert JVM opcodes to the equivalent ASM specific opcodes, and vice versa.

// The delta between the ASM_IFEQ, ..., ASM_IF_ACMPNE, ASM_GOTO and ASM_JSR opcodes
// and IFEQ, ..., IF_ACMPNE, GOTO and JSR.
pub(crate) const ASM_OPCODE_DELTA: u8 = 49;

// The delta between the ASM_IFNULL and ASM_IFNONNULL opcodes and IFNULL and IFNONNULL.
pub(crate) const ASM_IFNULL_OPCODE_DELTA: u8 = 20;

// ASM specific opcodes, used for long forward jump instructions.

pub(crate) const ASM_IFEQ: u8 = opcodes::IFEQ + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFNE: u8 = opcodes::IFNE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFLT: u8 = opcodes::IFLT + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFGE: u8 = opcodes::IFGE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFGT: u8 = opcodes::IFGT + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFLE: u8 = opcodes::IFLE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPEQ: u8 = opcodes::IF_ICMPEQ + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPNE: u8 = opcodes::IF_ICMPNE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPLT: u8 = opcodes::IF_ICMPLT + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPGE: u8 = opcodes::IF_ICMPGE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPGT: u8 = opcodes::IF_ICMPGT + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ICMPLE: u8 = opcodes::IF_ICMPLE + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ACMPEQ: u8 = opcodes::IF_ACMPEQ + ASM_OPCODE_DELTA;
pub(crate) const ASM_IF_ACMPNE: u8 = opcodes::IF_ACMPNE + ASM_OPCODE_DELTA;
pub(crate) const ASM_GOTO: u8 = opcodes::GOTO + ASM_OPCODE_DELTA;
pub(crate) const ASM_JSR: u8 = opcodes::JSR + ASM_OPCODE_DELTA;
pub(crate) const ASM_IFNULL: u8 = opcodes::IFNULL + ASM_IFNULL_OPCODE_DELTA;
pub(crate) const ASM_IFNONNULL: u8 = opcodes::IFNONNULL + ASM_IFNULL_OPCODE_DELTA;
pub(crate) const ASM_GOTO_W: u8 = 220;

pub struct ConstantDynamic {
    name: String,
    descriptor: String,
}
