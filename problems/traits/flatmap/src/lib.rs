#![forbid(unsafe_code)]

use std::{borrow::Borrow, fmt::Debug, iter::FromIterator, ops::Index, vec::IntoIter};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord + Debug, V: Debug> FlatMap<K, V> {
    pub fn new() -> Self {
        FlatMap::<K, V>(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.0.binary_search_by(|x| x.0.cmp(&key)) {
            Ok(i) => {
                let old_val = std::mem::replace(&mut self.0[i], (key, value));
                Some(old_val.1)
            }
            Err(i) => {
                self.0.insert(i, (key, value));
                None
            }
        }
    }

    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        let searched = self.0.binary_search_by(|x| x.0.borrow().cmp(key));
        match searched {
            Ok(i) => Some(&self.0[i].1),
            Err(_) => None,
        }
    }

    pub fn remove<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        let kv = self.remove_entry(key);
        match kv {
            None => None,
            Some((_, v)) => Some(v),
        }
    }

    pub fn remove_entry<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
    {
        let searched = self.0.binary_search_by(|x| x.0.borrow().cmp(key));
        match searched {
            Ok(i) => Some(self.0.remove(i)),
            Err(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K: Debug, V: Debug, Q: Ord + ?Sized> Index<&Q> for FlatMap<K, V>
where
    K: Ord + Borrow<Q>,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        let searched = self.0.binary_search_by(|x| x.0.borrow().cmp(key));
        match searched {
            Ok(i) => &self.0[i].1,
            Err(_) => panic!("no entry found for key"),
        }
    }
}

impl<K: Ord + Debug, V: Debug> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Ord + Debug, V: Debug> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(mut vec: Vec<(K, V)>) -> Self {
        if vec.len() < 2 {
            return FlatMap::<K, V>(vec);
        }
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        let mut ri = 1;
        let mut wi = 0;
        while ri < vec.len() {
            if vec[ri].0 != vec[ri - 1].0 {
                vec.swap(wi, ri - 1);
                wi += 1;
            }
            ri += 1;
        }
        vec.swap(wi, ri - 1);
        vec.truncate(wi + 1);
        FlatMap::<K, V>(vec)
    }
}

impl<K, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(fm: FlatMap<K, V>) -> Self {
        fm.0
    }
}

impl<K: Ord + Debug, V: Debug> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let vec: Vec<(K, V)> = iter.into_iter().collect();
        FlatMap::from(vec)
    }
}

impl<K, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
