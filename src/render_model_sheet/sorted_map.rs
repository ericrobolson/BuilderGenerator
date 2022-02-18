use std::fmt::Debug;

pub struct SortedMap<K, Value>
where
    K: PartialEq,
{
    items: Vec<(K, Value)>,
}

impl<K, V> SortedMap<K, V>
where
    K: PartialEq + Debug,
    V: Debug,
{
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn delete(&mut self, k: &K) {
        for idx in 0..self.items.len() {
            if self.items[idx].0 == *k {
                self.items.remove(0);
                return;
            }
        }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        for idx in 0..self.items.len() {
            let matched = self.items[idx].0 == *k;
            if matched {
                return Some(&mut self.items[idx].1);
            }
        }

        return None;
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.delete(&k);

        self.items.push((k, v));
    }

    pub fn iter(&self) -> &[(K, V)] {
        &self.items
    }

    pub fn iter_mut(&mut self) -> &mut [(K, V)] {
        &mut self.items
    }
}
