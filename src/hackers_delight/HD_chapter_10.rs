//relayed concepts: Modulus and floor division, inverse operations (negate)
//Look into: shift right sighned instruction, Barret reduction

#[derive(PartialEq, Debug)]
pub struct ms {
    magic: i64,
    shift: i64,
}

//algorithm found in 10-7
pub fn divide_magic(d: i64) -> ms {
    let powtwo31 = 0x80000000; //2^31
    let mut mag = ms { magic: 0, shift: 0 };
    let ad = d.abs();
    let tmp = powtwo31 + (d >> 31);
    let anc = tmp - 1 - tmp % ad;
    let mut p = 31;
    let mut quatiant1 = powtwo31 / anc;
    let mut rem1 = powtwo31 - quatiant1 * anc;
    let mut quatiant2 = powtwo31 / ad;
    let mut rem2 = powtwo31 - quatiant2 * ad;
    let mut delta = 0;
    while quatiant1 < delta || (quatiant1 == delta && rem1 == 0) {
        p = p + 1;
        quatiant1 = 2 * quatiant1; //advance quationt = 2 ^p/|nc|
        rem1 = 2 * rem1; //advance remainder = rem(2^p, |nc|)
        if rem1 >= anc {
            quatiant1 = quatiant1 + 1;
            rem1 = rem1 - anc;
        }
        quatiant2 = 2 * quatiant2;
        rem2 = 2 * rem2;
        if rem2 >= ad {
            quatiant2 = quatiant2 + 1;
            rem2 = rem2 - ad;
        }
        delta = ad - rem2;
    }
    mag.magic = quatiant2 + 1;
    if d < 0 {
        mag.magic = -mag.magic
    }
    mag.shift = p - 32;
    return mag;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hamming_test() {
        let right = ms {
            magic: 3074457345618258603,
            shift: 0,
        };
        assert_eq!(divide_magic(6), right);
    }
}
