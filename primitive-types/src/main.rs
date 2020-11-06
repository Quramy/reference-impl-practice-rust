fn hello() {
    println!("hello");
}

fn twice(x: i32) -> i32 {
    x * 2
}

fn exec1<T>(x: i32, f: T) -> i32
where
    T: Fn(i32) -> i32,
{
    f(x)
}

fn exec2(x: i32, f: fn(i32) -> i32) -> i32 {
    f(x)
}

fn main() {
    // unit
    let ret = hello();
    assert_eq!(ret, ());
    let s = std::mem::size_of::<()>();
    assert_eq!(s, 0);

    // bool
    let b1 = true;
    println!("b1: {}", b1);

    // integer
    let n1 = 10_000;
    let n2 = 0u8;
    let n3 = -100_isize;

    let n4 = 10;
    let n5 = n3 + n4;

    println!("n1: {}", n1);
    println!("n2: {}", n2);
    println!("n3: {}", n3);
    println!("n4: {}", n4);
    println!("n5: {}", n5);

    // 型が違うので足し算できない
    // println!("error, :{}", n1 + n2);
    println!("n1 + n2, :{}", n1 + n2 as i32);

    println!("condition, :{}", if b1 { n1 } else { 1000 });

    // character
    let c1 = 'a';
    let c2 = '\t';
    let c3 = '\\';
    println!("c1: {}", c1);
    println!("c2c1: {}{}", c2, c1);
    println!("c3c1: {}{}", c3, c1);

    let c4 = '😎';
    println!("c4: {}", c4);

    // reference
    let n1 = 1000;
    let r1 = &n1;
    println!("addr r1: {:p}", r1);
    println!("value r1: {}", r1);
    println!("deref r1: {}", *r1);

    // unsafe pointer
    let c1 = 'A';
    let c1_ptr: *const char = &c1;

    // 生ポインタのderefはunsafe
    // println!("unsafe deref c1: {}", *c1_ptr);
    println!("unsafe deref c1: {}", unsafe { *c1_ptr });

    // function pointer
    let f1: fn(i32) -> i32 = twice;
    let f2 = twice;
    // どっちでも呼び出しはできる
    println!("call f1: {}", f1(100));
    println!("call f2: {}", f2(100));
    // f1は関数ポインタだけど、f2は関数定義なのでサイズが異なる
    println!("size of f1: {}", std::mem::size_of_val(&f1));
    println!("size of f2: {}", std::mem::size_of_val(&f2));
    println!("addr f1: {:p}", f1);

    println!("pass f1 as callback: {}", exec2(100, f1));
    println!("pass f2 as callback: {}", exec2(100, f2));
    println!("use f1 as closure callback: {}", exec1(100, f1));
    println!("use f2 as closure callback: {}", exec1(100, f2));

    // tuple
    let t1 = (100, 100);
    println!("t1: {:?}", t1);
    println!("1st elm of t1: {}", t1.0);
    println!("2nd elm of t1: {}", t1.1);
    // これはダメ
    // println!("2nd elm of t1: {}", t1.2);

    // array
    let arr1 = [1, 2, 3, 4];
    println!("arr1: {:?}", arr1);
    for x in arr1.iter() {
        println!("\titer of arr1: {}", x);
    }

    // slice
    let s1: &[i32] = &arr1[..];
    println!("s1: {:?}", s1);
    for x in s1.iter() {
        println!("\titer of s1: {}", x);
    }
    let v1 = vec![1, 2, 3, 4];
    let s2: &[i32] = &v1;
    println!("s2: {:?}", s2);
    for x in s2.iter() {
        println!("\titer of s1: {}", x);
    }

    // str
    let str1: &str = "hello";
    println!("str1: {}", str1);
    println!("str1: {}", str1.len());

    let str2 = "Hello

world";
    for line in str2.lines() {
        println!("\tstr2 line: {}", line);
    }
    for c in str2.chars() {
        println!("\tsrt2 character: {}", c);
    }
}
