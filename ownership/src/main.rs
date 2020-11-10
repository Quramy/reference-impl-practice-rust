use std::ops::Drop;

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

#[derive(Debug, Copy, Clone)]
struct ParentCopy(usize, ChildCopy, ChildCopy);

#[derive(Debug, Copy, Clone)]
struct ChildCopy(usize);

fn value_scope_example() {
    println!("");
    println!("*** value_scope_example ***");
    let p1 = Parent(1, Child(11), Child(12));
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);

        // dropは自分で呼んじゃ駄目
        // p2.drop();
    }
    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);
}

fn move_semantics_example() {
    println!("");
    println!("*** move_semantics_example ***");
    let mut p1 = Parent(1, Child(11), Child(12));

    let p2 = p1; // ここでp1からp2に値の所有権がmoveしている
    println!("p2: {:?}", p2);

    // p1の所有権が移動後なので駄目
    // println!("p1: {:?}", p1);

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1); // p1が別の値の所有権を得たので、コンパイルできる
}

fn func1(p: Parent) {
    println!("p in func1: {:?}", p);
}

fn func2(p: &Parent) {
    println!("p in func2: {:?}", p);
}

fn func3(p: &mut Parent) {
    p.0 += 1;
}

fn move_semantics_example_fn() {
    println!("");
    println!("*** move_semantics_example_fn ***");
    let p1 = Parent(1, Child(11), Child(12));
    func1(p1);

    // 関数に渡した時点でp1の所有権がmoveしてしまうので駄目。
    // println!("p1: {:?}", p1);

    let mut p2 = Parent(2, Child(21), Child(22));

    // 不変参照をとる関数にわたすだけであれば、所有権の移動ではなく、借用が発生するだけ。
    func2(&p2);
    println!("p2: {:?}", p2); // ので、この行のコンパイルが通る

    // 可変参照を渡しているが、所有権が移ったわけではない
    func3(&mut p2);
    println!("p2: {:?}", p2); // ので、この行のコンパイルが通る
}

fn copy_semantics_example() {
    println!("");
    println!("*** copy_semantics_example ***");
    let mut p1 = ParentCopy(1, ChildCopy(11), ChildCopy(12));
    let p2 = p1;
    println!("p2: {:?}", p2);

    // p1の所有権が移動後なので駄目
    println!("p1: {:?}", p1);

    p1.0 = 100;
    println!("p2: {:?}", p2); // p2の値はcloneで作られてるので、p2.1の値は変わらない
    println!("p1: {:?}", p1);
}

fn main() {
    value_scope_example();
    move_semantics_example();
    copy_semantics_example();
    move_semantics_example_fn();
}
