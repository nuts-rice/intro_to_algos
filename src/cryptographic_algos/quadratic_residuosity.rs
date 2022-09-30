use std::rc::Rc;

pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
    assert!(base >= 1);

    let mut result = 1;
    while power > 0 {
        if power & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        power >>= 1;
    }
    result
}

#[derive(Debug)]
struct CustomFiniteField {
    modulus: u32,
    i_square: u32,
}

impl CustomFiniteField {
    pub fn new(modulus: u32, i_square: u32) -> Self {
        Self { modulus, i_square }
    }
}

#[derive(Clone, Debug)]
struct CustomComplexNumber {
    real: u32,
    imag: u32,
    f: Rc<CustomFiniteField>,
}

impl CustomComplexNumber {
    pub fn new(real: u32, imag: u32, f: Rc<CustomFiniteField>) -> Self {
        Self { real, imag, f }
    }

    pub fn mult_other(&mut self, rhs: &Self) {
        let tmp = (self.imag * rhs.real + self.real * rhs.imag) % self.f.modulus;
        self.real = (self.real * rhs.real
            + ((self.imag * rhs.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp;
    }

    pub fn mult_self(&mut self) {
        let tmp = (self.imag * self.real + self.real * self.imag) % self.f.modulus;
        self.real = (self.real * self.real
            + ((self.imag * self.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp;
    }

    pub fn fast_power(mut base: Self, mut power: u32) -> Self {
        let mut result = CustomComplexNumber::new(1, 0, base.f.clone());
        while power != 0 {
            if (power & 1) != 0 {
                result.mult_other(&base); // result *= base;
            }
            base.mult_self(); // base *= base;
            power >>= 1;
        }
        result
    }
}

fn is_residue(x: u32, modulus: u32) -> bool {
    let power = (modulus - 1) >> 1;
    x != 0 && fast_power(x as usize, power as usize, modulus as usize) == 1
}

pub fn cipolla(a: u32, p: u32) -> Option<(u32, u32)> {
    let a = a as u32;
    let p = p as u32;
    if a == 0 {
        return Some((0, 0));
    }
    if !is_residue(a, p) {
        return None;
    }
    let rand_1: u32 = rand::random();
    let r = loop {
        let r = rand_1 % p;
        if r == 0 || !is_residue((p + r * r - a) % p, p) {
            break r;
        }
    };
    let field = Rc::new(CustomFiniteField::new(p, (p + r * r - a) % p));
    let comp = CustomComplexNumber::new(r, 1, field);
    let power = (p + 1) >> 1;
    let x0 = CustomComplexNumber::fast_power(comp, power).real as u32;
    let x1 = p as u32 - x0 as u32;
    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_residuity_test() {
        assert_eq!(cipolla(2, 23), Some((5, 18)));
        assert_eq!(cipolla(17, 83), Some((10, 73)));
    }
}
