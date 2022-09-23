pub fn is_sqroot1(x: i64) -> i64 {
    if x <= 1 {
        x;
    }
    let mut s = 1;
    let mut x1 = x - 1;
    if x > 65535 {
        s += 8;
        x1 >>= 16;
    }
    if x1 > 255 {
        s += 4;
        x1 >>= 8;
    }
    if x1 > 15 {
        s += 2;
        x1 >>= 4;
    }
    if x1 > 3 {
        s += 1;
    }

    let mut g0 = 1 << s; // g0 = 2 **s
    let mut g1 = (g0 + (x >> s)) >> 1; //g1 = (g0 + x/g0)/2
    while g1 < g0 {
        //Guesses strictly decrease
        g0 = g1;
        g1 = (g0 + (x / g0)) >> 1;
    }
    g0
}

//Binary search for first guess
pub fn is_sqroot2(x: i64) -> i64 {
    let mut s = 0;
    if x <= 4224 {
        if x <= 24 {
            if x <= 3 {
                return (x + 3) >> 2;
            } else if x <= 8 {
                return 2;
            } else {
                return (x >> 4) + 3;
            }
        } else if x <= 288 {
            if x <= 80 {
                s = 3;
            } else {
                s = 4;
            }
        } else if x <= 1088 {
            s = 5;
        } else {
            s = 6;
        }
    } else if x < 1025 * 1025 {
        if x < 257 * 257 {
            if x < 129 * 129 {
                s = 7;
            } else {
                s = 8;
            }
        } else if x < 513 * 513 {
            s = 9;
        } else {
            s = 10;
        }
    } else if x < 4097 * 4097 {
        if x < 2049 * 2049 {
            s = 11;
        } else {
            s = 12;
        }
    } else if x < 16385 * 16385 {
        if x < 8193 * 8193 {
            s = 13;
        } else {
            s = 14;
        }
    } else if x < 32769 * 32769 {
        s = 15;
    } else {
        s = 16;
    }
    let mut g0 = 1 << s;
    //Compute with guesses as b4, upper and lower bounds
    let mut g1 = (g0 + (x >> s)) >> 1;
    while g1 < g0 {
        g0 = g1;
        g1 = (g0 + (x / g0)) >> 1;
    }
    g0
}

#[test]
fn is_sqroot_test() {
    assert_eq!(is_sqroot1(1), 1);
}
