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
