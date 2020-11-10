use std::collections::HashMap;

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    //
    // get_mut, insertともに、可変の借用をとる
    // pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    // pub fn insert(&mut self, k: K, v: V) -> Option<V>
    //
    match map.get_mut(&key) {
        // Someの中ではvalue: &mut Vがmapの可変借用を必要とする
        // Someの節を抜けたら、借用は返却されるべきだが、Lexical Scope Lifetimeでチェックされると、
        // matchのブロック内側で、一度borrowingを見てしまっていた。
        // したがって、このコードのように、Noneでinsert（mapの可変借用）をとろうとしても、コンパイルエラーとなってしまっていた。
        // NLL(Non Lexical Lifetime)では、制御フローも含めてLifetimeを見ているため、より直感的になっている。
        Some(value) => value.push_str(", world"),
        None => {
            map.insert(key, Default::default());
        }
    };
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    process_or_default('b', &mut map);
}
