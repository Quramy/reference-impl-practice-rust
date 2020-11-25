use std::cell::RefCell;

struct A {
    c: char,
    s: String,
}

#[derive(Debug)]
struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    // let a = A { c: 'c', s: "Hoge".to_string() };
    // let r = &a;
    // r.s.push('f'); // 当たり前だけどエラー。

    let b = B {
        c: 'c',
        s: RefCell::new("Hoge".to_string()),
    };

    let r = &b;
    r.s.borrow_mut().push_str(" fuga");

    println!("{:?}", r);

    {
        let x = r.s.borrow();

        // r.s.borrow_mut().push_str(" fuga"); panicする
        
        // try_borrow_mut は Resultを返す
        assert!(r.s.try_borrow_mut().is_err());
        println!("{}", x);
    }
}
