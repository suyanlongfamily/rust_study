use std::ops::Add;


trait Math<K, V, U> {
    fn kv_to_u(&self, k: K, v: V) -> U;
}

#[derive(Debug)]
struct Entry {
    v: i32,
    k: i32,
}


impl Math<K, V, U> for Entry {
    fn kv_to_u<K, V, U>(&self, k: K, v: V) -> U {}
}

fn main() {
    println!("-----begin---");




}
