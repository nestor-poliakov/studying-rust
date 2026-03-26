#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    n: usize,
    cap: usize,
    hm: HashMap<K, (V, usize)>,
    btm: BTreeMap<usize, K>,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        if capacity < 1 {
            panic!("capacity below 1")
        }
        Self {
            n: 0,
            cap: capacity,
            hm: HashMap::new(),
            btm: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.n += 1;

        let v = self.hm.get_mut(key);
        v.as_ref()?;
        let v = v.unwrap();

        self.btm.remove(&v.1);
        self.btm.insert(self.n, key.clone());
        v.1 = self.n;
        Some(&v.0)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.n += 1;

        let v = self.hm.remove(&key);
        let mut result = None;
        if let Some(v) = v {
            result = Some(v.0);
            self.btm.remove(&v.1);
        }
        self.hm.insert(key.clone(), (value, self.n));
        self.btm.insert(self.n, key);

        if self.btm.len() > self.cap {
            let (_, key_last) = self.btm.pop_first().unwrap();
            self.hm.remove(&key_last);
        }
        result
    }
}
