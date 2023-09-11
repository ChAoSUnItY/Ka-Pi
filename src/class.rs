use crate::{
    access_flag::ClassAccessFlag,
    byte_vec::{ByteVec, ToBytes, SizeComputable},
    symbol::ConstantPool, attrs,
};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
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

impl Default for JavaVersion {
    fn default() -> Self {
        Self::V17
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

    fn visit_source(&mut self, source_file: &str) {
        if let Some(inner) = self.inner() {
            inner.visit_source(source_file);
        }
    }

    fn visit_debug_extension(&mut self, debug_extension: &str) {
        if let Some(inner) = self.inner() {
            inner.visit_debug_extension(debug_extension);
        }
    }

    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct ClassWriter {
    version: JavaVersion,
    access: ClassAccessFlag,
    constant_pool: ConstantPool,
    this_class: Option<u16>,
    signature: Option<u16>,
    super_class: Option<u16>,
    interfaces: Vec<u16>,
    // Attribute SourceFile
    source: Option<u16>,
    debug_extension: Option<Vec<u8>>,
}

impl ClassWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let size = self.compute_size();
        // We avoid additional reallocation by precomputing the
        // class file size based on spec
        let mut vec = ByteVec::with_capacity(size);

        self.put_bytes(&mut vec);

        vec.as_vec()
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
        self.this_class = Some(self.constant_pool.put_class(name));

        if let Some(signature) = signature {
            self.constant_pool.put_utf8(attrs::SIGNATURE);
            self.signature = Some(self.constant_pool.put_class(signature));
        }

        self.super_class = Some(self.constant_pool.put_class(super_name));
        self.interfaces = interfaces
            .into_iter()
            .map(|interface| self.constant_pool.put_class(interface))
            .collect()
    }

    fn visit_source(&mut self, source_file: &str) {
        self.constant_pool.put_utf8(attrs::SOURCE_FILE);
        self.source = Some(self.constant_pool.put_utf8(source_file));
    }

    fn visit_debug_extension(&mut self, debug_extension: &str) {
        self.constant_pool.put_utf8(attrs::SOURCE_DEBUG_EXTENSION);
        self.debug_extension = Some(cesu8::to_java_cesu8(debug_extension).to_vec());
    }
}

impl ToBytes for ClassWriter {
    fn put_bytes(&self, vec: &mut ByteVec) {
        vec.push_u32(0xCAFEBABE)
            .push_u32(self.version.version());

        self.constant_pool.put_bytes(vec);

        vec.push_u16(self.access.bits())
            .push_u16(self.this_class.expect("This class is unset, probably missing `visit` call?"))
            .push_u16(self.super_class.expect("Super class is unset, probably missing `visit` call?"))
            .push_u16(self.interfaces.len() as u16);

        for interface in &self.interfaces {
            vec.push_u16(*interface);
        }

        // TODO: Field
        vec.push_u16(0);
        // TODO: Method
        vec.push_u16(0);
        // TODO: Attribute
        vec.push_u16(self.attributes_count() as u16);

        if let Some(signature) = self.signature {
            vec.push_u16(self.constant_pool.get_utf8(attrs::SIGNATURE).unwrap())
                .push_u32(2)
                .push_u16(signature);
        }

        if let Some(source) = self.source {
            vec.push_u16(self.constant_pool.get_utf8(attrs::SOURCE_FILE).unwrap())
                .push_u32(2)
                .push_u16(source);
        }

        if let Some(debug_extension) = &self.debug_extension {            
            vec.push_u16(self.constant_pool.get_utf8(attrs::SOURCE_DEBUG_EXTENSION).unwrap())
                .push_u32(debug_extension.len() as u32)
                .push_u8s(&debug_extension);
        }
    }
}

impl SizeComputable for ClassWriter {
    fn compute_size(&self) -> usize {
        let mut size = 24 + 2 * self.interfaces.len();
        // TODO: Fields
        // TODO: Methods
        // TODO: Attributes
        if self.signature.is_some() {
            size += 8;
        }
        if self.source.is_some() {
            size += 8;
        }
        if let Some(debug_extension) = &self.debug_extension {
            size += 6 + debug_extension.len();
        }
        size
    }

    fn attributes_count(&self) -> usize {
        let mut count = 0;
        
        if self.signature.is_some() {
            count += 1;
        }

        if self.source.is_some() {
            count += 1;
        }

        if self.debug_extension.is_some() {
            count += 1;
        }

        count
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

        writer.visit_source("Main.java");
        writer.visit_debug_extension("Debug Message");

        writer.visit_end();

        let bytes = writer.to_bytes();

        fs::write("output/Main.class", bytes).expect("Unexpected error while writing class file bytecode");
    }
}
