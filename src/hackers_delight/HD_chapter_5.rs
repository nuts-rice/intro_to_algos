use c8;
use rand::Rng;
use rand::SeedableRng;
use rand_core::RngCore;
use rand_isaac::IsaacRng;
#[cfg(target_arch = "x86_64")]
use std::{borrow::{Borrow,BorrowMut}, sync::atomic::{Ordering,AtomicUsize}};
#[cfg(target_arch = "riscv64")]
use core::{borrow::{Borrow,BorrowMut}, sync::atomic::{Ordering,AtomicUsize}};

static SEED: AtomicUsize = AtomicUsize::new(0);

pub fn counts_divide_and_conquer(mut x: i32) -> i32 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    x
}


#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
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
    return x & 0x0000003F;
}
//method of equation (1) improved with "rotate and sum"
pub fn counts_rotate_1(mut x: u32, n: u32) -> u32 {
    if n > 63 {
        ()
    }
    return (x.wrapping_shl(n)) | (x.wrapping_shr(32 - n));
}

pub fn counts_pop_0(mut x: i64) -> i64 {
    x = (x & 0x55555555) + ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = (x & 0x00FF00FF) + ((x >> 8) & 0x00FF00FF);
    x = (x & 0x0000FFFF) + ((x >> 16) & 0x0000FFFF);
    return x;
}

//simplification with one fewer instruction 
pub fn count_pop_1(mut x: i64) -> i64 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0F0F0F0F;
    x = x + (x >> 8);
    x = x + (x >> 16);
    return x & 0x0000003F;
}

//variation of HAKEMEM algo,
//counts number of 1s in 4bit nibble, then works on all eight mibbles in parallel
pub fn count_pop_2(mut x: i64) -> i64 {
    let mut n = (x >> 1) & 0x77777777;
    x = x - n;
    n = (n >> 1) & 0x77777777;
    x = x - n;
    n = (n >> 1) & 0x77777777;
    x = x - n;
    x = (x + (x >> 4)) & 0x0F0F0F0F; 
    x = x * 0x01010101;
    return x >> 24;
}



#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_counts_pop(){
    assert_eq!(counts_pop(1), 1);
    assert_eq!(counts_pop(3), 2);
    assert_eq!(counts_pop_0(1), 1);
    assert_eq!(counts_pop_0(3), 2);
    assert_eq!(counts_divide_and_conquer(1), 1);
    assert_eq!(counts_pop_1(1), 1);
    assert_eq!(counts_pop_1(3), 2);
    assert_eq!(counts_pop_2(1), 1);
    assert_eq!(counts_pop_2(3), 2);
}

//clear a single bit in each word until one of the words is all zero  
//and the other has a larger population count
////returns negative int if pop(x) < pop(y), 0 if pop(x) = pop(y), 1 if pop(x) > pop(y)
pub fn pop_diff(mut x: i32, mut y: i32) -> i32 {
    x =  x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    y = !y;
    y = y - ((y >> 1) & 0x55555555);
    y = (y & 0x33333333) + ((y >> 2) & 0x33333333);
    x = x + y;
    x = (x & 0x0F0F0F0F) + ((x >> 4) & 0x0F0F0F0F);
    x = x + (x >> 8);
    x = x + (x >> 16);
    return (x & 0x0000007F) - 32;
}

#[cfg_attr(not(target_arch = "x86_64"),test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_pop_diff() {
    assert_eq!(pop_diff(1, 1), 0);
}
