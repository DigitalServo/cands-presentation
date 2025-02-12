use super::DigitalServoDataType;

mod impls;

pub trait IntoDigitalServoDataType {
    fn data_type() -> DigitalServoDataType;
}