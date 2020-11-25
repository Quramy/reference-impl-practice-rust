use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
static RABBITS: RefCell<HashSet<&'static str>> = {
    let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
    RefCell::new(rb)
}
);

fn main() {
    RABBITS.with(|rb| {
        assert!(rb.try_borrow_mut().is_ok());
        rb.try_borrow_mut()
            .expect("borrowing error")
            .insert("ネザーランド・ドワーフ");
    });
    std::thread::spawn(|| {
        RABBITS.with(|rb| {
            rb.try_borrow_mut()
                .expect("borrowing error")
                .insert("ドワーフホト");
        });
    })
    .join()
    .expect("Thread error");

    RABBITS.with(|rb| {
        assert_eq!(rb.borrow().contains("ネザーランド・ドワーフ"), true);
        assert_eq!(rb.borrow().contains("ドワーフホト"), false);
    });
}
