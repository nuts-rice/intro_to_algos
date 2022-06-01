//Rounding up or down
// clp2 defined as similar to 'ceiling' but directed roundings to closest intergal power of 2
//right-propogation rounding up to next ^2
pub fn clp2_0(mut x: i32) -> i32 {
    x = x - 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    return x + 1;
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_clp2() {
    assert_eq!(clp2_0(0), 0);
}
//cyclic redundancy checks

pub fn crc_reverse(mut x: i32) -> i32 {
    x = ((x & 0x55555555) << 1) | ((x >> 1) & 0x55555555);
    x = ((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333);
    x = ((x & 0x0F0F0F0F) << 4) | ((x >> 4) & 0x0F0F0F0F);
    x = (x << 24 ) | ((x & 0xFF00) << 8) |
         ((x >> 8) & 0xFF00) | (x >> 24);
    return x;
}

    
#[allow(overflowing_literals)]
pub fn crc32a(message: &[char]) -> i32 {
    let mut i = 0;
    let mut crc = 0xFFFFFFFF;
    while message[1] != '\x00' {
        let mut byte = message[1]; //next byte
        byte = crc_reverse((byte as u8) as i32) as u8 as char;
        for j in 0..7 { //8 times
            if (crc ^ (byte as i32)) < 0 {
                crc = (crc << 1) ^ 0x04C11DB7;
            } else { crc = crc << 1; }
            byte = ((byte as u8) << 1) as char; //next message bit
        }
        i = i + 1;
    }
    return crc_reverse(!crc);
}
