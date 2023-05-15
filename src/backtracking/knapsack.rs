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
    fn fill(&self, items: Vec<&Item>) -> u64 {
        let value = self.fill_r(&items, 0);
        if value < 0 {
            0
        } else {
            value as u64
        }
    }

    fn fill_r(&self, remaining: &[&Item], current_weight: usize) -> i64 {
        let w = current_weight;
        if w > self.capacity {
            return i64::min_value();
        }
        if remaining.len() > 0 && w < self.capacity {
            let include = remaining[0].value as i64
                + self.fill_r(
                    &remaining[1..],
                    current_weight + remaining[0].weight as usize,
                );
            let exclude = self.fill_r(&remaining[1..], current_weight);
            if include >= exclude {
                return include;
            } else {
                return exclude;
            }
        } else {
            0
        }
    }
}
