pub fn rand_search(_k: usize, _t: u32) -> u32 {
    unimplemented!();
}

pub fn step_by_search(max: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    if max >= 2 {
        result.push(2)
    }

    //only odds
    for i in (3..max + 1).step_by(2) {
        let stop: usize = (i as f64).sqrt() as usize + 1;
        let mut is_prime: bool = true;

        for j in (3..stop).step_by(2) {
            //binary search for conditional
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            result.push(i)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_by_search_test() {
        assert_eq!(step_by_search(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(step_by_search(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    }
}
