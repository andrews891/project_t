use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct BiHashMap<A, B> where A: Eq + Hash + Copy, B: Eq + Hash + Copy {
    map_a: HashMap<A, B>,
    map_b: HashMap<B, A>,
}

impl <A, B> BiHashMap<A, B> where A: Eq + Hash + Copy, B: Eq + Hash + Copy {
    pub fn new() -> Self {
        Self {
            map_a: HashMap::<A, B>::new(),
            map_b: HashMap::<B, A>::new()
        }
    }

    pub fn insert(&mut self, a: A, b: B) {
        self.map_a.insert(a, b);
        self.map_b.insert(b, a);
    }

    pub fn remove(&mut self, a: &A, b: &B) {
        self.map_a.remove(a);
        self.map_b.remove(b);
    }
    
    pub fn get(&self, a: &A, b: &B) -> (Option<&B>, Option<&A>) {
        (self.map_a.get(a), self.map_b.get(b))
    }
}