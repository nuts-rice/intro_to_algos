use std::cmp::max;

pub fn rod_cut(profit: &[usize]) -> usize {
    let n = p.len();
    //dynamic table of possible cuttings
    let mut f = vec![0; n];

    for i in 0..n {
        let mut max_price = p[i];

        for j in 1..=i {
            max_price = max(max_price, p[j - 1] + f[i - j])
        }
        f[i] = max_price;
    }

    if n != 0 {
        f[n - 1]
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::rod_cut;
    #[test]
    fn test_rod_cut() {
        assert_eq!(15, rod_cut(&[5, 8, 2]));
    }
}
