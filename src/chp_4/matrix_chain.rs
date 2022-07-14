use std::collections::HashMap;

fn mcm(dims: Vec<i32>, costs: &mut HashMap<Vec<i32>, (i32, Vec<usize>)>) -> (i32, Vec<usize>) {
    match costs.get(&dims) {
        Some(c) => c.clone(),
        None => {
            let ans = if dims.len() == 3 {
                (dims[0] * dims[1] * dims[2], vec![0])
            } else {
                let mut min_cost = std::i32::MAX;
                let mut min_path = Vec::new();
                for i in 1..dims.len() - 1 {
                    let taken = dims[(i - 1)..(i + 2)].to_vec();
                    let mut rest = dims[..i].to_vec();
                    rest.extend_from_slice(&dims[(i + 1)..]);
                    let a1 = mcm(taken, costs);
                    let a2 = mcm(rest, costs);
                    if a1.0 + a2.0 < min_cost {
                        min_cost = a1.0 + a2.0;
                        min_path = vec![i - 1];
                        min_path.extend_from_slice(&a2.1);
                    }
                }
                (min_cost, min_path)
            };
            costs.insert(dims, ans.clone());
            ans
        }
    }
}
