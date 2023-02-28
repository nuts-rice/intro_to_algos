fn advance_euclid(a: &mut i32, old_a: &mut i32, quotiant: i32) {
    let temp = *a;
    *a = *old_a - quotiant * temp;
    *old_a = temp;
}

pub fn extended_euclid(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotiant = old_r / rem;
        advance_euclid(&mut rem, &mut old_r, quotiant);
        advance_euclid(&mut coeff_s, &mut old_s, quotiant);
        advance_euclid(&mut coeff_t, &mut old_t, quotiant);
    }

    (old_r, old_s, old_t)
}

pub fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = extended_euclid(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn mod_pow(mut x: i64, mut e: u32, prime: i64) -> i64 {
    let mut tmp = 1;
    while e > 0 {
        if e % 2 == 0 {
        } else {
            tmp = (tmp * x) % prime;
        }
        x = (x * x) % prime;
        e = e >> 1;
    }
    tmp
}

pub fn chinese_remainder(residues: &[i32], modulli: &[i32]) -> Option<i32> {
    let prod = modulli.iter().product::<i32>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chinese_remainder_tests() {
        assert_eq!(chinese_remainder(&[3, 5, 7], &[2, 3, 1]), Some(5));
        assert_eq!(chinese_remainder(&[1, 4, 6], &[3, 5, 7]), Some(34));
    }

    #[test]
    fn eea_tabular_test() {
        assert_eq!(mod_inv(397, 2357), Some(1603));
    }
}
