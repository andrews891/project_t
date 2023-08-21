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
        let b_return = self.map_a.remove(a);
        let a_return = self.map_b.remove(b);
        match a_return {
            Some(a) => {
                self.map_a.remove(&a);
                return ()
            },
            None => (),
        };
        match b_return {
            Some(b) => {
                self.map_b.remove(&b);
                return ()
            },
            None => (),
        };
    }
    
    pub fn get(&self, a: &A, b: &B) -> (Option<&B>, Option<&A>) {
        (self.map_a.get(a), self.map_b.get(b))
    }
}

#[test]
fn test_insertion() {
    let mut bhm = BiHashMap::<&str, &str>::new();
    bhm.insert(&"foo", &"bar");
    let mut hma = HashMap::new();
    let mut hmb = HashMap::new();
    hma.insert("foo", "bar");
    hmb.insert("bar", "foo");
    assert!(bhm.map_a == hma);
    assert!(bhm.map_b == hmb);
}

#[test]
fn test_a_deletion() {
    let mut bhm = BiHashMap::<&str, &str>::new();

    bhm.insert(&"foo", &"bar");
    bhm.remove(&"foo", &"");
    assert!(bhm.map_a == HashMap::new() && bhm.map_b == HashMap::new());
}

#[test]
fn test_b_deletion() {
    let mut bhm = BiHashMap::<&str, &str>::new();

    bhm.insert(&"foo", &"bar");
    bhm.remove(&"", &"bar");
    assert!(bhm.map_a == HashMap::new() && bhm.map_b == HashMap::new());
}