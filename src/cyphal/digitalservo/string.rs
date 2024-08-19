
const STRQTY_SIZE: usize = 1;

#[derive(Debug)]
pub struct Str {
    pub value: String
}

impl Str {
    pub fn serialize(value: &str) -> Vec<u8> {

        /* No. of str length */
        let str_size: usize = value.len();

        //TODO: error (length > 255)

        let buffer_size: usize = STRQTY_SIZE + str_size;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        let str_qty: u8 = str_size as u8;
        buffer.push(str_qty);

        let str_buffer: Vec<u8> = value.as_bytes().to_vec();
        buffer.extend(str_buffer);

        buffer
    }


    pub fn deserialize(bytearray: &[u8]) -> Result<Str, Box<dyn std::error::Error>>{

        let str_qty: usize = bytearray[0] as usize;

        //TODO: error (length > 255)

        let str_tail_pos: usize = STRQTY_SIZE + str_qty;
        let string: String = String::from_utf8_lossy(&bytearray[STRQTY_SIZE..str_tail_pos]).to_string();

        Ok( Str { value: string } )
    }
}