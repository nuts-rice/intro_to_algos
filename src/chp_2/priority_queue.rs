use std::cmp::Ordering;

//There are max-priority and min-priority, just like heap-properties
//Mantains set S of elements, each with associated value called Q 


type Comparator<T> = Box<dyn Fn(&T, &T) -> bool>;

pub struct priority_queue<T>{
    pq: Vec<T>,
    n: usize,
    comparator: Comparator<T>,
}

pub struct index_pq<T>{
    n: usize, //S of elements
    pq: Vec<i32>,
    qp: Vec<i32>,
    keys: Vec<Option<T>>,
    comparator: Comparator<Option<T>>,
}

impl<T: PartialOrd + Default> priority_queue<T>{
    //peeks at minimum key
    pub fn new_min_priority_queue(cap: usize) -> Self {
        let comparator = Box::new(|a: &T, b: &T| a.gt(b));
        Self::new(cap, comparator)
    }

    fn new_max_priority_queue(cap: usize)-> Self{
        let comparator = Box::new(|a: &T, b: &T| a.lt(b));
        Self::new(cap, comparator)
    }
}
