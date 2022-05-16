fn main() {
    let mut a: [i8; 6] =[43, 55, 11, 33, 42, 52];
    println!("{:?}", &a[0..6]);

    for j in (0..a.len() -1 ).rev(){
        let key = a[j];
        let mut i = j;
        while i < a.len() - 1 && a[i+1]> key
        {
            a[i] = a[i + 1];
            i = i + 1;
        }
        a[i] = key;
        println!("{:?}", &a[0..6]);
    }
    println!("{:?}", &a[0..6]);
}
