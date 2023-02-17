use super::HD_chapter_2::nlz;

//find leftmost zero byte
pub fn zbyte1(x: i64) -> i64 {
    if (x >> 24) == 0 {
        0
    } else if (x & 0x00FF0000) == 0 {
        1
    } else if (x & 0x0000FF00) == 0 {
        2
    } else if (x & 0x000000FF) == 0 {
        3
    } else {
        4
    }
}

//branch free zero byte
//convert each zero byte to 0x80 and nonzero to 0x00, then using number of leading zero
//can be modified to find other values in given range as we'll see soon
pub fn zbyte2(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F; //1xxxxxxx
    y = !(y | x | 0x7F7F7F7F); //80 00 00 00 00 etc
    let n = nlz(y as u32) >> 3;
    n as i64
}

//find left most 0-byte not using nlz
pub fn zbyte3(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F;
    y = !(y | x | 0x7F7F7F7F);
    if y == 0 {
        4
    } else if y > 0x0000FFFF {
        (y >> 31) ^ 1
    } else {
        (y >> 15) ^ 3
    }
}

//find leftmost byte having less than 9
pub fn le_9_byte(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x76767676;
    y |= x;
    y |= 0x7F7F7F7F; // > 9 are 0xFF
    y = !y; // 0x00
            //<= 9 are 0x80
    let n = nlz(y as u32) >> 3;
    n as i64
}

//left most uppercase letter
pub fn uppercase_byte(x: i64) -> i64 {
    let mut d = (x | 0x80808080) - 0x41414141;
    d = !((x | 0x7F7F7F7F) ^ d);
    let mut y = (d & 0x7F7F7F7F) + 0x66666666;
    y |= d;
    y |= 0x7F7F7F7F; //0xFF again
    y = !y; //0x00 again
    let n = nlz(y as u32) >> 3;
    n as i64
}

//finds length and position of shortest contiguous string of 1s in word
//if n-consecutive 1-bits are found, returns number 0 to 31, otherwise if
//not found returns 32
pub fn one_bit1(x: i32, n: &mut i32) -> i32 {
    if x == 0 {
        *n = 32;
        return 0;
    } //initalize to return position
    let b = !(x >> 1) & x;
    let mut c = x & !(x << 1);
    let mut k = 1;
    loop {
        if b & c != 0 {
            break;
        }
        c <<= 1;
        k += 1;
    }
    *n = nlz((b & c) as u32) as i32;
    k
}

//first string of 1s of n given len
//can be found in 6 instructions using number of leading zeros
pub fn ffstr11(mut x: i32, n: i32) -> i32 {
    let mut p = 0;
    while x != 0 {
        let mut k = nlz(x as u32);
        x <<= k; //skip over leading zeros
        p += k;
        k = nlz(!x as u32);
        if k >= n as u32 {
            return p as i32;
        } //return if position is enough for N
        x <<= k; //else skip over 1s
        p += k;
    }
    32
}

//first string of 1s of n given len using shif-and sequence
pub fn ffstr12(x: i32, mut n: i32) -> i32 {
    while n > 1 {
        let s = n >> 1;
        let _x = x & (x << s);
        n -= s;
    }
    nlz(x as u32) as i32
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn search_test1() {
    assert_eq!(ffstr11(1, 1), 31);
    assert_eq!(ffstr12(1, 1), 31);
}
