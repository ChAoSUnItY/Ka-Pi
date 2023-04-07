use std::collections::HashMap;
use std::hash::Hash;

pub(crate) trait InsertAndRetrieve<K, V> {
    fn insert_retrieve(&mut self, key: K, value: V) -> V;
}

impl<K, V> InsertAndRetrieve<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
    V: Copy,
{
    fn insert_retrieve(&mut self, key: K, value: V) -> V {
        self.insert(key, value);
        value
    }
}
