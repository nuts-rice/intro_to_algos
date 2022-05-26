fn main() {
    let mut a: [i8; 6] = [3, 7, 2, 8, 5, 1];
    println!("{:?}", &a[0..6]);

    for j in 1..a.len() {
        let key = a[j];
        let mut i = j;
        //Initialization invariant is true while a[1]
        while i > 0 && a[i - 1] > key {
            a[i] = a[i - 1];
            i = i - 1;
        }
        a[i] = key;
    }
    println!("{:?}", &a[0..6]);
}
