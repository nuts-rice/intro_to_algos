pub fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();
    let (na, nb) = (a.len(), b.len());

    //solns[i][j] is the length of the longest common subsequence
    //between a[0..i-1] and b[0..j-1]
    let mut solns = vec![vec![0; nb + 1]; na + 1];

    for (i, ci) in a.iter().enumerate() {
        for (j, cj) in b.iter().enumerate() {
            //if ci == cj, there is a common character;
            //otherwise , take the best of the two solns
            //at (i-1, j) and (i, j-1)
            solns[i + 1][j + 1] = if ci == cj {
                solns[i][j] + 1
            } else {
                solns[i][j + 1].max(solns[i + 1][j])
            }
        }
    }

    let mut result: Vec<char> = Vec::new();
    let (mut i, mut j) = (na, nb);

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if solns[i - 1][j] > solns[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(&longest_common_subsequence("", ""), "");
        assert_eq!(&longest_common_subsequence("", "abcd"), "");
        assert_eq!(&longest_common_subsequence("abcd", "c"), "c");
    }
}
