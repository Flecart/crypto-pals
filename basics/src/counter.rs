use std::collections::HashMap;

pub struct Counter<'a, T> {
    counter: HashMap<&'a T, i32>,
    total: i32,
}

impl<'a, T: std::cmp::Eq + std::hash::Hash> Counter<'a, T> {
    pub fn new() -> Counter<'a, T> {
        Counter {
            counter: HashMap::new(),
            total: 0,
        }
    }

    pub fn from(input: &'a Vec<T>) -> Counter<'a, T> {
        let mut table = HashMap::new();
        let mut total: i32 = 0;
        for ch in input {
            let curr_entry = table.entry(ch).or_insert(0);
            *curr_entry += 1;
            total += 1;
        }
        
        Counter {
            counter: table,
            total: total,
        }
    }

    pub fn get(&self, key: &T) -> Result<i32, &'static str> {
        match self.counter.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err("This key does not exist")
        }
    }
}

// the trait `Eq` is not implemented for `T`
// I know compiler >:()