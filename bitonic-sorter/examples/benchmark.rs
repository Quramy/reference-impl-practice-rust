use num_cpus;

use bitonic_sorter::fourth::sort as pr_sort;
use bitonic_sorter::third::sort as seq_sort;
use bitonic_sorter::utils::{is_sorted_ascending, new_u32_vec};
use bitonic_sorter::SortOrder;

use std::str::FromStr;
use std::time::Instant;
use std::{env, f64};

fn main() {
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sort(bits);
    } else {
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}

fn run_sort(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;

    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );

    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let seq_duration = timed_sort(&seq_sort, len, "seq_sort");

    let pr_duration = timed_sort(&pr_sort, len, "pr_sort");

    println!("speed up: {:.2}x", seq_duration / pr_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    let mut x = new_u32_vec(len);
    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Failed to sort");
    let dur = start.elapsed();

    let nano_sec = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!(
        "{}: sorted {} integers in {} seconds",
        name,
        len,
        nano_sec / 1e9
    );
    assert!(is_sorted_ascending(&x));
    nano_sec
}
