/* Mongomtry multiplication is computing ab mod m, useful for a^n mod m for large value of n
The number of multiplications needed for modulo m can be reduced by squaring and multiplying according to pattern of bits in binary form of n
see https://github.com/yancz1989/hacker.delight.code.base/blob/master/MontgomeryMultiplication.pdf
//or https://cp-algorithms.com/algebra/montgomery_multiplication.html#montgomery-representation
Use an interger r >= m where gcd(r,m)= 1 and r is 2^m (make it easier for binary decomposition)
*/

use super::*;
use ff::*;
//a value x defined as x * r mod n
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Value<'a>(pub &'a MongtgomerySpace);

#[derive(Copy, Clone, Debug)]
pub struct MongtgomerySpace {
    pub m: u64,
    pub m_mark: u64, //r * r^-1 + n * n' = 1 (computed by EEA)
    pub r_inv: u64,
    pub r_cube: u64,
}

impl MongtgomerySpace {
    pub fn new(prime: u64) -> MongtgomerySpace {
        //2^64, powers of two for binary decomposition
        let r = 1u64 << 64;
        let tmp = chinese_remainder::mod_inv(r as i32, prime as i32).unwrap();
        let r_inv = tmp as u64;
        let tmp = chinese_remainder::mod_inv(prime as i32, r as i32).unwrap();
        let m_mark = (r as i32 - tmp) as u64;
        let r_cube = chinese_remainder::mod_pow(r as i64 % prime as i64, 3u32, prime as i64) as u64;
        MongtgomerySpace {
            m: prime,
            m_mark: m_mark,
            r_inv: r_inv,
            r_cube: r_cube,
        }
    }
    //reduce number of multiplications needed to find x^m
    fn redc(&self, a: u64) -> Value {
        unsafe {
            let p: u64 = (a).wrapping_mul(self.m_mark);
            let t: u64 = (a as u128 + (p * self.m) as u128 >> 64) as u64;
            todo!()
            //    Value((if t >= (self.m) { t - (self.m) } else { t }))
        }
    }
}
//TODO: figure this out lol

// impl Mul for Value<'MongtgomerySpace> {
//     type Output<'a> = Value<'a>;

//     fn mul(self, b: Self::Output) -> Self::Output {
//         Value(self.0.redc(self.0.r_cube).0.wrapping_mul(b.0))
//     }
// }

// impl Add for Value<'MongtgomerySpace> {
//     type Output<'a> = Value<'a>;

//     fn add(self, b: Self::Output) -> Self::Output {
//         let sum = self.0.r_cube + b.0;
//         if sum > self.0.m {
//             Value((sum - self.0.m))
//         } else {
//             Value(sum)
//         }
//     }
// }

// impl Sub for Value<'MongtgomerySpace> {
//     type Output = Value;

//     fn sub(self, b: Self::Output) -> Self::Output {
//         todo!()
//     }
// }

//fn modulus(&self) -> u64 {
//        self.n as u64
//    }

/*
impl Mul<Value> for Value {
    type Output = Value;
        fn mul(&self, x: u64, y: u64) -> Value {
            (MongtgomerySpace::redc(x) * y)
        }
}
*/
