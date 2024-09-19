use super::traits::{DigitalServoDataType, DigitalServoPrimitiveData, IntoDigitalServoDataType};

const KEYLEN_SIZE: usize = 1;
const KEY_MAXLEN: usize = 32;
const KEY_MEM_LEN: usize = KEYLEN_SIZE + KEY_MAXLEN;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dict {
    pub key: String,
    pub value: Vec<DigitalServoPrimitiveData>,
}

impl Dict {
    pub fn serialize<T: Clone + IntoDigitalServoDataType + Into<DigitalServoPrimitiveData>>(key: &str, value: &[T]) -> Vec<u8> {

        /* Type code */
        let type_code_size: usize = 1;
        let data_type: DigitalServoDataType = T::data_type();
        let type_code: u8 = data_type.into_type_code();

        /* No. of values */
        let value_num_size: usize = data_type.sizeof_sizecode();
        let value_num: usize = match data_type {
            DigitalServoDataType::String => value[0].clone().into().get_char_len(),
            _ => value.len()
        };

        let value_num_vec: Vec<u8> = match value_num_size {
            1 => (value_num as u8).to_le_bytes().to_vec(),
            2 => (value_num as u16).to_le_bytes().to_vec(),
            4 => (value_num as u32).to_le_bytes().to_vec(),
            _ => vec![]
        };
        let value_size: usize = data_type.get_datasize(value_num);

        let buffer_size: usize = KEY_MEM_LEN + type_code_size + value_num_size + value_size;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        /* Key */
        let key_len: u8 = key.len() as u8;
        let key_buffer: Vec<u8> = key.as_bytes().to_vec();
        buffer.push(key_len);
        buffer.extend(key_buffer);

        buffer.push(type_code);
        buffer.extend(value_num_vec);

        /* Value */
        let value_buffer: Vec<u8> = unsafe {
            match data_type {
                DigitalServoDataType::String => {
                    value[0].clone().into().get_char_buffer()
                },
                DigitalServoDataType::Bool => {
                    let value: Vec<u8> = value
                        .iter()
                        .map(|x| -> DigitalServoPrimitiveData { x.clone().into() })
                        .collect::<Vec<DigitalServoPrimitiveData>>()
                        .chunks(8)
                        .map(|x| -> u8 {
                            x
                                .iter()
                                .enumerate()
                                .fold(0, |x, y| 
                                    if let DigitalServoPrimitiveData::Bool(v) = y.1 { x + ((*v as u8) << (y.0 as u8)) } else { x }
                                )
                        })
                        .collect();
                    std::slice::from_raw_parts(value.as_ptr() as *const u8, value_size).to_vec()
                },
                _ => std::slice::from_raw_parts(value.as_ptr() as *const u8, value_size).to_vec()
            }
        };

        buffer.extend(value_buffer);

        buffer
    }


    pub fn deserialize(bytearray: &[u8]) -> Result<Dict, Box<dyn std::error::Error>>{

        /* Key */
        let key_len: usize = bytearray[0] as usize;
        let key_mem_size: usize = KEYLEN_SIZE + key_len;
        let key: String = String::from_utf8_lossy(&bytearray[KEYLEN_SIZE..key_mem_size]).to_string();

        /* Type code */
        let type_code_size: usize = 1;
        let type_code: u8 = bytearray[key_mem_size];
        let data_type: DigitalServoDataType = DigitalServoDataType::try_from_type_code(type_code)?;

        /* Value */
        let value_num_size: usize = data_type.sizeof_sizecode();
        let value_num_addr: usize = key_mem_size + type_code_size;
        let value_num: usize = match value_num_size {
            1 => u8::from_le_bytes([bytearray[value_num_addr]]) as usize,
            2 => u16::from_le_bytes([bytearray[value_num_addr], bytearray[value_num_addr + 1]]) as usize,
            4 => u32::from_le_bytes([bytearray[value_num_addr], bytearray[value_num_addr + 1], bytearray[value_num_addr + 2], bytearray[value_num_addr + 3]]) as usize,
            _ => 0
        };

        let value_size: usize = data_type.get_datasize(value_num);
        let value_start_addr: usize = key_mem_size + type_code_size + value_num_size;
        let value_end_adddr: usize = value_start_addr + value_size;
        let value_bytes: Vec<u8> = Vec::from(&bytearray[value_start_addr..value_end_adddr]);

        let value: Vec<DigitalServoPrimitiveData> = match data_type {
            DigitalServoDataType::String => {
                vec![DigitalServoPrimitiveData::String(String::from_utf8_lossy(&value_bytes).to_string())]
            },
            DigitalServoDataType::Bool => {
                let mut buffer: Vec<u8> = Vec::with_capacity(value_bytes.len() * 8);
                for x in value_bytes {
                    for i in 0..8 {
                        buffer.push(x >> i & 0x01);
                    }
                }
                DigitalServoPrimitiveData::vec_from_buffer::<bool>(&buffer, value_num)
            },
            DigitalServoDataType::I64 => DigitalServoPrimitiveData::vec_from_buffer::<i64>(&value_bytes, value_num),
            DigitalServoDataType::I32 => DigitalServoPrimitiveData::vec_from_buffer::<i32>(&value_bytes, value_num),
            DigitalServoDataType::I16 => DigitalServoPrimitiveData::vec_from_buffer::<i16>(&value_bytes, value_num),
            DigitalServoDataType::I8 => DigitalServoPrimitiveData::vec_from_buffer::<i8>(&value_bytes, value_num),
            DigitalServoDataType::U64 => DigitalServoPrimitiveData::vec_from_buffer::<u64>(&value_bytes, value_num),
            DigitalServoDataType::U32 => DigitalServoPrimitiveData::vec_from_buffer::<u32>(&value_bytes, value_num),
            DigitalServoDataType::U16 => DigitalServoPrimitiveData::vec_from_buffer::<u16>(&value_bytes, value_num),
            DigitalServoDataType::U8 => DigitalServoPrimitiveData::vec_from_buffer::<u8>(&value_bytes, value_num),
            DigitalServoDataType::F64 => DigitalServoPrimitiveData::vec_from_buffer::<f64>(&value_bytes, value_num),
            DigitalServoDataType::F32 => DigitalServoPrimitiveData::vec_from_buffer::<f32>(&value_bytes, value_num),
        };

        Ok( Dict { key, value: Vec::from(value) } )
    }
}