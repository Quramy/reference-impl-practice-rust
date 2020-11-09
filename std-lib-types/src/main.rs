use std::collections::HashMap;

fn box_example() {
    let t1: (i32, String) = (3, "birds".to_string());
    let mut b1 = Box::new(t1);
    // タプルの所有権がb1に移動しているので、コンパイルできない
    // println!("{:?}", t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));
}

fn vec_example() {
    let v1: Vec<bool> = vec![false, true, false];
    let v2: Vec<f64> = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v2.len(), 4);

    let v3 = vec![0; 100];
    let v4: Vec<Vec<char>> = vec![vec!['a', 'b', 'c'], vec!['d']];

    // 型の違う要素は無理
    // let v5 = vec![false, 'a'];

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);

    assert_eq!(v6.pop(), Some('e'));

    v6.insert(1, 'f');
    assert_eq!(v6.remove(2), 'b');
    assert_eq!(v6, ['a', 'f', 'c', 'd']);
}

fn hashmap_example() {
    let mut m1: HashMap<&str, i32> = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);

    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);

    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));
}

fn string_fmt(input: &str) -> String {
    format!("{:.2}", input)
}

fn string_example() {
    let mut s1: String = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    let s3 = "ストロベリー".to_owned();

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と');

    // これはエラー(引数は &str じゃないと駄目)
    // s2.push_str(s3);
    s2.push_str(&s3); // &つけると型強制されて、&Stringではなく、&strになる

    assert_eq!(s2, "ブラックベリーとストロベリー");

    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    println!("{}", f.to_string());
    assert_eq!(format!("{:.2}", f), "4.40");

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    println!("{}", string_fmt("1000.000"));

    let s1 = String::from("1000.000");
    let ss1 = string_fmt(&s1);
    println!("{}", ss1);
}

fn range_example() {
    let a = "abcdef".chars().collect::<Vec<char>>();
    println!("[..]: {:?}", &a[..]);
    println!("[0..]: {:?}", &a[0..]);
    println!("[..6]: {:?}", &a[..6]);
    println!("[..=5]: {:?}", &a[..=5]);
    let r: std::ops::Range<usize> = 0..a.len();
    println!("[r]: {:?}", &a[r]);
}

fn main() {
    box_example();
    vec_example();
    hashmap_example();
    string_example();
    range_example();
}
