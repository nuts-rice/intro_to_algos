use crate::*;

//2-1 manipulating rightmost bits

//use formula: 'x&(x -1)' to turn off the rightmost 1 bit in a word, producing 0 if none
// this can be used to determine if an unsigned integer is power of 2 or is 0
pub fn formula_get_and(x: u8) -> u8 {
    x & (x - 1)
}

//use formula 'x | (x+1)' to turn on the rightmost 0 bit in word, producing 1s if none
pub fn formula_get_or(x: u8) -> u8 {
    x | (x + 1)
}

//formula: 'x&(x+1)' to turn off trailing 1s in word, producing x if none
// can be used to determin if unsigned integer is the form 2^n -1, 0, or all 1s
pub fn formula_trailing_1_to_0(x: u8) -> u8 {
    x & (x + 1)
}

//formula: 'x | (x -1)' to turn on all the trailing 0s in word, producing x if none
pub fn formula_trailing_0_to_1(x: u8) -> u8 {
    x | (x - 1)
}

//formula '!x & (x + 1)' to create a word with single 1 bit at position of the rightmost 0-bit
// producing 0 if none
pub fn formula_rightmost_0_to_1(x: u8) -> u8 {
    (!x) & (x + 1)
}

//formula '!x | (x - 1)' to create word with single 0 at position of rightmost 1 bit
// producing all 1s if none
pub fn formula_rightmost_1_to_0(x: u8) -> u8 {
    (!x) | (x - 1)
}

//turn off rightmost continous string of 1s
pub fn formula_turnoff_rightmost_1s(x: u8) -> u8 {
    ((x | (x - 1))) + (1) & 1
}

//Demorgan's Laws
// We can think of demorgan's as distrubuting the 'not' sign
pub fn demorgan_1(x: u8, y: u8) -> bool {
    !(x & y) == !x | !y
}

pub fn demorgan_2(x: u8, y: u8) -> bool {
    !(x | y) == !x | !y
}

//number of trailing zeros, number of leading zeros, population count
pub fn ntz(mut x: u32) -> u32 {
    if x == 0 {return 32; }
    let mut n = 1;
    if (x & 0x0000FFFF) == 0 {
        n = n + 16;
        x = x >> 16;
    }
    if (x & 0x000000FF) == 0 {
        n = n + 8;
        x = x >> 8;
    }
    if (x & 0x0000000F) == 0 {
        n = n + 4;
        x = x >> 4;
    }
    if (x & 0x00000003) == 0 {
        n = n + 2;
        x = x >> 2;
    }
    n - (x & 1)
}


pub fn nlz(mut x: u32) -> u32 {
    if x == 0 {return 32;}
    let mut n = 0;
    if x <= 0x0000FFFF {
        n = n + 16;
        x = x << 16;
    }
    if x <= 0x00FFFFFF {
        n = n + 8;
        x = x << 8;
    }
    if x <= 0x0FFFFFFFF {
        n = n + 4;
        x = x << 4;
    }
    if x <= 0x3FFFFFFF {
        n = n + 2;
        x = x << 2;
    }
    if x <= 0x7FFFFFF {n = n + 1;}
    return n;
}

pub fn population_cnt(mut x: u32) -> u32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x << 8);
    x = x + (x << 16);
    x >> 24
}

pub fn ntz1(mut x: u32) -> u32 {
    return 32 - nlz(!x & (x - 1));
}

pub fn ntz2(mut x: u32) -> u32 {
    return population_cnt(!x & (x - 1));
}


//Snoob: bit shifting for continous 1s or 0s
// given that theres subsets of 1s or 0s, we can find rightmost continous group of 1s in x and following 0s
// then we can "increment" that quantity to the next value that has the same number of 1s
pub fn snoob0(mut x: i32) -> i32 {
    let mut smallest = 0;
    let mut ripple  = 0;
    let mut ones = 0;
    smallest = x & -x;
    ripple = x + smallest;
    ones = x ^ ripple;
    ones = (ones >> 2) / smallest;
    return ripple | ones;
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_2_1(){
    assert_eq!(formula_get_and(0b01011000), (0b001010000));
    assert_eq!(formula_get_or(0b10100111), (0b10101111));
    assert_eq!(formula_trailing_1_to_0(0b10100111), (0b10100000));
    assert_eq!(formula_trailing_0_to_1(0b10101000), (0b10101111));
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_basics2() {
    assert_eq!(ntz(12), 2);
    assert_eq!(ntz1(12), 2);
    assert_eq!(ntz2(12), 2);
    assert_eq!(ntz(123), 0);
    assert_eq!(ntz(21), 0);
}
