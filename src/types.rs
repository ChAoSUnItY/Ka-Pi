pub const VOID: u8 = 0;

/** The sort of the {@code boolean} type. See {@link #getSort}. */
pub const BOOLEAN: u8 = 1;

/** The sort of the {@code char} type. See {@link #getSort}. */
pub const CHAR: u8 = 2;

/** The sort of the {@code byte} type. See {@link #getSort}. */
pub const BYTE: u8 = 3;

/** The sort of the {@code short} type. See {@link #getSort}. */
pub const SHORT: u8 = 4;

/** The sort of the {@code int} type. See {@link #getSort}. */
pub const INT: u8 = 5;

/** The sort of the {@code float} type. See {@link #getSort}. */
pub const FLOAT: u8 = 6;

/** The sort of the {@code long} type. See {@link #getSort}. */
pub const LONG: u8 = 7;

/** The sort of the {@code double} type. See {@link #getSort}. */
pub const DOUBLE: u8 = 8;

/** The sort of array reference types. See {@link #getSort}. */
pub const ARRAY: u8 = 9;

/** The sort of object reference types. See {@link #getSort}. */
pub const OBJECT: u8 = 10;

/** The sort of method types. See {@link #getSort}. */
pub const METHOD: u8 = 11;

/** The (private) sort of object reference types represented with an internal name. */
const INTERNAL: u8 = 12;

/** The descriptors of the primitive types. */
const PRIMITIVE_DESCRIPTORS: &'static str = "VZCBSIFJD";

pub struct Type {
    sort: u8,
    value_buffer: String,
    value_begin: usize,
    value_end: usize,
}

impl Type {
    pub const fn new(sort: u8, value_buffer: String, value_begin: usize, value_end: usize) -> Self {
        Self {
            sort,
            value_buffer,
            value_begin,
            value_end,
        }
    }
}
