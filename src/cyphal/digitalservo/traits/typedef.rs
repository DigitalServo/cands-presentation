#[derive(core::cmp::PartialEq)]
pub enum DigitalServoDataType {
    String,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64
}

pub trait IntoDigitalServoDataType {
    fn data_type() -> DigitalServoDataType;
}

impl IntoDigitalServoDataType for String {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::String }
}

impl IntoDigitalServoDataType for bool {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::Bool }
}

impl IntoDigitalServoDataType for i64 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::I64 }
}

impl IntoDigitalServoDataType for i32 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::I32 }
}

impl IntoDigitalServoDataType for i16 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::I16 }
}

impl IntoDigitalServoDataType for i8 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::I8 }
}

impl IntoDigitalServoDataType for u64 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::U64 }
}

impl IntoDigitalServoDataType for u32 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::U32 }
}

impl IntoDigitalServoDataType for u16 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::U16 }
}

impl IntoDigitalServoDataType for u8 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::U8 }
}

impl IntoDigitalServoDataType for f64 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::F64 }
}

impl IntoDigitalServoDataType for f32 {
    fn data_type() -> DigitalServoDataType { DigitalServoDataType::F32 }
}

impl DigitalServoDataType {
    pub fn into_type_code(&self) -> u8 {
        match self {
            Self::String => 0x01,
            Self::Bool => 0x03,
            Self::I64 => 0x04,
            Self::I32 => 0x05,
            Self::I16 => 0x06,
            Self::I8 => 0x07,
            Self::U64 => 0x08,
            Self::U32 => 0x09,
            Self::U16 => 0x0a,
            Self::U8 => 0x0b,
            Self::F64 => 0x0c,
            Self::F32 => 0x0d,
        }
    }

    pub fn try_from_type_code(type_code: u8) -> Result<Self, Box<dyn std::error::Error>> {
        match type_code {
            0x01 => Ok(Self::String),
            0x03 => Ok(Self::Bool),
            0x04 => Ok(Self::I64),
            0x05 => Ok(Self::I32),
            0x06 => Ok(Self::I16),
            0x07 => Ok(Self::I8),
            0x08 => Ok(Self::U64),
            0x09 => Ok(Self::U32),
            0x0a => Ok(Self::U16),
            0x0b => Ok(Self::U8),
            0x0c => Ok(Self::F64),
            0x0d => Ok(Self::F32),
            _ => Err("Unknown type code.".into())
        }
    }

    pub fn sizeof_sizecode(&self) -> usize {
        match self {
            Self::String => 1,
            Self::Bool => 1,
            Self::I64 => 1,
            Self::I32 => 1,
            Self::I16 => 1,
            Self::I8 => 1,
            Self::U64 => 1,
            Self::U32 => 1,
            Self::U16 => 1,
            Self::U8 => 1,
            Self::F64 => 1,
            Self::F32 => 1,
        } 
    }

    pub fn get_datasize(&self, len: usize) -> usize {
        match self {
            Self::String => std::mem::size_of::<u8>() * len,
            Self::Bool => (len as f32 / 8.0).ceil() as usize,
            Self::I64 => std::mem::size_of::<i64>() * len,
            Self::I32 => std::mem::size_of::<i32>() * len,
            Self::I16 => std::mem::size_of::<i16>() * len,
            Self::I8 => std::mem::size_of::<i8>() * len,
            Self::U64 => std::mem::size_of::<u64>() * len,
            Self::U32 => std::mem::size_of::<u32>() * len,
            Self::U16 => std::mem::size_of::<u16>() * len,
            Self::U8 => std::mem::size_of::<u8>() * len,
            Self::F64 => std::mem::size_of::<f64>() * len,
            Self::F32 => std::mem::size_of::<f32>() * len,
        } 
    }
}