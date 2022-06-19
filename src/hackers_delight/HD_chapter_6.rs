use HD_chapter_2::{nlz, population_count};

//find leftmost zero byte
pub fn zbyte1(x: i64) -> i64{
    return if (x >> 24) == 0 {0} else if (x & 0x00FF0000) == 0 {1} else if (x & 0x0000FF00) == 0 {2} else if (x & 0x000000FF) == 0 {3} else {4};
}

//branch free zero byte
//convert each zero byte to 0x80 and nonzero to 0x00, then using number of leading zero
//can be modified to find other values in given range as we'll see soon 
pub fn zbyte2(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F; //1xxxxxxx
    y = !(y | x 0x7F7F7F7F);    //80 00 00 00 00 etc
    let n = nlz(y as u32) >> 3;
    return n as i64;
}

//find left most 0-byte not using nlz
pub fn zbyte3(x: i64) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x7F7F7F7F;
    y = !(y | x | 0x7F7F7F7F);
    return if y == 0 {4}
    else if y > 0x0000FFFF
    {( y >> 31) ^ 1}
    else 
    {(y >> 15) ^ 3};
}

//find leftmost byte having less than 9 
pub fn le_9_byte(x: i64 ) -> i64 {
    let mut y = (x & 0x7F7F7F7F) + 0x76767676;
    y = y | x;
    y = y | 0x7F7F7F7F; // > 9 are 0xFF
    y = !y; // 0x00
    //<= 9 are 0x80
    let n = nlz(y as u32) >> 3;
    return n as i64;
}

//left most uppercase letter
pub fn uppercase_byte(x: i64) -> i64 {
    let mut d = ((x as i64 | 0x80808080 ) - 0x41414141) as i64;
    d = !((x | 0x7F7F7F7F) ^ d);
    let mut y = (d & 0x7F7F7F7F) + 0x66666666;
    y =  y | d;
    y = y | 0x7F7F7F7F; //0xFF again
    y = !y; //0x00 again
    let n = nlz(y as u32) >> 3;
    return n as i64;
}

//finds length and position of shortest contiguous string of 1s in word
//if n-consecutive 1-bits are found, returns number 0 to 31, otherwise if
//not found returns 32
pub fn one_bit1(mut x: i32, mut n: & mut i32) -> i32 {
    if x == 0 {*n = 32; return 0; } //initalize to return position
    let b = !(x >> 1) & x;  
    let mut c = x & !(x << 1) ;
    let mut k = 1;
    loop {
        if b & c != 0 {
            break;
        }
        c = c << 1;
        k += 1;
    } 
    *n = nlz((b & c) as u32) as i32;
    return k;
}





