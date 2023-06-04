use anyhow::Result;
use std::str;

pub fn strlen(data: &[u8], offset: usize) -> usize {
    let mut len = 0;
    while data[offset + len] != 0 {
        len += 1;
    }
    len
}
pub fn get_str(data: &[u8], offset: usize) -> Result<&str> {
    let len = strlen(data, offset);
    let str = str::from_utf8(&data[offset..offset + len])?;
    Ok(str)
}

pub unsafe fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
    let mut nul_range_end = 1_usize;
    for b in utf8_src {
        if *b == 0 {
            break;
        }
        nul_range_end += 1;
    }
    return ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end]);
}
