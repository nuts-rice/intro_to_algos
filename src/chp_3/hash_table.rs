mod hash_table{
    use std::cmp::PartialEq;
    use std::fmt::Debug;

    pub trait Hashable{
        fn hash(&self) -> usize;
    }

    impl Hashable for String {
        fn hash(&self) -> usize {
            let mut result: usize = 5381;
            for c in self.bytes() {
                result = ((result << 5)).wrapping_add(result)).wrapping_add(c.into());

            }
            result
        }
    }

    impl Hashable for usize {
        fn hash(&self) -> usize {
            *self
        }
    }

    #[derive(Default, Clone)]
    struct HashCell<Key, Value> {
        key: Key,
        value: Value,
        taken: bool,
    }

    pub struct HashTable<Key, Value> {
        cells: Vec<HashCell<Key, Value>>,
        taken_count: usize,
    }

    impl<Key, Value> HashTable<Key, Value>
        where
            Key: Clone + Default + Debug + PartialEq + Hashable,
            Value: Clone + Default + Debug, 
        {
                pub fn new() -> Self {
                    const INITIAL_CAPACITY: usize = 11;
                    Self {
                        cells: vec![HashCell::<_, _>::default(); INITIAL_CAPACITY],
                        taken_count: 0,
                    }
                }
        
        
        }

                
            














