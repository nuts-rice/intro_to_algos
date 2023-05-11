#[derive(Debug, PartialEq)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub struct Knapsack {
    capacity: usize,
}

impl Knapsack {
    pub fn new(capacity: usize) -> Knapsack {
        Knapsack { capacity: capacity }
    }
}

pub trait Backtracking {
    fn fill(&self, items: Vec<&Item>) -> u64;
    fn fill_r(&self, remaining: &[&Item], current_weight: usize) -> i64;
}

impl Backtracking for Knapsack {
    fn fill(&self, items: Vec<&Item>) -> u64 {}

    fn fill_r(&self, remaining: &[&Item], current_weight: usize) -> i64 {
        let w = current_weight;
    }
}
