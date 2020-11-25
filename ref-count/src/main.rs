use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self);
    }
}

fn rc_example() {
    let mut rc1 = Rc::new(Child(1));
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2
        );
    }
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    {
        if let Some(child) = Rc::get_mut(&mut rc1) {
            // 参照カウントが1のときだけ可変借用できる
            child.0 += 1;
        }
        println!("(d-1) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    }

    {
        let rc2 = Rc::clone(&rc1);
        if let Some(child) = Rc::get_mut(&mut rc1) {
            child.0 += 1;
        } else {
            // 参照カウントが2なので、Noneが帰る
            println!("(d-2) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
        }
        println!(
            "(d-3) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2
        );
    }

    let weak: Weak<Child> = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1),
        rc1,
        weak
    );

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3
        );
    }

    std::mem::drop(rc1);
    match weak.upgrade() {
        None => {
            println!("(g) count: 0, weak: {:?}", weak);
        },
        _ => {},
    }
}

fn main() {
    rc_example();
}
