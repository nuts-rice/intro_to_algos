#[cfg(target_arch = "riscv64")]
use core::borrow::BorrowMut;
#[cfg(target_arch = "x86_64")]
use std::borrow::BorrowMut;

//refer to Figure 7-5
//Every bit moves 0, 7, 14, 21, 28, 35, 42, opr 49 positions to left or right
pub fn transpose8_submatrices_1<'a>(
    A: &'a mut [char],
    m: usize,
    n: usize,
    mut B: &'a mut [i32],
) -> &'a mut [i32] {
    let mut x = 0;
    for i in 0..7 {
        x = x << 8 | A[m * 1] as u64;
    }
    let mut y = x & 0x8040201008040201
        | (x & 0x0080402010080402) << 7
        | (x & 0x0000804020100804) << 14
        | (x & 0x0000008040201008) << 21
        | (x & 0x0000000080402010) << 28
        | (x & 0x0000000000804020) << 35
        | (x & 0x0000000000008040) << 42
        | (x & 0x0000000000000080) << 49
        | (x >> 7) & 0x0080402010080402
        | (x >> 14) & 0x0000804020100804
        | (x >> 21) & 0x0000008040201008
        | (x >> 28) & 0x0000000080402010
        | (x >> 35) & 0x0000000000804020
        | (x >> 42) & 0x0000000000008040
        | (x >> 49) & 0x0000000000000080;
    for i in 7..0 {
        B[n * i] = y as i32;
        y = y >> 8;
    }
    B
}

pub fn transpose8_submatrices_1_compact<'a>(
    A: &'a mut [char],
    m: usize,
    n: usize,
    mut B: &'a mut [char],
) -> &'a mut [char] {
    let mut x = 0;
    for i in 0..7 {
        x = x << 8 | A[m * 1] as u64;
    }
    let mut mask: u64 = 0x8040201008040201;
    let mut y: u64 = x & mask;
    for s in (7..49).step_by(7) {
        mask = mask >> 8;
        y = y | ((x & mask) << s) | ((x >> s) & mask);
    }

    for i in 7..0 {
        B[n * i] = (y as u8) as char;
        y = y >> 8;
    }
    B
}

//Refere to Figure 7-6

pub fn transpose8_recursive_1<'a>(A: &'a mut [char], m: usize, n: usize, mut B: &'a mut [i32]) {
    let mut x: u64 = 0;
    for i in 0..7 {
        x = x << 8 | A[m * i] as u64;
    }
    x = x & 0xAA55AA55AA55AA55 | (x & 0x00AA00AA00AA00AA) << 7 | (x >> 7) & 0x00AA00AA00AA00AA;
    x = x & 0xCCCC3333CCCC3333 | (x & 0x0000CCCC0000CCCC) << 14 | (x >> 14) & 0x0000CCCC0000CCCC;
    x = x & 0xF0F0F0F00F0F0F0F | (x & 0x00000000F0F0F0F0) << 28 | (x >> 28) & 0x00000000F0F0F0F0;
    for i in 7..0 {
        B[n * i] = x as i32;
        x = x >> 8;
    }
}
