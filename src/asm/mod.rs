pub use handle::Handle;

/// The JVM opcodes, access flags and array type codes. This interface does not define all the JVM
/// opcodes because some opcodes are automatically handled. For example, the xLOAD and xSTORE opcodes
/// are automatically replaced by xLOAD_n and xSTORE_n opcodes when possible. The xLOAD_n and
/// xSTORE_n opcodes are therefore not defined in this interface. Likewise for LDC, automatically
/// replaced by LDC_W or LDC2_W when necessary, WIDE, GOTO_W and JSR_W.
///
/// See <a href="https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-6.html">JVMS 6</a><br/>
/// **Author** Eric Bruneton<br/>
/// **Author** Eugene Kuleshov<br/>
pub mod opcodes;
pub mod byte_vec;
#[allow(unused)]
mod constants;
mod edge;
mod frame;
mod label;
mod symbol;
pub mod types;
mod attribute;
mod handle;
mod handler;