use bitflags::bitflags;

use serde::{Deserialize, Serialize};

bitflags! {
    /// Access flag for [node::class::Class].
    ///
    /// See [Table 4.1-B](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=85).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ClassAccessFlag: u16 {
        const Public = 0x0001;
        const Final = 0x0010;
        const Super = 0x0020;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
        const Module = 0x8000;
    }

    /// Access flag for [node::field::Field].
    ///
    /// See [Table 4.5-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=110).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct FieldAccessFlag: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Volatile = 0x0040;
        const Transient = 0x0080;
        const Synthetic = 0x1000;
        const Enum = 0x4000;
    }

    /// Access flag for [node::method::Method].
    ///
    /// See [Table 4.6-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=112).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct MethodAccessFlag: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Synchronized = 0x0020;
        const Bridge = 0x0040;
        const Varargs = 0x0080;
        const Native = 0x0100;
        const Abstract = 0x0400;
        const Strict = 0x0800;
        const Synthetic = 0x1000;
    }

    /// Access flag for [node::attribute::InnerClass].
    ///
    /// See [Table 4.7.6-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=138).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct NestedClassAccessFlag: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
    }

    /// Access flag for [node::attribute::MethodParameter].
    ///
    /// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=183).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ParameterAccessFlag: u16 {
        const Final = 0x0010;
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }

    /// Access flag for [node::attribute::Module].
    ///
    /// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=186).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ModuleAccessFlag: u16 {
        const Open = 0x0020;
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }

    /// Access flag for [node::attribute::module::Requires].
    ///
    /// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=187).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct RequiresAccessFlag: u16 {
        const Transitive = 0x0020;
        const StaticPhase = 0x0040;
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }

    /// Access flag for [node::attribute::module::Exports].
    ///
    /// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=188).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ExportsAccessFlag: u16 {
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }

    /// Access flag for [node::attribute::module::Opens].
    ///
    /// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=189).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct OpensAccessFlag: u16 {
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }
}
