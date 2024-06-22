#[derive(Debug, Clone)]
pub enum DigitalServoPrimitiveData {
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl From<String> for DigitalServoPrimitiveData {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}

impl From<bool> for DigitalServoPrimitiveData {
    fn from(val: bool) -> Self {
        Self::Bool(val)
    }
}

impl From<i8> for DigitalServoPrimitiveData {
    fn from(val: i8) -> Self {
        Self::I8(val)
    }
}

impl From<i16> for DigitalServoPrimitiveData {
    fn from(val: i16) -> Self {
        Self::I16(val)
    }
}

impl From<i32> for DigitalServoPrimitiveData {
    fn from(val: i32) -> Self {
        Self::I32(val)
    }
}

impl From<i64> for DigitalServoPrimitiveData {
    fn from(val: i64) -> Self {
        Self::I64(val)
    }
}

impl From<u8> for DigitalServoPrimitiveData {
    fn from(val: u8) -> Self {
        Self::U8(val)
    }
}

impl From<u16> for DigitalServoPrimitiveData {
    fn from(val: u16) -> Self {
        Self::U16(val)
    }
}

impl From<u32> for DigitalServoPrimitiveData {
    fn from(val: u32) -> Self {
        Self::U32(val)
    }
}

impl From<u64> for DigitalServoPrimitiveData {
    fn from(val: u64) -> Self {
        Self::U64(val)
    }
}

impl From<f32> for DigitalServoPrimitiveData {
    fn from(val: f32) -> Self {
        Self::F32(val)
    }
}

impl From<f64> for DigitalServoPrimitiveData {
    fn from(val: f64) -> Self {
        Self::F64(val)
    }
}

impl DigitalServoPrimitiveData {
    pub fn vec_from_buffer<T: Into<DigitalServoPrimitiveData> + Copy>(buffer: &[u8], len: usize) -> Vec<Self>{
        unsafe {
            std::slice::from_raw_parts(buffer.as_ptr() as *const T, len)
                .iter()
                .map(|&x| x.into())
                .collect()
        }
    }

    pub fn get_char_len(&self) -> usize {
        match self{
            Self::String(x) => x.len(),
            _ => 0
        }
    }

    pub fn get_char_buffer(&self) -> Vec<u8> {
        match self{
            Self::String(x) => x.as_bytes().to_vec(),
            _ => vec![]
        }
    }

}
