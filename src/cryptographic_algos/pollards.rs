struct LinearCongruenceGenerator {
    //modulus as 2 ^ 32
    multiplier: u32,
    increment: u32,
    state: u32,
}
impl LinearCongruenceGenerator {
    fn new(multiplier: u32, increment: u32, state: u32) -> Self {
        Self {
            multiplier,
            increment,
            state,
        }
    }
    fn next(&mut self) -> u32 {
        self.state = (self.multiplier as u64 * self.state as u64 + self.increment as u64) as u32;
        self.state
    }
    fn get_64bits(&mut self) -> u64 {
        ((self.next() as u64) << 32) | (self.next() as u64)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        let tmp = b % a;
        b = a;
        a = tmp;
    }
    b
}

#[inline]
fn advance(x: u128, c: u64, number: u64) -> u128 {
    ((x * x) + c as u128) % number as u128
}

fn modulo_power(mut base: u64, mut power: u64, modulo: u64) -> u64 {
    base %= modulo;
    if base == 0 {
        return 0;
    }
    let mut ans: u128 = 1;
    let mut bbase: u128 = base as u128;
    while power > 0 {
        if (power % 2) == 1 {
            ans = (ans * bbase) % (modulo as u128);
        }
        bbase = (bbase * bbase) % (modulo as u128);
        power /= 2;
    }
    ans as u64
}

fn is_prime_base(number: u64, base: u64, two_power: u64, odd_power: u64) -> bool {
    let mut x: u128 = modulo_power(base, odd_power, number) as u128;
    let bnumber: u128 = number as u128;
    if x == 1 || x == (bnumber - 1) {
        return true;
    }
    for _ in 1..two_power {
        x = (x * x) % bnumber;
        if x == (bnumber - 1) {
            return true;
        }
    }
    false
}

//returns 0 on probable prime otherwise witness
pub fn miller_rabbin(number: u64, bases: &[u64]) -> u64 {
    if number <= 4 {
        match number {
            0 => {
                panic!("0 isnt prime and has no witnesses")
            }
            2 => return 0,
            2 => return 0,
            _ => return number,
        }
    }
    if bases.contains(&number) {
        return 0;
    }
    let two_power: u64 = (number - 1).trailing_zeroes() as u64;
    let odd_power = (number - 1) >> two_power;
    for base in bases {
        if !is_prime_base(number, *base, two_power, odd_power) {
            return *base;
        }
    }
    0
}
