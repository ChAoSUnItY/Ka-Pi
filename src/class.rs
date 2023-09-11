use crate::{
    access_flag::ClassAccessFlag,
    byte_vec::{ByteVec, ToBytes},
    symbol::ConstantPool,
};

#[derive(Debug, Clone, Copy)]
pub enum JavaVersion {
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    V1_7,
    V1_8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    Custom { minor: u16, major: u16 },
}

impl JavaVersion {
    pub fn version(&self) -> u32 {
        match self {
            Self::V1_1 => 3 << 16 | 45,
            Self::V1_2 => 0 << 16 | 46,
            Self::V1_3 => 0 << 16 | 47,
            Self::V1_4 => 0 << 16 | 48,
            Self::V1_5 => 0 << 16 | 49,
            Self::V1_6 => 0 << 16 | 50,
            Self::V1_7 => 0 << 16 | 51,
            Self::V1_8 => 0 << 16 | 52,
            Self::V9 => 0 << 16 | 53,
            Self::V10 => 0 << 16 | 54,
            Self::V11 => 0 << 16 | 55,
            Self::V12 => 0 << 16 | 56,
            Self::V13 => 0 << 16 | 57,
            Self::V14 => 0 << 16 | 58,
            Self::V15 => 0 << 16 | 59,
            Self::V16 => 0 << 16 | 60,
            Self::V17 => 0 << 16 | 61,
            Self::V18 => 0 << 16 | 62,
            Self::V19 => 0 << 16 | 63,
            Self::V20 => 0 << 16 | 64,
            Self::V21 => 0 << 16 | 65,
            JavaVersion::Custom { minor, major } => {
                let minor = minor.to_be_bytes();
                let major = major.to_be_bytes();

                u32::from_be_bytes([minor[0], minor[1], major[0], major[1]])
            }
        }
    }
}

pub trait ClassVisitor {
    fn inner(&mut self) -> Option<&mut dyn ClassVisitor> {
        None
    }

    fn visit(
        &mut self,
        version: JavaVersion,
        access: ClassAccessFlag,
        name: &str,
        signature: Option<&str>,
        super_name: &str,
        interfaces: &[&str],
    ) {
        if let Some(inner) = self.inner() {
            inner.visit(version, access, name, signature, super_name, interfaces);
        }
    }

    fn visit_source(&mut self, source: &str, debug: &str) {
        if let Some(inner) = self.inner() {
            inner.visit_source(source, debug);
        }
    }

    fn visit_end(&mut self) {}
}

#[derive(Debug)]
pub struct ClassWriter {
    version: JavaVersion,
    access: ClassAccessFlag,
    constant_pool: ConstantPool,
    this_class: u16,
    signature: Option<u16>,
    super_class: u16,
    interfaces: Vec<u16>,
}

impl ClassWriter {
    pub fn new() -> Self {
        Self {
            version: JavaVersion::V17,
            access: ClassAccessFlag::empty(),
            constant_pool: ConstantPool::default(),
            this_class: 0,
            signature: None,
            super_class: 0,
            interfaces: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = ByteVec::with_capacity(64);

        output.push_u32(0xCAFEBABE);
        output.push_u32(self.version.version());

        self.constant_pool.put_bytes(&mut output);

        output.push_u16(self.access.bits());
        output.push_u16(self.this_class);
        output.push_u16(self.super_class);
        output.push_u16(self.interfaces.len() as u16);

        for interface in &self.interfaces {
            output.push_u16(*interface);
        }

        // TODO: Field
        output.push_u16(0);
        // TODO: Method
        output.push_u16(0);
        // TODO: Attribute
        output.push_u16(0);

        output.as_vec()
    }
}

impl ClassVisitor for ClassWriter {
    fn visit(
        &mut self,
        version: JavaVersion,
        access: ClassAccessFlag,
        name: &str,
        signature: Option<&str>,
        super_name: &str,
        interfaces: &[&str],
    ) {
        self.version = version;
        self.access = access;
        self.this_class = self.constant_pool.put_class(name);
        self.signature = signature.map(|signature| self.constant_pool.put_class(signature));
        self.super_class = self.constant_pool.put_class(super_name);
        self.interfaces = interfaces
            .into_iter()
            .map(|interface| self.constant_pool.put_class(interface))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::access_flag::ClassAccessFlag;

    use super::{ClassVisitor, ClassWriter, JavaVersion};

    #[test]
    fn test_basic_class_gen() {
        let mut writer = ClassWriter::new();

        writer.visit(
            JavaVersion::V17,
            ClassAccessFlag::Super | ClassAccessFlag::Public,
            "Main",
            None,
            "java/lang/Object",
            &[],
        );

        writer.visit_end();

        let bytes = writer.to_bytes();

        fs::write("output/Main.class", bytes).expect("Unexpected error while writing class file bytecode");
    }
}
