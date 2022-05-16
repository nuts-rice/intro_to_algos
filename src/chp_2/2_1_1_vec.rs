fn insert_sort(v : &mut Vec<i32>){
    for j in 1..v.len(){
        let key = v[j];
        let mut i = j;

        while i>0 && v[i-1]>key
        {
            v[i] = v[i - 1];
            i = i-1;
        }
        v[i] = key;
    }
}

fn main(){
    let mut v: Vec<i32> = vec![5, 2, 8, 1];
    println!("v={:?}", &v);

    insert_sort(&mut v);

    println!("sorted v={:?}", &v);
}
