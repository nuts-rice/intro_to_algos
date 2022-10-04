use std::collections::HashMap;

pub fn baby_step_giant_step(a: usize, b: usize, n: usize) -> Option<usize> {
    if b == 1 {
        return Some(n);
    }
    let mut h_map = HashMap::new();
    let m = (n as f64).sqrt().ceil() as usize;
    //baby step
    let mut step = 1;
    for i in 0..m {
        h_map.insert((step * b) % n, i);
        step = (step * a) % n;
    }
    //giant step  a^m (mod n)
    let giant_step = step;
    for i in (m..=n).step_by(m) {
        if let Some(v) = h_map.get(&step) {
            return Some(i - v);
        }
        step = (step * giant_step) % n;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baby_step_giant_step_test() {
        assert_eq!(baby_step_giant_step(3, 57, 113), Some(100));
    }
}
