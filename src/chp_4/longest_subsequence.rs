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

pub fn longest_increasing_subsequence<T: Ord + Clone>(input_array: &[T]) -> Vec<T> {
    let n = input_array.len();
    if n <= 1 {
        return input_array.to_vec();
    }

    let mut increasing_sequence: Vec<(T, usize)> = Vec::new();
    let mut previous = vec![0_usize; n];

    increasing_sequence.push((input_array[0].clone(), 1));
    for i in 1..n {
        let value = input_array[i].clone();
        if value > increasing_sequence.last().unwrap().0 {
            previous[i] = increasing_sequence.last().unwrap().1 - 1;
            increasing_sequence.push((value, i + 1));
            continue;
        }

        let change_position = increasing_sequence
            .binary_search(&(value.clone(), 0))
            .unwrap_or_else(|x| x);
        increasing_sequence[change_position] = (value, i + 1);
        previous[i] = match change_position {
            0 => i,
            other => increasing_sequence[other - 1].1 - 1,
        };
    }

    let mut out: Vec<T> = Vec::with_capacity(increasing_sequence.len());

    out.push(increasing_sequence.last().unwrap().0.clone());
    let mut current_index = increasing_sequence.last().unwrap().1 - 1;
    while previous[current_index] != current_index {
        current_index = previous[current_index];
        out.push(input_array[current_index].clone());
    }

    out.into_iter().rev().collect()
}

enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

enum OperationEvaluator {
    Operator(Operator),
    Operand(i32),
}

fn reverse_polish_notation_aux(expr: &str) -> Result<Vec<OperationEvaluator>, String> {
    expr.split_whitespace()
        .map(|eval| match eval {
            "+" => Ok(OperationEvaluator::Operator(Operator::Addition)),
            "-" => Ok(OperationEvaluator::Operator(Operator::Substraction)),
            "*" => Ok(OperationEvaluator::Operator(Operator::Multiplication)),
            "/" => Ok(OperationEvaluator::Operator(Operator::Division)),
            operand => match operand.parse::<i32>() {
                Ok(val) => Ok(OperationEvaluator::Operand(val)),
                Err(_) => Err(format!("Cannot parse operand \"{}\"", operand)),
            },
        })
        .into_iter()
        .collect()
}

pub fn reverse_polish_notation(expr: &str) -> Result<i32, String> {
    return match reverse_polish_notation_aux(expr) {
        Ok(tokens) => {
            let mut stack: Vec<i32> = Vec::new();
            for token in tokens {
                match token {
                    OperationEvaluator::Operator(operator) => {
                        if stack.len() < 2 {
                            return Err("not enough operands before operator".to_string());
                        }
                        let operand2 = stack.pop().expect("expected integer in stack");
                        let operand1 = stack.pop().expect("expected integer in stack");
                        let result = match operator {
                            Operator::Addition => operand1 + operand2,
                            Operator::Substraction => operand1 - operand2,
                            Operator::Multiplication => operand1 * operand2,
                            Operator::Division => operand1 / operand2,
                        };
                        stack.push(result);
                    }
                    OperationEvaluator::Operand(val) => stack.push(val),
                }
            }
            if stack.len() != 1 {
                return Err("Missing operator".to_string());
            }
            return Ok(stack.pop().expect("expected integer remaining in stack"));
        }
        Err(err) => Err(err),
    };
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;
    use super::longest_increasing_subsequence;
    #[test]
    fn longest_common_subsequence_test() {
        assert_eq!(&longest_common_subsequence("", ""), "");
        assert_eq!(&longest_common_subsequence("", "abcd"), "");
        assert_eq!(&longest_common_subsequence("abcd", "c"), "c");
    }

    #[test]
    fn longest_increasing_subsequence_test_1() {
        assert_eq!(
            longest_increasing_subsequence(&vec![10, 9, 2, 5, 3, 7, 101, 18]),
            vec![2, 3, 7, 18]
        );
    }
}
