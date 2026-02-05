pub trait Mapper<K, V> {
    fn get(&self, key: K) -> Option<&V>;
}

pub trait MutMapper<K, V> {
    fn get(&mut self, key: K) -> Option<&mut V>;
}