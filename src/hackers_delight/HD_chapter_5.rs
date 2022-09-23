#[cfg(target_arch = "riscv64")]
use core::{
    borrow::{Borrow, BorrowMut},
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(target_arch = "x86_64")]
use std::sync::atomic::AtomicUsize;

static SEED: AtomicUsize = AtomicUsize::new(0);

pub fn counts_divide_and_conquer(mut x: i32) -> i32 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    x
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_counts() {
    assert_eq!(counts_divide_and_conquer(1), 1);
    assert_eq!(counts_divide_and_conquer(2), 1);
}

// The first assignment to x is based on the first two terms of the rather surprising
// formula
// equation (1)
pub fn counts_pop(mut x: i64) -> i64 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x >> 8);
    x = x + (x >> 16);
    x & 0x0000003F
}
//method of equation (1) improved with "rotate and sum"
pub fn counts_rotate_1(x: u32, n: u32) -> u32 {
    if n > 63 {}
    (x.wrapping_shl(n)) | (x.wrapping_shr(32 - n))
}

pub fn counts_pop_0(mut x: i64) -> i64 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    x
}

//simplification with one fewer instruction
pub fn count_pop_1(mut x: i64) -> i64 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x >> 8);
    x = x + (x >> 16);
    x & 0x0000003F
}

//variation of HAKEMEM algo,
//counts number of 1s in 4bit nibble, then works on all eight mibbles in parallel
pub fn count_pop_2(mut x: i64) -> i64 {
    let mut n = (x >> 1) & 0x77777777;
    x -= n;
    n = (n >> 1) & 0x77777777;
    x -= n;
    n = (n >> 1) & 0x77777777;
    x -= n;
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x *= 0x01010101;
    x >> 24
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn counts_pop_test1() {
    assert_eq!(counts_pop(1), 1);
    assert_eq!(counts_pop(3), 2);
    assert_eq!(counts_pop_0(1), 1);
    assert_eq!(counts_pop_0(3), 2);
    assert_eq!(counts_divide_and_conquer(1), 1);
    assert_eq!(count_pop_1(1), 1);
    assert_eq!(count_pop_1(3), 2);
    assert_eq!(count_pop_2(1), 1);
    assert_eq!(count_pop_2(3), 2);
}

//clear a single bit in each word until one of the words is all zero
//and the other has a larger population count
////returns negative int if pop(x) < pop(y), 0 if pop(x) = pop(y), 1 if pop(x) > pop(y)
pub fn pop_diff(mut x: i32, mut y: i32) -> i32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    y = !y;
    y = y - ((y >> 1) & 0x55555555);
    y = (y & 0x33333333) + ((y >> 2) & 0x33333333);
    x += y;
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = x + (x >> 8);
    x = x + (x >> 16);
    (x & 0x0000007F) - 32
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_pop_diff() {
    assert_eq!(pop_diff(1, 1), 0);
}

macro_rules! CSA {
    ($h: expr, $l: expr, $a: expr, $b: expr, $c: expr ) => {
        let u = $a ^ $b;
        let v = $c;
        $h = ($a & $b) | (u & v);
        $l = u ^ v;
    };
}

//refer to circut diagram in book
pub fn counts_pop_csa(A: &mut [i32], n: i32) -> i32 {
    let mut total1 = 0;
    let mut total2 = 0;
    let mut i: usize = 0;
    let mut twos = 0;
    let mut ones = 0;
    loop {
        i += 3;
        if i <= (n - 3) as usize {
            break;
        }
        CSA!(twos, ones, A[i], A[i + 1], A[i + 2]);
        total1 += counts_pop(ones as i64);
        total2 += counts_pop(twos as i64);
    }
    for t in i..(n as usize) {
        total1 += counts_pop(A[t] as i64)
    }
    (2 * total2 + total1) as i32
}

//brings in values two at a time, combines with ones and twos
pub fn counts_pop_csa2(A: &mut [i32], n: i32) -> i32 {
    let mut total = 0;
    let _total2 = 0;
    let mut i: usize = 0;
    let mut twos = 0;
    let mut ones = 0;

    loop {
        i += 2;
        if i <= (n - 2) as usize {
            break;
        }
        CSA!(twos, ones, ones, A[i], A[i + 1]);
        total += counts_pop(twos as i64);
    }
    total = 2 * total + counts_pop(ones as i64);

    if n & 1 != 0 {
        total += counts_pop(A[i] as i64);
    }

    total as i32
}
