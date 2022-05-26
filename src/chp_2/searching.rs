fn search(v: i8) -> usize {
    let a: [i8; 6] = [33, 66, 33, 86, 24, 64];
    println!("{:?}", &a[0..6]);

    for i in 0..a.len() {
        if a[i] == v {
            return i + 1;
        }
    }
    return 0;
}

fn main() {
    let v = 86;
    println!("{}", v, search(v));
}
