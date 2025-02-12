use super::IntoDigitalServoDataType;
use super::super::DigitalServoDataType;

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