use std::convert::TryInto;

use i24::i24;

pub fn convert_i24_buf_to_le_i32(old_buf: &[u8]) -> Vec<i32> {
    // This method converts a buffer of bytes (that's actually a list of i24), and converts them
    // to i32. Note: new_buf must be large enough to fit
    return old_buf
        .chunks_exact(3)
        .map(|s| i24::from_le_bytes(s.try_into().unwrap()).to_i32())
        .collect();
}

pub fn convert_i32_list_to_bytes(i32_buf: &[i32]) -> Vec<u8> {
    return i32_buf
        .into_iter()
        .map(|i| i32::to_le_bytes(*i))
        .into_iter()
        .flatten()
        .collect();
}
