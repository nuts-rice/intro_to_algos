use core::fmt::Debug;
//Merge two ordered lists, ordering result is copied to worker list l2
fn merge(l1: &Vec<i32>, s: usize, m: usize, e: usize, l2: &mut Vec<i32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;

    for i in s..e {
        if (ptr1 < m) && (ptr2 >= e || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}
//Copies from l2 to primary list l1 using mapping inside closure
fn merge_copy(l1: &mut Vec<i32>, s: usize, e: usize, l2: &Vec<i32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}

//Splits mutable list into two sub-lists, done recusively until only n sub-lists remain where n=number of elements in original list
fn merge_split(l1: &mut Vec<i32>, s: usize, e: usize, l2: &mut Vec<i32>) {
    if e - s > 1 {
        let m: usize = (e + s) / 2;
        merge_split(l1, s, m, l2);
        merge_split(l1, m, e, l2);
        merge(l1, s, m, e, l2);
        merge_copy(l1, s, e, l2);
    }
}

pub fn mergesort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    let mut worker: Vec<i32> = vec![0; size];
    merge_split(list, 0, size, &mut worker);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = vec![1, 2, 5, 8, 3, 9];
        mergesort(&mut list);
        assert_eq!(list, vec![1, 2, 3, 5, 8, 9]);
    }
}
