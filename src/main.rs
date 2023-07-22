use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

const DEFAULT_SIZE: usize = 10;

struct SwissTable<K, V> {
    size: usize,
    table: Vec<Option<(K, V)>>,
}

impl<K, V> SwissTable<K, V>
    where
        K: Eq + std::hash::Hash + std::clone::Clone,
        V: Clone,
{
    fn new(size: usize) -> Self {
        SwissTable {
            size,
            table: vec![None; size],
        }
    }

    fn hash_function(&self, key: &K, attempt: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash::<std::collections::hash_map::DefaultHasher>(&mut hasher);
        let result = hasher.finish() as usize;
        (result + attempt.pow(2)) % self.size
    }

    fn find_slot(&self, key: &K) -> Option<usize> {
        let mut attempt = 0;
        let mut index = self.hash_function(key, attempt);
        while let Some((k, _)) = &self.table[index] {
            if k == key {
                return Some(index);
            }
            attempt += 1;
            index = self.hash_function(key, attempt);
        }
        None
    }

    fn insert(&mut self, key: K, value: V) {
        if self.find_slot(&key).is_none() {
            let mut attempt = 0;
            let mut index = self.hash_function(&key, attempt);
            while let Some((_, _)) = &self.table[index] {
                attempt += 1;
                index = self.hash_function(&key, attempt);
            }
            self.table[index] = Some((key, value));
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        if let Some(index) = self.find_slot(key) {
            return self.table[index].as_ref().map(|(_, v)| v);
        }
        None
    }
}

fn main() {
    let mut table: SwissTable<String, String> = SwissTable::new(DEFAULT_SIZE);
    for i in 0..DEFAULT_SIZE {
        let k = format!("k{}", i);
        let v = format!("v{}", i);
        table.insert(k, v);
    }
    println!("{:?}", table.get(&"k0".to_string()));
    println!("{:?}", table.get(&"not found".to_string()));
}
