use std::convert::TryInto;

pub fn get_packet_num(buffer: &[u8]) -> u32 {
    u32::from_be_bytes(buffer.try_into().expect(""))
}