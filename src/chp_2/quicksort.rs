use std::cmp::PartialOrd;

pub fn partition<T: PartialOrd>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

fn _quicksort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quicksort(arr, low, p - 1);
        _quicksort(arr, p + 1, high);
    }
}

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quicksort(arr, 0(len - 1) as isize);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn quicksort_test() {
        let mut list = [1, 5, 3, 8, 4, 9];
        quicksort(&mut list);
        assert_eq!(list, [1, 3, 4, 5, 8, 9]);
    }
}
