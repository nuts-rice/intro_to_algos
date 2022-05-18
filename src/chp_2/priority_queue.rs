use std::cmp::Ordering;

//There are max-priority and min-priority, just like heap-properties
//Mantains set S of elements, each with associated value called Q 


type Comparator<T> = Box<dyn Fn(&T, &T) -> bool>;

pub struct Priority_Queue<T>{
    pq: Vec<T>,
    n: usize,
    comparator: Comparator<T>,
}

pub struct Index_PQ<T>{
    n: usize, //S of elements
    pq: Vec<i32>,
    qp: Vec<i32>,
    keys: Vec<Option<T>>,
    comparator: Comparator<Option<T>>,
}

impl<T: PartialOrd + Default> Priority_Queue<T>{
    //peeks at minimum key
    pub fn new_min_priority_queue(cap: usize) -> Self {
        let comparator = Box::new(|a: &T, b: &T| a.gt(b));
        Self::new(cap, comparator)
    }

    fn new_max_priority_queue(cap: usize)-> Self{
        let comparator = Box::new(|a: &T, b: &T| a.lt(b));
        Self::new(cap, comparator)
    }

    fn new(cap: usize, comparator: Comparator<T>) -> Self{
        let mut pq = Self {
            pq: Vec::with_capacity(cap + 1),
            n: 0,
            comparator, 
        };
        pq.pq.push(T::default());

        pq
    }

    pub fn is_empty(&self) -> bool{
        self.n == 0
    }

    pub fn len(&self) -> usize {
        self.n
    }
    
    //Depends on heap property, min gets smallest key, max gets largest key
    pub fn peek(&self) -> Option<&T>{
        self.pq.get(1)
    }
}






