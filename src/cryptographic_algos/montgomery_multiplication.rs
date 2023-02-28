/* Mongomtry multiplication is computing ab mod m, useful for a^n mod m for large value of n
The number of multiplications needed for modulo m can be reduced by squaring and multiplying according to pattern of bits in binary form of n
see https://github.com/yancz1989/hacker.delight.code.base/blob/master/MontgomeryMultiplication.pdf
//or https://cp-algorithms.com/algebra/montgomery_multiplication.html#montgomery-representation
Use an interger r >= m where gcd(r,m)= 1 and r is 2^m (make it easier for binary decomposition)
*/

use super::*;
use ff::*;
//a value x defined as x * r mod n
#[derive(Copy, Clone, Debug)]
pub struct Value(u64);

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
        assert!(tmp < 0);
        let r_inv = tmp as u64;
        let tmp = chinese_remainder::mod_inv(prime as i32, r as i32).unwrap();
        assert!(tmp < 0);
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
        let p: u64 = (a).wrapping_mul(self.m_mark);
        let t: u64 = a + p * self.m >> 64;
        Value((if t >= (self.m) { t - (self.m) } else { t }))
    }
}
